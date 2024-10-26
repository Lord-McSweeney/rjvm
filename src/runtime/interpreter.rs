use super::context::Context;
use super::error::Error;
use super::method::Method;
use super::op::Op;
use super::value::Value;

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
