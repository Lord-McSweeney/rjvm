use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::interpreter::Interpreter;
use super::loader::ClassLoader;
use super::object::Object;
use super::op::Op;
use super::value::Value;
use super::verify::verify_ops;

use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::flags::MethodFlags;
use crate::classfile::method::Method as ClassFileMethod;
use crate::gc::{Gc, Trace};
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::{OnceCell, RefCell};
use core::fmt;
use core::hash::{Hash, Hasher};
use hashbrown::HashMap;

#[derive(Clone, Copy)]
pub struct Method(Gc<MethodData>);

impl Trace for Method {
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Method")
            .field("name", &self.name())
            .finish()
    }
}

struct MethodData {
    descriptor: MethodDescriptor,

    physical_arg_count: usize,

    flags: MethodFlags,

    name: JvmString,

    class: Class,

    // The `java.lang.reflect.Executable` object for this `Method`, lazily
    // initialized
    object: OnceCell<Object>,

    method_info: RefCell<MethodInfo>,
}

impl Method {
    pub fn from_method(
        context: &Context,
        method: &ClassFileMethod,
        class: Class,
        has_receiver: bool,
    ) -> Result<Self, Error> {
        let descriptor = MethodDescriptor::from_string(context, method.descriptor())?;

        let attributes = method.attributes();
        let mut raw_code_data = None;
        for attribute in attributes {
            if attribute.name().as_bytes() == b"Code" {
                // This performs a clone, but this code shouldn't be perf-sensitive
                raw_code_data = Some(attribute.data().to_vec());
            }
        }

        let method_info = if let Some(raw_code_data) = raw_code_data {
            MethodInfo::BytecodeUnparsed(raw_code_data)
        } else if method.flags().contains(MethodFlags::NATIVE) {
            let native_method = context.get_native_method(class.name(), method.name(), descriptor);

            if let Some(native_method) = native_method {
                MethodInfo::Native(native_method)
            } else {
                // We don't want to panic right away, as the method might not
                // end up getting called at all
                MethodInfo::NativeNotFound
            }
        } else {
            MethodInfo::Empty
        };

        let mut physical_arg_count = descriptor.physical_arg_count();
        if has_receiver {
            // +1 for the receiver arg
            physical_arg_count += 1;
        }

        Ok(Self(Gc::new(
            context.gc_ctx,
            MethodData {
                descriptor,
                physical_arg_count,
                flags: method.flags(),
                name: method.name(),
                class,
                object: OnceCell::new(),
                method_info: RefCell::new(method_info),
            },
        )))
    }

    /// Internal method for executing a method- `pub(crate)` so it's not
    /// accidentally called by user code.
    ///
    /// NOTE: This method reads arguments from the stack, and pops them after
    /// execution!
    pub(crate) fn exec(self, context: &Context) -> Result<Option<Value>, Error> {
        self.class().run_clinit(context)?;

        // Run everything in a closure so we can handle the call stack more easily
        let closure = || -> Result<Option<Value>, Error> {
            // Parse bytecode if it hasn't been already
            if matches!(
                &*self.0.method_info.borrow(),
                MethodInfo::BytecodeUnparsed(_)
            ) {
                self.parse_info(context)?;
            }

            // All checks are performed in the verifier

            match &*self.0.method_info.borrow() {
                MethodInfo::Bytecode(bytecode_info) => {
                    let mut interpreter = Interpreter::new(context, self)?;

                    interpreter.interpret_ops(&bytecode_info.code, &bytecode_info.exceptions)
                }
                MethodInfo::BytecodeUnparsed(_) => unreachable!(),
                MethodInfo::Native(native_method) => {
                    let physical_arg_count = self.physical_arg_count();
                    let current_position = context.frame_index.get();
                    let slice = &context.frame_data
                        [(current_position - physical_arg_count)..current_position];

                    let args = slice.iter().map(|a| a.get()).collect::<Vec<_>>();

                    native_method(context, &args)
                }
                MethodInfo::NativeNotFound => {
                    panic!(
                        "associated native method for {}.{} not found",
                        self.class().name(),
                        self.name()
                    );
                }
                MethodInfo::Empty => panic!("cannot call method without body"),
            }
        };

        let initial_frame_index = context.frame_index.get();

        context.push_call(self);
        let result = closure();
        context.pop_call();

        assert!(initial_frame_index == context.frame_index.get());

        // Pop args
        context
            .frame_index
            .set(context.frame_index.get() - self.physical_arg_count());

        result
    }

    fn parse_info(self, context: &Context) -> Result<(), Error> {
        let borrow = self.0.method_info.borrow();

        let new_method_info = match &*borrow {
            MethodInfo::BytecodeUnparsed(code_data) => {
                let bytecode_method_info =
                    BytecodeMethodInfo::from_code_data(context, self, &*code_data)?;

                Some(MethodInfo::Bytecode(bytecode_method_info))
            }
            // None of the other method info types need (re-)parsing
            _ => None,
        };

        drop(borrow);

        if let Some(new_method_info) = new_method_info {
            *self.0.method_info.borrow_mut() = new_method_info;
        }

        Ok(())
    }

