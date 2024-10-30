use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::method::{Exception, Method};
use super::object::Object;
use super::op::{ArrayType, Op};
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

enum ControlFlow {
    Continue,
    ManualContinue,
    Return(Option<Value>),
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

    fn handle_err(&mut self, error: Error, exceptions: &[Exception]) -> Result<(), Error> {
        match error {
            Error::Native(_) => Err(error),
            Error::Java(error_object) => {
                for exception in exceptions {
                    if self.ip >= exception.start && self.ip < exception.end {
                        if error_object.class().matches_class(exception.catch_class) {
                            self.ip = exception.target;

                            self.stack.clear();
                            self.stack.push(Value::Object(Some(error_object)));

                            return Ok(());
                        }
                    }
                }

                Err(error)
            }
        }
    }

    pub fn interpret_ops(
        &mut self,
        ops: &[Op],
        exceptions: &[Exception],
    ) -> Result<Option<Value>, Error> {
        while self.ip < ops.len() {
            let op = ops[self.ip];
            let control_flow = match op {
                Op::AConstNull => self.op_a_const_null(),
                Op::IConst(val) => self.op_i_const(val),
                Op::Ldc(constant_pool_entry) => self.op_ldc(constant_pool_entry),
                Op::ILoad(index) => self.op_i_load(index),
                Op::ALoad(index) => self.op_a_load(index),
                Op::IaLoad => self.op_ia_load(),
                Op::AaLoad => self.op_aa_load(),
                Op::BaLoad => self.op_ba_load(),
                Op::IStore(index) => self.op_i_store(index),
                Op::AStore(index) => self.op_a_store(index),
                Op::CaStore => self.op_ca_store(),
                Op::Dup => self.op_dup(),
                Op::IAdd => self.op_i_add(),
                Op::ISub => self.op_i_sub(),
                Op::IDiv => self.op_i_div(),
                Op::IRem => self.op_i_rem(),
                Op::INeg => self.op_i_neg(),
                Op::IInc(index, amount) => self.op_i_inc(index, amount),
                Op::I2C => self.op_i2c(),
                Op::IfEq(position) => self.op_if_eq(position),
                Op::IfNe(position) => self.op_if_ne(position),
                Op::IfLt(position) => self.op_if_lt(position),
                Op::IfGe(position) => self.op_if_ge(position),
                Op::IfLe(position) => self.op_if_le(position),
                Op::IfICmpNe(position) => self.op_if_i_cmp_ne(position),
                Op::IfICmpGe(position) => self.op_if_i_cmp_ge(position),
                Op::IfICmpGt(position) => self.op_if_i_cmp_gt(position),
                Op::IfICmpLe(position) => self.op_if_i_cmp_le(position),
                Op::Goto(position) => self.op_goto(position),
                Op::IReturn => self.op_i_return(),
                Op::AReturn => self.op_a_return(),
                Op::Return => Ok(ControlFlow::Return(None)),
                Op::GetStatic(class, static_field_idx) => {
                    self.op_get_static(class, static_field_idx)
                }
                Op::PutStatic(class, static_field_idx) => {
                    self.op_put_static(class, static_field_idx)
                }
                Op::GetField(class, field_idx) => self.op_get_field(class, field_idx),
                Op::PutField(class, field_idx) => self.op_put_field(class, field_idx),
                Op::InvokeVirtual((method_name, method_descriptor)) => {
                    self.op_invoke_virtual(method_name, method_descriptor)
                }
                Op::InvokeSpecial(class, method) => self.op_invoke_special(class, method),
                Op::InvokeStatic(method) => self.op_invoke_static(method),
                Op::New(class) => self.op_new(class),
                Op::NewArray(array_type) => self.op_new_array(array_type),
                Op::ArrayLength => self.op_array_length(),
                Op::AThrow => todo!(),
                Op::IfNonNull(position) => self.op_if_non_null(position),
            };

            match control_flow {
                Ok(ControlFlow::Continue) => self.ip += 1,
                Ok(ControlFlow::ManualContinue) => {}
                Ok(ControlFlow::Return(value)) => return Ok(value),
                Err(error) => self.handle_err(error, exceptions)?,
            }
        }

        panic!("Execution should never fall off function")
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn op_a_const_null(&mut self) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Object(None));

