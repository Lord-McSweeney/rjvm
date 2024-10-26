use super::class::Class;
use super::context::Context;
use super::error::{Error, NativeError};
use super::method::Method;
use super::object::Object;
use super::op::Op;
use super::value::Value;

use crate::classfile::constant_pool::{ConstantPool, ConstantPoolEntry};

pub struct Interpreter {
    method: Method,
    stack: Vec<Value>,
    local_registers: Vec<Value>,

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
            context,
        }
    }

    pub fn interpret_ops(&mut self, ops: &[Op]) -> Result<Option<Value>, Error> {
        let mut ip = 0;
        while ip < ops.len() {
            let op = ops[ip];
            match op {
                Op::ALoad(index) => self.a_load(index),
                Op::Ldc(constant_pool_entry) => self.ldc(constant_pool_entry),
                Op::Return => return Ok(None),
                Op::GetStatic(class, static_field_idx) => self.get_static(class, static_field_idx),
                Op::PutField(class, field_idx) => self.put_field(class, field_idx)?,
                Op::InvokeVirtual(class, (method_name, method_descriptor)) => todo!(),
                Op::InvokeSpecial(class, method) => self.invoke_special(class, method)?,
                other => unimplemented!("Tried to execute unimplemented op"),
            }

            ip += 1;
        }

        panic!("Execution should never fall off function")
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn a_load(&mut self, index: usize) {
        let loaded = self.local_registers[index];

        if !matches!(loaded, Value::Object(_)) {
            panic!("Local should be of reference type");
        }

        self.stack_push(loaded);
    }

    fn ldc(&mut self, constant_pool_entry: ConstantPoolEntry) {
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

    fn get_static(&mut self, class: Class, static_field_idx: usize) {
        let static_field = class.static_fields()[static_field_idx];

        self.stack_push(static_field.value());
    }

    fn put_field(&mut self, class: Class, field_idx: usize) -> Result<(), Error> {
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

    fn invoke_special(&mut self, class: Class, method: Method) -> Result<(), Error> {
        let mut args = vec![Value::Object(None); method.arg_count() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop();
        if !receiver.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        args[0] = receiver;

        let result = method.exec(self.context, &args)?;
        if let Some(result) = result {
            self.stack_push(result);
        }

        Ok(())
    }
}