    // Return an instance of `java.lang.reflect.Executable` for this `Method`.
    pub fn get_or_init_object(self, context: &Context) -> Object {
        *self.0.object.get_or_init(|| {
            let id = context.add_executable_object(self);

            let object = if self.0.name.as_bytes() == b"<init>" {
                Object::constructor_object(context)
            } else {
                Object::method_object(context)
            };

            object.set_field(0, Value::Integer(id));

            object
        })
    }

    pub fn descriptor(self) -> MethodDescriptor {
        self.0.descriptor
    }

    /// The number of arguments the method was declared to have in Java.
    pub fn arg_count(self) -> usize {
        self.descriptor().args().len()
    }

    /// The "physical" argument count of this method. This counts two arguments
    /// for doubles and longs, and includes the receiver if this method takes
    /// one.
    pub fn physical_arg_count(self) -> usize {
        self.0.physical_arg_count
    }

    pub fn flags(self) -> MethodFlags {
        self.0.flags
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn class(self) -> Class {
        self.0.class
    }

    pub fn class_loader(self) -> ClassLoader {
        self.0.class.loader().expect("Should have a loader")
    }

    pub fn max_stack(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_stack as usize,
            _ => panic!("Should not call max_stack for non-Bytecode method info"),
        }
    }

    pub fn max_locals(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_locals as usize,
            _ => panic!("Should not call max_locals for non-Bytecode method info"),
        }
    }
}

impl Trace for MethodData {
    #[inline(always)]
    fn trace(&self) {
        self.descriptor.trace();
        self.name.trace();
        self.class.trace();
        self.object.trace();
        self.method_info.trace();
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        Gc::as_ptr(self.0) == Gc::as_ptr(other.0)
    }
}

impl Eq for Method {}

impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Gc::as_ptr(self.0).hash(state);
    }
}

enum MethodInfo {
    Bytecode(BytecodeMethodInfo),
    BytecodeUnparsed(Vec<u8>),
    Native(NativeMethod),
    NativeNotFound,
    Empty,
}

impl Trace for MethodInfo {
    #[inline(always)]
    fn trace(&self) {
        match self {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.trace(),
            MethodInfo::Native(native_method) => native_method.trace(),
            _ => {}
        }
    }
}

struct BytecodeMethodInfo {
    max_stack: u16,
    max_locals: u16,
    code: Box<[Op]>,
    exceptions: Box<[Exception]>,
}

pub struct Exception {
    // Inclusive
    pub start: usize,

    // Exclusive
    pub end: usize,

    pub target: usize,

    pub catch_class: Option<Class>,
}

impl Trace for Exception {
    #[inline(always)]
    fn trace(&self) {
        self.catch_class.trace();
    }
}

impl BytecodeMethodInfo {
    pub fn from_code_data(context: &Context, method: Method, data: &[u8]) -> Result<Self, Error> {
        let mut reader = FileData::new(data);

        let class_file = method.class().class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let max_stack = reader.read_u16_be()?;
        let max_locals = reader.read_u16_be()?;

        let (code, offset_to_idx_map) = Op::read_ops(context, method, constant_pool, &mut reader)?;

        let exceptions = BytecodeMethodInfo::read_exceptions(
            context,
            method,
            constant_pool,
            &mut reader,
            offset_to_idx_map,
        )?;

        match verify_ops(
            method,
            max_stack as usize,
            max_locals as usize,
            &code,
            &exceptions,
        ) {
            Ok(()) => {}
            Err(verify_error) => return Err(verify_error.to_error(context)),
        }

        Ok(Self {
            max_stack,
            max_locals,
            code,
            exceptions,
        })
    }

    fn read_exceptions(
        context: &Context,
        method: Method,
        constant_pool: &ConstantPool,
        reader: &mut FileData<'_>,
        offset_to_idx_map: HashMap<usize, usize>,
    ) -> Result<Box<[Exception]>, Error> {
        let exception_count = reader.read_u16_be()?;
        let mut exceptions = Vec::with_capacity(exception_count as usize);
        for _ in 0..exception_count {
            let start_offset = reader.read_u16_be()? as usize;
            let end_offset = reader.read_u16_be()? as usize;
            let target_offset = reader.read_u16_be()? as usize;

            let start = offset_to_idx_map
                .get(&start_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;
            let end = offset_to_idx_map
                .get(&end_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;
            let target = offset_to_idx_map
                .get(&target_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;

            let class_idx = reader.read_u16_be()?;
            let class = if class_idx != 0 {
                let class_name = constant_pool.get_class(class_idx)?;
                let class = method.class_loader().lookup_class(context, class_name)?;

                let throwable_class = context.builtins().java_lang_throwable;
                if !class.matches_class(throwable_class) {
                    return Err(Error::Native(NativeError::ErrorClassNotThrowable));
                }

                Some(class)
            } else {
                None
            };

            exceptions.push(Exception {
                start,
                end,
                target,
                catch_class: class,
            });
        }

        Ok(exceptions.into_boxed_slice())
    }
}

impl Trace for BytecodeMethodInfo {
    #[inline(always)]
    fn trace(&self) {
        self.code.trace();
        self.exceptions.trace();
    }
}

pub type NativeMethod = for<'a> fn(&Context, &[Value]) -> Result<Option<Value>, Error>;

impl Trace for NativeMethod {
    fn trace(&self) {}
}