        Ok(ControlFlow::Continue)
    }

    fn op_i_const(&mut self, value: i32) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Integer(value));

        Ok(ControlFlow::Continue)
    }

    fn op_ldc(&mut self, constant_pool_entry: ConstantPoolEntry) -> Result<ControlFlow, Error> {
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
            ConstantPoolEntry::Integer { value } => Value::Integer(value),
            _ => unimplemented!(),
        };

        self.stack_push(pushed_value);

        Ok(ControlFlow::Continue)
    }

    fn op_i_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_registers[index];

        if !matches!(loaded, Value::Integer(_)) {
            panic!("Local should be of integer type");
        }

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_a_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_registers[index];

        if !matches!(loaded, Value::Object(_)) {
            panic!("Local should be of reference type");
        }

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_ia_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop();

        let Value::Integer(index) = index else {
            panic!("Stack value should be of integer type");
        };

        let array = self.stack_pop().expect_as_object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                let result = array.get_integer_at_index(index as usize);

                self.stack_push(Value::Integer(result));

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_aa_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop();

        let Value::Integer(index) = index else {
            panic!("Stack value should be of integer type");
        };

        let array = self.stack_pop().expect_as_object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                let result = array.get_object_at_index(index as usize);

                self.stack_push(Value::Object(result));

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_ba_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop();

        let Value::Integer(index) = index else {
            panic!("Stack value should be of integer type");
        };

        let array = self.stack_pop().expect_as_object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                let result = array.get_byte_at_index(index as usize);

                self.stack_push(Value::Integer(result as i32));

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_i_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        if !matches!(value, Value::Integer(_)) {
            panic!("Stack value should be of integer type");
        }

        self.local_registers[index] = value;

        Ok(ControlFlow::Continue)
    }

    fn op_a_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        if !matches!(value, Value::Object(_)) {
            panic!("Stack value should be of reference type");
        }

        self.local_registers[index] = value;

        Ok(ControlFlow::Continue)
    }

    fn op_ca_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let Value::Integer(value) = value else {
            panic!("Stack value should be of integer type");
        };

        let index = self.stack_pop();

        let Value::Integer(index) = index else {
            panic!("Stack value should be of integer type");
        };

        let array = self.stack_pop().expect_as_object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                array.set_char_at_index(index as usize, value as u16);

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_dup(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        self.stack_push(value);
        self.stack_push(value);

        Ok(ControlFlow::Continue)
    }

    fn op_i_add(&mut self) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let value2 = self.stack_pop();

        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        self.stack_push(Value::Integer(int1 + int2));

        Ok(ControlFlow::Continue)
    }

    fn op_i_sub(&mut self) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let value2 = self.stack_pop();

        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        self.stack_push(Value::Integer(int2 - int1));

        Ok(ControlFlow::Continue)
    }

    fn op_i_div(&mut self) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let value2 = self.stack_pop();

        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        self.stack_push(Value::Integer(int2 / int1));

        Ok(ControlFlow::Continue)
    }

    fn op_i_rem(&mut self) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let value2 = self.stack_pop();

        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        if int1 == 0 {
            Err(Error::Native(NativeError::ArithmeticException))
        } else {
            self.stack_push(Value::Integer(int2 % int1));

            Ok(ControlFlow::Continue)
        }
    }

    fn op_i_neg(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        self.stack_push(Value::Integer(-int));

        Ok(ControlFlow::Continue)
    }

    fn op_i_inc(&mut self, index: usize, amount: i32) -> Result<ControlFlow, Error> {
        let loaded = self.local_registers[index];

        let Value::Integer(loaded) = loaded else {
            panic!("Local should be of integer type");
        };

        self.local_registers[index] = Value::Integer(loaded + amount);

        Ok(ControlFlow::Continue)
    }

    fn op_i2c(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        self.stack_push(Value::Integer((int as u16) as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_if_eq(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        if int == 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_ne(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        if int != 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_lt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        if int < 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_ge(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        if int >= 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_le(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(int) = value else {
            panic!("Stack value should be of integer type");
        };

        if int <= 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_ne(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let value2 = self.stack_pop();
        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        if int2 != int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_ge(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let value2 = self.stack_pop();
        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        if int2 >= int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_gt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let value2 = self.stack_pop();
        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        if int2 > int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_le(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value1 = self.stack_pop();
        let Value::Integer(int1) = value1 else {
            panic!("Stack value should be of integer type");
        };

        let value2 = self.stack_pop();
        let Value::Integer(int2) = value2 else {
            panic!("Stack value should be of integer type");
        };

        if int2 <= int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_goto(&mut self, position: usize) -> Result<ControlFlow, Error> {
        self.ip = position;

        Ok(ControlFlow::ManualContinue)
    }

    fn op_i_return(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Integer(value) = value else {
            panic!("Stack value should be of integer type");
        };

        Ok(ControlFlow::Return(Some(Value::Integer(value))))
    }

    fn op_a_return(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Object(value) = value else {
            panic!("Stack value should be of reference type");
        };

        Ok(ControlFlow::Return(Some(Value::Object(value))))
    }

    fn op_get_static(
        &mut self,
        class: Class,
        static_field_idx: usize,
    ) -> Result<ControlFlow, Error> {
        let static_field = class.static_fields()[static_field_idx];

        self.stack_push(static_field.value());

        Ok(ControlFlow::Continue)
    }

    fn op_put_static(
        &mut self,
        class: Class,
        static_field_idx: usize,
    ) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let static_field = class.static_fields()[static_field_idx];

        static_field.set_value(value);

        Ok(ControlFlow::Continue)
    }

    fn op_get_field(&mut self, class: Class, field_idx: usize) -> Result<ControlFlow, Error> {
        let object = self.stack_pop();
        if !object.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        let object = object.expect_as_object();
        if let Some(object) = object {
            self.stack_push(object.get_field(field_idx));

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_put_field(&mut self, class: Class, field_idx: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let object = self.stack_pop();
        if !object.is_of_class(class) {
            panic!("Object on stack was of wrong Class");
        }

        let object = object.expect_as_object();
        if let Some(object) = object {
            object.set_field(field_idx, value);

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_invoke_virtual(
        &mut self,
        method_name: JvmString,
        method_descriptor: MethodDescriptor,
    ) -> Result<ControlFlow, Error> {
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

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_invoke_special(&mut self, class: Class, method: Method) -> Result<ControlFlow, Error> {
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
            return Err(self.context.null_pointer_exception());
        }

        args[0] = receiver;

        let result = method.exec(self.context, &args)?;
        if let Some(result) = result {
            self.stack_push(result);
        }

        Ok(ControlFlow::Continue)
    }

    fn op_invoke_static(&mut self, method: Method) -> Result<ControlFlow, Error> {
        let mut args = vec![Value::Object(None); method.arg_count()];
        for arg in args.iter_mut().rev() {
            // TODO: Long and Double arguments require two pops
            *arg = self.stack_pop();
        }

        let result = method.exec(self.context, &args)?;
        if let Some(result) = result {
            self.stack_push(result);
        }

        Ok(ControlFlow::Continue)
    }

    fn op_new(&mut self, class: Class) -> Result<ControlFlow, Error> {
        let instance = class.new_instance(self.context.gc_ctx);

        self.stack_push(Value::Object(Some(instance)));

        Ok(ControlFlow::Continue)
    }

    fn op_new_array(&mut self, array_type: ArrayType) -> Result<ControlFlow, Error> {
        let array_length = self.stack_pop();
        let Value::Integer(array_length) = array_length else {
            panic!("Stack value should be of integer type");
        };

        if array_length < 0 {
            return Err(Error::Native(NativeError::NegativeArraySizeException));
        }

        let array_length = array_length as usize;

        let array_object = match array_type {
            ArrayType::Char => {
                let chars = vec![0; array_length];

                Object::char_array(self.context, &chars)
            }
            ArrayType::Int => {
                let ints = vec![0; array_length];

                Object::int_array(self.context, &ints)
            }
            _ => unimplemented!("Array type unimplemented"),
        };

        self.stack_push(Value::Object(Some(array_object)));

        Ok(ControlFlow::Continue)
    }

    fn op_array_length(&mut self) -> Result<ControlFlow, Error> {
        let object = self.stack_pop().expect_as_object();

        if let Some(object) = object {
            let length = object.array_length();

            self.stack_push(Value::Integer(length as i32));

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_if_non_null(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        let Value::Object(obj) = value else {
            panic!("Stack value should be of reference type");
        };

        if obj.is_some() {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }
}
