use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::method::Method;
use super::object::Object;
use super::op::Op;
use super::value::Value;

use crate::classfile::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::string::JvmString;

pub struct Interpreter {
    method: Method,
    stack: Vec<Value>,
    local_registers: Vec<Value>,

    ip: usize,

    context: Context,
}

impl Interpreter {
    pub fn new(context: Context, method: Method, args: Vec<Value>) -> Self {
        let stack = Vec::with_capacity(method.max_stack());
        let mut local_registers = vec![Value::Object(None); method.max_locals()];
        for (i, arg) in args.iter().enumerate() {
            local_registers[i] = *arg;
        }

        Self {
            method,
            stack,
            local_registers,

            ip: 0,

            context,
        }
    }

    pub fn interpret_ops(&mut self, ops: &[Op]) -> Result<Option<Value>, Error> {
        while self.ip < ops.len() {
            let op = ops[self.ip];
            match op {
                Op::AConstNull => self.op_a_const_null(),
                Op::IConst(val) => self.op_i_const(val),
                Op::Ldc(constant_pool_entry) => self.op_ldc(constant_pool_entry),
                Op::ILoad(_) => todo!(),
                Op::ALoad(index) => self.op_a_load(index),
                Op::BaLoad => todo!(),
                Op::IStore(_) => todo!(),
                Op::AStore(_) => todo!(),
                Op::Dup => self.op_dup(),
                Op::IAdd => todo!(),
                Op::IInc(_, _) => todo!(),
                Op::IfLt(_) => todo!(),
                Op::IfGe(_) => todo!(),
                Op::IfICmpGe(_) => todo!(),
                Op::IfICmpGt(_) => todo!(),
                Op::Goto(_) => todo!(),
                Op::Return => return Ok(None),
                Op::GetStatic(class, static_field_idx) => {
                    self.op_get_static(class, static_field_idx)
                }
                Op::PutStatic(class, static_field_idx) => {
                    self.op_put_static(class, static_field_idx)
                }
                Op::GetField(class, field_idx) => self.op_get_field(class, field_idx)?,
                Op::PutField(class, field_idx) => self.op_put_field(class, field_idx)?,
                Op::InvokeVirtual((method_name, method_descriptor)) => {
                    self.op_invoke_virtual(method_name, method_descriptor)?
                }
                Op::InvokeSpecial(class, method) => self.op_invoke_special(class, method)?,
                Op::InvokeStatic(method) => self.op_invoke_static(method)?,
                Op::New(class) => self.op_new(class),
                Op::ArrayLength => todo!(),
                Op::AThrow => todo!(),
                Op::IfNonNull(position) => {
                    self.op_if_non_null(position);
                    continue;
                }
            }

            self.ip += 1;
        }

        panic!("Execution should never fall off function")
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn op_a_const_null(&mut self) {
        self.stack_push(Value::Object(None));
    }

    fn op_i_const(&mut self, value: i32) {
        self.stack_push(Value::Integer(value));
    }

    fn op_ldc(&mut self, constant_pool_entry: ConstantPoolEntry) {
        let class_file = self.method.class().unwrap().class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let pushed_value = match constant_pool_entry {
            ConstantPoolEntry::String { string_idx } => {
                let string = constant_pool
                    .get_utf8(string_idx)
                    .expect("Should refer to valid entry");
                let string_chars = string.encode_utf16().collect::<Vec<_>>();
                let chars_array_object = Object::char_array(self.context, &string_chars);

                let string_class = self
                    .context
                    .lookup_class(self.context.common.java_lang_string)
                    .expect("String class should exist");

                let string_instance = string_class.new_instance(self.context.gc_ctx);
                string_instance
                    .call_construct(
                        self.context,
                        self.context.common.arg_char_array_void_desc,
                        &[
                            Value::Object(Some(string_instance)),
                            Value::Object(Some(chars_array_object)),
                        ],
                    )
                    .expect("String class should construct");

                Value::Object(Some(string_instance))
            }
            _ => unimplemented!(),
        };

        self.stack_push(pushed_value);
    }

    fn op_a_load(&mut self, index: usize) {
        let loaded = self.local_registers[index];

        if !matches!(loaded, Value::Object(_)) {
            panic!("Local should be of reference type");
        }

        self.stack_push(loaded);
    }

    fn op_dup(&mut self) {
        let value = self.stack_pop();
        self.stack_push(value);
        self.stack_push(value);
    }

    fn op_get_static(&mut self, class: Class, static_field_idx: usize) {
        let static_field = class.static_fields()[static_field_idx];

        self.stack_push(static_field.value());
    }

    fn op_put_static(&mut self, class: Class, static_field_idx: usize) {
        let value = self.stack_pop();

        let static_field = class.static_fields()[static_field_idx];

        static_field.set_value(value);
    }

    fn op_get_field(&mut self, class: Class, field_idx: usize) -> Result<(), Error> {
        let object = self.stack_pop();
        if !object.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        let object = object.expect_as_object();
        if let Some(object) = object {
            self.stack_push(object.get_field(field_idx));

            Ok(())
        } else {
            Err(Error::Native(NativeError::NullPointerException))
        }
    }

    fn op_put_field(&mut self, class: Class, field_idx: usize) -> Result<(), Error> {
        let value = self.stack_pop();

        let object = self.stack_pop();
        if !object.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        let object = object.expect_as_object();
        if let Some(object) = object {
            object.set_field(field_idx, value);

            Ok(())
        } else {
            Err(Error::Native(NativeError::NullPointerException))
        }
    }

    fn op_invoke_virtual(
        &mut self,
        method_name: JvmString,
        method_descriptor: MethodDescriptor,
    ) -> Result<(), Error> {
        let mut args = vec![Value::Object(None); method_descriptor.args().len() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            // TODO: Long and Double arguments require two pops
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop().expect_as_object();

        if let Some(receiver) = receiver {
            let receiver_class = receiver.class();
            let method_idx = receiver_class
                .instance_method_vtable()
                .lookup((method_name, method_descriptor))
                .ok_or(Error::Native(NativeError::VTableLookupFailed))?;
            let method = receiver_class.instance_methods()[method_idx];

            args[0] = Value::Object(Some(receiver));

            let result = method.exec(self.context, &args)?;
            if let Some(result) = result {
                self.stack_push(result);
            }
            Ok(())
        } else {
            Err(Error::Native(NativeError::NullPointerException))
        }
    }

    fn op_invoke_special(&mut self, class: Class, method: Method) -> Result<(), Error> {
        let mut args = vec![Value::Object(None); method.arg_count() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            // TODO: Long and Double arguments require two pops
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop();
        if !receiver.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        if matches!(receiver, Value::Object(None)) {
            return Err(Error::Native(NativeError::NullPointerException));
        }

        args[0] = receiver;

        let result = method.exec(self.context, &args)?;
        if let Some(result) = result {
            self.stack_push(result);
        }

        Ok(())
    }

    fn op_invoke_static(&mut self, method: Method) -> Result<(), Error> {
        let mut args = vec![Value::Object(None); method.arg_count()];
        for arg in args.iter_mut().rev() {
            // TODO: Long and Double arguments require two pops
            *arg = self.stack_pop();
        }

        let result = method.exec(self.context, &args)?;
        if let Some(result) = result {
            self.stack_push(result);
        }

        Ok(())
    }

    fn op_new(&mut self, class: Class) {
        let instance = class.new_instance(self.context.gc_ctx);

        self.stack_push(Value::Object(Some(instance)));
    }

    fn op_if_non_null(&mut self, position: usize) {
        let value = self.stack_pop();
        let Value::Object(obj) = value else {
            panic!("Stack value should be of reference type");
        };

        if obj.is_some() {
            self.ip = position;
        } else {
            self.ip += 1;
        }
    }
}
