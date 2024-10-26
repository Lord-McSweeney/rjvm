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
}

impl Interpreter {
    pub fn new(method: Method, args: Vec<Value>) -> Self {
        let stack = Vec::with_capacity(method.max_stack());
        let mut local_registers = vec![Value::Object(None); method.max_locals()];
        for (i, arg) in args.iter().enumerate() {
            local_registers[i] = *arg;
        }

        Self {
            method,
            stack,
            local_registers,
        }
    }

    pub fn interpret_ops(&mut self, context: Context, ops: &[Op]) -> Result<Option<Value>, Error> {
        let mut ip = 0;
        while ip < ops.len() {
            let op = ops[ip];
            match op {
                Op::ALoad(index) => {
                    let loaded = self.local_registers[index];

                    if !matches!(loaded, Value::Object(_)) {
                        return Err(Error::Native(NativeError::WrongValueType));
                    }

                    self.stack.push(loaded);
                }
                Op::Ldc(constant_pool_entry) => {
                    let class_file = self.method.class().unwrap().class_file().unwrap();
                    let constant_pool = class_file.constant_pool();

                    let pushed_value = match constant_pool_entry {
                        ConstantPoolEntry::String { string_idx } => {
                            let string = constant_pool.get_utf8(string_idx)?;
                            let string_chars = string.encode_utf16().collect::<Vec<_>>();
                            let chars_array_object = Object::char_array(context, &string_chars);

                            let string_class = context
                                .lookup_class(context.common.java_lang_string)
                                .expect("String class should exist");

                            let string_instance = string_class.new_instance(context.gc_ctx);
                            string_instance.call_construct(
                                context,
                                context.common.arg_char_array_void_desc,
                                &[Value::Object(Some(chars_array_object))],
                            )?;

                            Value::Object(Some(string_instance))
                        }
                        _ => unimplemented!(),
                    };

                    self.stack.push(pushed_value);
                }
                Op::Return => {
                    return Ok(None);
                }
                Op::GetStatic(class, static_field_idx) => {
                    let static_field = class.static_fields()[static_field_idx];

                    self.stack.push(static_field.value());
                }
                other => unimplemented!("Tried to execute unimplemented op"),
            }

            ip += 1;
        }

        panic!("Execution should never fall off function")
    }
}
