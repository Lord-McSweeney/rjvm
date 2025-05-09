use super::class::Class;
use super::context::Context;
use super::descriptor::{MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::method::{Exception, Method};
use super::object::Object;
use super::op::{ArrayType, Op};
use super::value::Value;

use crate::classfile::constant_pool::ConstantPoolEntry;
use crate::string::JvmString;

use std::cell::{Cell, Ref};
use std::cmp::Ordering;

pub struct Interpreter<'a> {
    method: Method,
    frame_reference: Ref<'a, Box<[Cell<Value>]>>,
    local_count: usize,
    local_base: usize,

    ip: usize,

    context: Context,
}

enum ControlFlow {
    Continue,
    ManualContinue,
    Return(Option<Value>),
}

impl<'a> Interpreter<'a> {
    pub fn new(
        context: Context,
        frame_reference: Ref<'a, Box<[Cell<Value>]>>,
        method: Method,
        args: &[Value],
    ) -> Result<Self, Error> {
        let prev_index = context.frame_index.get();

        let mut i = 0;
        while i < method.max_locals() {
            if i < args.len() {
                frame_reference[prev_index + i].set(args[i]);
            } else {
                frame_reference[prev_index + i].set(Value::Integer(0));
            }

            i += 1;
        }

        context.frame_index.set(prev_index + method.max_locals());

        Ok(Self {
            method,
            frame_reference,
            local_count: method.max_locals(),
            local_base: prev_index,

            ip: 0,

            context,
        })
    }

    fn handle_err(&mut self, error: Error, exceptions: &[Exception]) -> Result<(), Error> {
        match error {
            Error::Native(_) => Err(error),
            Error::Java(error_object) => {
                for exception in exceptions {
                    if self.ip >= exception.start && self.ip < exception.end {
                        // If the catch_class is None, this catch() { } matches any exception
                        if exception
                            .catch_class
                            .map_or(true, |c| error_object.class().matches_class(c))
                        {
                            self.ip = exception.target;

                            self.stack_clear();
                            self.stack_push(Value::Object(Some(error_object)));

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
            let op = &ops[self.ip];
            let control_flow = match op {
                Op::AConstNull => self.op_a_const_null(),
                Op::IConst(val) => self.op_i_const(*val),
                Op::LConst(val) => self.op_l_const(*val),
                Op::DConst(val) => self.op_d_const(*val),
                Op::Ldc(constant_pool_entry) | Op::Ldc2(constant_pool_entry) => {
                    self.op_ldc(*constant_pool_entry)
                }
                Op::ILoad(index) => self.op_i_load(*index),
                Op::LLoad(index) => self.op_l_load(*index),
                Op::DLoad(index) => self.op_d_load(*index),
                Op::ALoad(index) => self.op_a_load(*index),
                Op::IaLoad => self.op_ia_load(),
                Op::LaLoad => self.op_la_load(),
                Op::AaLoad => self.op_aa_load(),
                Op::BaLoad => self.op_ba_load(),
                Op::CaLoad => self.op_ca_load(),
                Op::IStore(index) => self.op_i_store(*index),
                Op::LStore(index) => self.op_l_store(*index),
                Op::DStore(index) => self.op_d_store(*index),
                Op::AStore(index) => self.op_a_store(*index),
                Op::IaStore => self.op_ia_store(),
                Op::LaStore => self.op_la_store(),
                Op::AaStore => self.op_aa_store(),
                Op::BaStore => self.op_ba_store(),
                Op::CaStore => self.op_ca_store(),
                Op::Pop => self.op_pop(),
                Op::Dup => self.op_dup(),
                Op::DupX1 => self.op_dup_x1(),
                Op::Dup2 => self.op_dup_2(),
                Op::IAdd => self.op_i_add(),
                Op::LAdd => self.op_l_add(),
                Op::DAdd => self.op_d_add(),
                Op::ISub => self.op_i_sub(),
                Op::LSub => self.op_l_sub(),
                Op::DSub => self.op_d_sub(),
                Op::IMul => self.op_i_mul(),
                Op::DMul => self.op_d_mul(),
                Op::IDiv => self.op_i_div(),
                Op::DDiv => self.op_d_div(),
                Op::IRem => self.op_i_rem(),
                Op::INeg => self.op_i_neg(),
                Op::IShl => self.op_i_shl(),
                Op::LShl => self.op_l_shl(),
                Op::IShr => self.op_i_shr(),
                Op::LShr => self.op_l_shr(),
                Op::LUshr => self.op_l_ushr(),
                Op::IAnd => self.op_i_and(),
                Op::LAnd => self.op_l_and(),
                Op::IOr => self.op_i_or(),
                Op::LOr => self.op_l_or(),
                Op::LXor => self.op_l_xor(),
                Op::IInc(index, amount) => self.op_i_inc(*index, *amount),
                Op::I2L => self.op_i2l(),
                Op::I2D => self.op_i2d(),
                Op::L2I => self.op_l2i(),
                Op::D2I => self.op_d2i(),
                Op::I2B => self.op_i2b(),
                Op::I2C => self.op_i2c(),
                Op::I2S => self.op_i2s(),
                Op::LCmp => self.op_l_cmp(),
                Op::IfEq(position) => self.op_if_eq(*position),
                Op::IfNe(position) => self.op_if_ne(*position),
                Op::IfLt(position) => self.op_if_lt(*position),
                Op::IfGe(position) => self.op_if_ge(*position),
                Op::IfGt(position) => self.op_if_gt(*position),
                Op::IfLe(position) => self.op_if_le(*position),
                Op::IfICmpEq(position) => self.op_if_i_cmp_eq(*position),
                Op::IfICmpNe(position) => self.op_if_i_cmp_ne(*position),
                Op::IfICmpLt(position) => self.op_if_i_cmp_lt(*position),
                Op::IfICmpGe(position) => self.op_if_i_cmp_ge(*position),
                Op::IfICmpGt(position) => self.op_if_i_cmp_gt(*position),
                Op::IfICmpLe(position) => self.op_if_i_cmp_le(*position),
                Op::IfACmpEq(position) => self.op_if_a_cmp_eq(*position),
                Op::IfACmpNe(position) => self.op_if_a_cmp_ne(*position),
                Op::Goto(position) => self.op_goto(*position),
                Op::TableSwitch(low_int, high_int, matches, default_offset) => {
                    self.op_table_switch(*low_int, *high_int, &**matches, *default_offset)
                }
                Op::LookupSwitch(matches, default_offset) => {
                    self.op_lookup_switch(&**matches, *default_offset)
                }
                Op::IReturn => self.op_i_return(),
                Op::AReturn => self.op_a_return(),
                Op::Return => Ok(ControlFlow::Return(None)),
                Op::GetStatic(class, static_field_idx) => {
                    self.op_get_static(*class, *static_field_idx)
                }
                Op::PutStatic(class, static_field_idx) => {
                    self.op_put_static(*class, *static_field_idx)
                }
                Op::GetField(class, field_idx) => self.op_get_field(*class, *field_idx),
                Op::PutField(class, field_idx) => self.op_put_field(*class, *field_idx),
                Op::InvokeVirtual(class, method_index) => {
                    self.op_invoke_virtual(*class, *method_index)
                }
                Op::InvokeSpecial(class, method) => self.op_invoke_special(*class, *method),
                Op::InvokeStatic(method) => self.op_invoke_static(*method),
                Op::InvokeInterface(class, (method_name, method_descriptor)) => {
                    self.op_invoke_interface(*class, *method_name, *method_descriptor)
                }
                Op::New(class) => self.op_new(*class),
                Op::NewArray(array_type) => self.op_new_array(*array_type),
                Op::ANewArray(class) => self.op_a_new_array(*class),
                Op::ArrayLength => self.op_array_length(),
                Op::AThrow => self.op_a_throw(),
                Op::CheckCast(class) => self.op_check_cast(*class),
                Op::InstanceOf(class) => self.op_instance_of(*class),
                Op::MultiANewArray(class, dim_count) => {
                    self.op_multi_a_new_array(*class, *dim_count)
                }
                Op::IfNull(position) => self.op_if_null(*position),
                Op::IfNonNull(position) => self.op_if_non_null(*position),
            };

            match control_flow {
                Ok(ControlFlow::Continue) => self.ip += 1,
                Ok(ControlFlow::ManualContinue) => {}
                Ok(ControlFlow::Return(value)) => {
                    // Reset frame index before returning
                    self.context.frame_index.set(self.local_base);

                    return Ok(value);
                }
                Err(error) => {
                    let result = self.handle_err(error, exceptions);

                    if let Err(error) = result {
                        // Reset frame index before returning
                        self.context.frame_index.set(self.local_base);

                        return Err(error);
                    }
                }
            }
        }

        panic!("Execution should never fall off function")
    }

    fn stack_push(&self, value: Value) {
        let prev = self.context.frame_index.get();
        self.frame_reference[prev].set(value);
        self.context.frame_index.set(prev + 1);
    }

    fn stack_pop(&self) -> Value {
        let new = self.context.frame_index.get() - 1;
        let result = self.frame_reference[new].get();
        self.context.frame_index.set(new);

        result
    }

    fn stack_clear(&self) {
        self.context
            .frame_index
            .set(self.local_base + self.local_count);
    }

    fn local_reg(&self, index: usize) -> Value {
        self.frame_reference[self.local_base + index].get()
    }

    fn set_local_reg(&self, index: usize, value: Value) {
        self.frame_reference[self.local_base + index].set(value);
    }

    fn op_a_const_null(&mut self) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Object(None));

        Ok(ControlFlow::Continue)
    }

    fn op_i_const(&mut self, value: i32) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Integer(value));

        Ok(ControlFlow::Continue)
    }

    fn op_l_const(&mut self, value: i8) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Long(value as i64));

        Ok(ControlFlow::Continue)
    }

    fn op_d_const(&mut self, value: f64) -> Result<ControlFlow, Error> {
        self.stack_push(Value::Double(value));

        Ok(ControlFlow::Continue)
    }

    fn op_ldc(&mut self, constant_pool_entry: ConstantPoolEntry) -> Result<ControlFlow, Error> {
        let class_file = self.method.class().class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let pushed_value = match constant_pool_entry {
            ConstantPoolEntry::String { string_idx } => {
                let string = constant_pool
                    .get_utf8(string_idx)
                    .expect("Should refer to valid entry");

                let string_chars = string.encode_utf16().collect::<Vec<_>>();

                Value::Object(Some(self.context.create_string(&string_chars)))
            }
            ConstantPoolEntry::Integer { value } => Value::Integer(value),
            ConstantPoolEntry::Class { name_idx } => {
                let class_name = constant_pool
                    .get_utf8(name_idx)
                    .expect("Should refer to valid entry");

                let class = self
                    .context
                    .lookup_class(class_name)
                    .expect("Class should exist");

                Value::Object(Some(self.context.class_object_for_class(class)))
            }
            ConstantPoolEntry::Double { value } => Value::Double(value),
            ConstantPoolEntry::Long { value } => Value::Long(value),
            _ => unimplemented!(),
        };

        self.stack_push(pushed_value);

        Ok(ControlFlow::Continue)
    }

    fn op_i_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_reg(index);

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_l_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_reg(index);

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_d_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_reg(index);

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_a_load(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let loaded = self.local_reg(index);

        self.stack_push(loaded);

        Ok(ControlFlow::Continue)
    }

    fn op_ia_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop().int();

        let array = self.stack_pop().object();
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

    fn op_la_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop().int();

        let array = self.stack_pop().object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                let result = array.get_long_at_index(index as usize);

                self.stack_push(Value::Long(result));

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_aa_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop().int();

        let array = self.stack_pop().object();
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
        let index = self.stack_pop().int();

        let array = self.stack_pop().object();
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

    fn op_ca_load(&mut self) -> Result<ControlFlow, Error> {
        let index = self.stack_pop().int();

        let array = self.stack_pop().object();
        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                let result = array.get_char_at_index(index as usize);

                self.stack_push(Value::Integer(result as i32));

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_i_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        self.set_local_reg(index, value);

        Ok(ControlFlow::Continue)
    }

    fn op_l_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        self.set_local_reg(index, value);

        Ok(ControlFlow::Continue)
    }

    fn op_d_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        self.set_local_reg(index, value);

        Ok(ControlFlow::Continue)
    }

    fn op_a_store(&mut self, index: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        self.set_local_reg(index, value);

        Ok(ControlFlow::Continue)
    }

    fn op_ia_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();
        let index = self.stack_pop().int();
        let array = self.stack_pop().object();

        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                array.set_integer_at_index(index as usize, value);

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_la_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().long();
        let index = self.stack_pop().int();
        let array = self.stack_pop().object();

        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                array.set_long_at_index(index as usize, value);

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_aa_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().object();
        let index = self.stack_pop().int();
        let array = self.stack_pop().object();

        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                array.set_object_at_index(index as usize, value);

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_ba_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();
        let index = self.stack_pop().int();
        let array = self.stack_pop().object();

        if let Some(array) = array {
            let length = array.array_length();
            if index < 0 || index as usize >= length {
                Err(self.context.array_index_oob_exception())
            } else {
                array.set_byte_at_index(index as usize, value as u8);

                Ok(ControlFlow::Continue)
            }
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_ca_store(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();
        let index = self.stack_pop().int();
        let array = self.stack_pop().object();

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

    fn op_pop(&mut self) -> Result<ControlFlow, Error> {
        self.stack_pop();

        Ok(ControlFlow::Continue)
    }

    fn op_dup(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        self.stack_push(value);
        self.stack_push(value);

        Ok(ControlFlow::Continue)
    }

    fn op_dup_x1(&mut self) -> Result<ControlFlow, Error> {
        let top_value = self.stack_pop();
        let under_value = self.stack_pop();

        self.stack_push(top_value);
        self.stack_push(under_value);
        self.stack_push(top_value);

        Ok(ControlFlow::Continue)
    }

    fn op_dup_2(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();
        if value.is_wide() {
            self.stack_push(value);
            self.stack_push(value);
        } else {
            let value2 = self.stack_pop();
            self.stack_push(value2);
            self.stack_push(value);
            self.stack_push(value2);
            self.stack_push(value);
        }

        Ok(ControlFlow::Continue)
    }

    fn op_i_add(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int1 + int2));

        Ok(ControlFlow::Continue)
    }

    fn op_l_add(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().long();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int1 + int2));

        Ok(ControlFlow::Continue)
    }

    fn op_d_add(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().double();
        let int2 = self.stack_pop().double();

        self.stack_push(Value::Double(int1 + int2));

        Ok(ControlFlow::Continue)
    }

    fn op_i_sub(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int2 - int1));

        Ok(ControlFlow::Continue)
    }

    fn op_l_sub(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().long();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int2 - int1));

        Ok(ControlFlow::Continue)
    }

    fn op_d_sub(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().double();
        let int2 = self.stack_pop().double();

        self.stack_push(Value::Double(int2 - int1));

        Ok(ControlFlow::Continue)
    }

    fn op_i_mul(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int1 * int2));

        Ok(ControlFlow::Continue)
    }

    fn op_d_mul(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().double();
        let int2 = self.stack_pop().double();

        self.stack_push(Value::Double(int1 * int2));

        Ok(ControlFlow::Continue)
    }

    fn op_i_div(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int1 == 0 {
            Err(self.context.arithmetic_exception())
        } else {
            self.stack_push(Value::Integer(int2 / int1));

            Ok(ControlFlow::Continue)
        }
    }

    fn op_d_div(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().double();
        let int2 = self.stack_pop().double();

        self.stack_push(Value::Double(int2 / int1));

        Ok(ControlFlow::Continue)
    }

    fn op_i_rem(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int1 == 0 {
            Err(self.context.arithmetic_exception())
        } else {
            self.stack_push(Value::Integer(int2 % int1));

            Ok(ControlFlow::Continue)
        }
    }

    fn op_i_neg(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Integer(-int));

        Ok(ControlFlow::Continue)
    }

    fn op_i_shl(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int2 << (int1 & 0x1F)));

        Ok(ControlFlow::Continue)
    }

    fn op_l_shl(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int2 << (int1 & 0x3F)));

        Ok(ControlFlow::Continue)
    }

    fn op_i_shr(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int2 >> (int1 & 0x1F)));

        Ok(ControlFlow::Continue)
    }

    fn op_l_shr(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int2 >> (int1 & 0x1F)));

        Ok(ControlFlow::Continue)
    }

    fn op_l_ushr(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().long() as u64;

        self.stack_push(Value::Long((int2 >> (int1 & 0x3F)) as i64));

        Ok(ControlFlow::Continue)
    }

    fn op_i_and(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int1 & int2));

        Ok(ControlFlow::Continue)
    }

    fn op_l_and(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().long();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int1 & int2));

        Ok(ControlFlow::Continue)
    }

    fn op_i_or(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        self.stack_push(Value::Integer(int1 | int2));

        Ok(ControlFlow::Continue)
    }

    fn op_l_or(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().long();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int1 | int2));

        Ok(ControlFlow::Continue)
    }

    fn op_l_xor(&mut self) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().long();
        let int2 = self.stack_pop().long();

        self.stack_push(Value::Long(int1 ^ int2));

        Ok(ControlFlow::Continue)
    }

    fn op_i_inc(&mut self, index: usize, amount: i32) -> Result<ControlFlow, Error> {
        let loaded = self.local_reg(index).int();

        self.set_local_reg(index, Value::Integer(loaded + amount));

        Ok(ControlFlow::Continue)
    }

    fn op_i2l(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Long(int as i64));

        Ok(ControlFlow::Continue)
    }

    fn op_i2d(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Double(int as f64));

        Ok(ControlFlow::Continue)
    }

    fn op_l2i(&mut self) -> Result<ControlFlow, Error> {
        let long = self.stack_pop().long();

        self.stack_push(Value::Integer(long as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_d2i(&mut self) -> Result<ControlFlow, Error> {
        let double = self.stack_pop().double();

        self.stack_push(Value::Integer(double as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_i2b(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Integer((int as u8) as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_i2c(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Integer((int as u16) as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_i2s(&mut self) -> Result<ControlFlow, Error> {
        let int = self.stack_pop().int();

        self.stack_push(Value::Integer((int as i16) as i32));

        Ok(ControlFlow::Continue)
    }

    fn op_l_cmp(&mut self) -> Result<ControlFlow, Error> {
        let long2 = self.stack_pop().long();
        let long1 = self.stack_pop().long();

        match long1.cmp(&long2) {
            Ordering::Greater => {
                self.stack_push(Value::Integer(1));
            }
            Ordering::Equal => {
                self.stack_push(Value::Integer(0));
            }
            Ordering::Less => {
                self.stack_push(Value::Integer(-1));
            }
        }

        Ok(ControlFlow::Continue)
    }

    fn op_if_eq(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value == 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_ne(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value != 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_lt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value < 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_ge(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value >= 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_gt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value > 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_le(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        if value <= 0 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_eq(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 == int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_ne(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 != int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_lt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 < int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_ge(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 >= int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_gt(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 > int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_i_cmp_le(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let int1 = self.stack_pop().int();
        let int2 = self.stack_pop().int();

        if int2 <= int1 {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_a_cmp_eq(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let obj1 = self.stack_pop().object();
        let obj2 = self.stack_pop().object();

        match (obj1, obj2) {
            (None, None) => self.ip = position,
            (Some(obj1), Some(obj2)) if obj1.ptr_eq(obj2) => self.ip = position,
            _ => self.ip += 1,
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_a_cmp_ne(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let obj1 = self.stack_pop().object();
        let obj2 = self.stack_pop().object();

        match (obj1, obj2) {
            (Some(obj1), Some(obj2)) if !obj1.ptr_eq(obj2) => self.ip = position,
            (Some(_), None) => self.ip = position,
            (None, Some(_)) => self.ip = position,
            _ => self.ip += 1,
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_goto(&mut self, position: usize) -> Result<ControlFlow, Error> {
        self.ip = position;

        Ok(ControlFlow::ManualContinue)
    }

    fn op_table_switch(
        &mut self,
        low_int: i32,
        high_int: i32,
        matches: &[usize],
        default_offset: usize,
    ) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();
        if value < low_int || value > high_int {
            self.ip = default_offset;
        } else {
            self.ip = matches[(value - low_int) as usize];
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_lookup_switch(
        &mut self,
        matches: &[(i32, usize)],
        default_offset: usize,
    ) -> Result<ControlFlow, Error> {
        let value = self.stack_pop().int();

        for (matched_value, offset) in matches {
            if value == *matched_value {
                self.ip = *offset;

                return Ok(ControlFlow::ManualContinue);
            }
        }

        self.ip = default_offset;

        Ok(ControlFlow::ManualContinue)
    }

    fn op_i_return(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        Ok(ControlFlow::Return(Some(value)))
    }

    fn op_a_return(&mut self) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        Ok(ControlFlow::Return(Some(value)))
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
        let object = self.stack_pop().object();

        if let Some(object) = object {
            if !object.is_of_class(class) {
                // TODO verify this in verifier
                panic!("Object on stack was of wrong Class");
            }

            self.stack_push(object.get_field(field_idx));

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_put_field(&mut self, class: Class, field_idx: usize) -> Result<ControlFlow, Error> {
        let value = self.stack_pop();

        let object = self.stack_pop().object();

        if let Some(object) = object {
            if !object.is_of_class(class) {
                // TODO verify this in verifier
                panic!("Object on stack was of wrong Class");
            }

            object.set_field(field_idx, value);

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_invoke_virtual(
        &mut self,
        class: Class,
        method_index: usize,
    ) -> Result<ControlFlow, Error> {
        // Increment the gc counter (can this even do an allocation?)
        self.context.increment_gc_counter();

        // We need to know the number of args, so let's lookup the method defined by
        // the base class to get the descriptor- this is the wrong method, but we
        // can still use its descriptor
        let method_descriptor = class
            .instance_method_vtable()
            .get_element(method_index)
            .descriptor();

        let mut args = vec![Value::Object(None); method_descriptor.args().len() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop().object();

        if let Some(receiver) = receiver {
            if !receiver.is_of_class(class) {
                // TODO verify this in verifier
                panic!("Object on stack was of wrong Class");
            }

            let receiver_class = receiver.class();
            let method = receiver_class
                .instance_method_vtable()
                .get_element(method_index);

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
        // Increment the gc counter (can this even do an allocation?)
        self.context.increment_gc_counter();

        let mut args = vec![Value::Integer(0); method.arg_count() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop().object();
        if let Some(receiver) = receiver {
            if !receiver.is_of_class(class) {
                // TODO verify this in verifier
                panic!("Object on stack was of wrong Class");
            }

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

    fn op_invoke_static(&mut self, method: Method) -> Result<ControlFlow, Error> {
        // Increment the gc counter (can this even do an allocation?)
        self.context.increment_gc_counter();

        let mut args = vec![Value::Integer(0); method.arg_count()];
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

    fn op_invoke_interface(
        &mut self,
        _class: Class,
        method_name: JvmString,
        method_descriptor: MethodDescriptor,
    ) -> Result<ControlFlow, Error> {
        // Increment the gc counter (can this even do an allocation?)
        self.context.increment_gc_counter();

        let mut args = vec![Value::Integer(0); method_descriptor.args().len() + 1];
        for arg in args.iter_mut().skip(1).rev() {
            *arg = self.stack_pop();
        }

        let receiver = self.stack_pop().object();

        if let Some(receiver) = receiver {
            // Should we check that the receiver is of the class?
            let receiver_class = receiver.class();
            let method_vtable = receiver_class.instance_method_vtable();

            let method_idx = method_vtable
                .lookup((method_name, method_descriptor))
                .ok_or(Error::Native(NativeError::VTableLookupFailed))?;
            let method = method_vtable.get_element(method_idx);

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

    fn op_new(&mut self, class: Class) -> Result<ControlFlow, Error> {
        // This does an allocation; we should increment the gc counter
        self.context.increment_gc_counter();

        let instance = class.new_instance(self.context.gc_ctx);

        self.stack_push(Value::Object(Some(instance)));

        Ok(ControlFlow::Continue)
    }

    fn op_new_array(&mut self, array_type: ArrayType) -> Result<ControlFlow, Error> {
        // This does an allocation; we should increment the gc counter
        self.context.increment_gc_counter();

        let array_length = self.stack_pop().int();
        if array_length < 0 {
            return Err(self.context.negative_array_size_exception());
        }

        let array_length = array_length as usize;

        let array_object = match array_type {
            ArrayType::Char => {
                let chars = vec![0; array_length];

                Object::char_array(self.context, &chars)
            }
            ArrayType::Byte => {
                let bytes = vec![0; array_length];

                Object::byte_array(self.context, &bytes)
            }
            ArrayType::Int => {
                let ints = vec![0; array_length];

                Object::int_array(self.context, &ints)
            }
            ArrayType::Long => {
                let longs = vec![0; array_length];

                Object::long_array(self.context, &longs)
            }
            ArrayType::Boolean => {
                let bools = vec![false; array_length];

                Object::bool_array(self.context, &bools)
            }
            _ => unimplemented!("Array type {:?} unimplemented", array_type),
        };

        self.stack_push(Value::Object(Some(array_object)));

        Ok(ControlFlow::Continue)
    }

    fn op_a_new_array(&mut self, class: Class) -> Result<ControlFlow, Error> {
        // This does an allocation; we should increment the gc counter
        self.context.increment_gc_counter();

        let array_length = self.stack_pop().int();
        if array_length < 0 {
            return Err(self.context.negative_array_size_exception());
        }

        let nulls = vec![None; array_length as usize];

        let array_object = Object::obj_array(self.context, class, &nulls);
        self.stack_push(Value::Object(Some(array_object)));

        Ok(ControlFlow::Continue)
    }

    fn op_array_length(&mut self) -> Result<ControlFlow, Error> {
        let object = self.stack_pop().object();

        if let Some(object) = object {
            let length = object.array_length();

            self.stack_push(Value::Integer(length as i32));

            Ok(ControlFlow::Continue)
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_a_throw(&mut self) -> Result<ControlFlow, Error> {
        let object = self.stack_pop().object();

        if let Some(object) = object {
            let throwable_class_name = self.context.common.java_lang_throwable;
            let throwable_class = self
                .context
                .lookup_class(throwable_class_name)
                .expect("Throwable class should exist");

            // TODO do this verification in the verifier
            if !object.is_of_class(throwable_class) {
                panic!("Class of object on stack should extend or be Throwable");
            }

            Err(Error::Java(object))
        } else {
            Err(self.context.null_pointer_exception())
        }
    }

    fn op_check_cast(&mut self, class: Class) -> Result<ControlFlow, Error> {
        let obj = self.stack_pop().object();

        // TODO: Special rules around handling arrays, see comment in op_instance_of

        if let Some(obj) = obj {
            if !obj.is_of_class(class) && !obj.implements_interface(class) {
                return Err(self.context.class_cast_exception());
            }
        }

        self.stack_push(Value::Object(obj));

        Ok(ControlFlow::Continue)
    }

    fn op_instance_of(&mut self, class: Class) -> Result<ControlFlow, Error> {
        let obj = self.stack_pop().object();

        // TODO: Special rules around handling arrays; currently, we have all
        // arrays extend Object, so only Object and the array class itself
        // will pass these tests. See JVMS 6.5.

        if let Some(obj) = obj {
            if obj.is_of_class(class) || obj.implements_interface(class) {
                self.stack_push(Value::Integer(1));
            } else {
                self.stack_push(Value::Integer(0));
            }
        } else {
            self.stack_push(Value::Integer(0));
        }

        Ok(ControlFlow::Continue)
    }

    fn op_multi_a_new_array(&mut self, class: Class, dim_count: u8) -> Result<ControlFlow, Error> {
        // This does an allocation; we should increment the gc counter
        self.context.increment_gc_counter();

        // FIXME: Do this without the temporary Vec

        let mut dimensions = Vec::with_capacity(dim_count as usize);
        for _ in 0..dim_count {
            let array_length = self.stack_pop().int();
            if array_length < 0 {
                return Err(self.context.negative_array_size_exception());
            }

            dimensions.push(array_length as usize);
        }
        dimensions.reverse();

        // Now that we have the dimensions, let's create the array

        fn recursive_create_array(
            context: Context,
            class: Class,
            dimensions: &Vec<usize>,
            dim_index: usize,
        ) -> Option<Object> {
            let Some(elem_count) = dimensions.get(dim_index).copied() else {
                // The iterator is finished; we're going to fill the elements of the
                // innermost arrays with `null`.
                return None;
            };

            let mut elems = vec![None; elem_count];
            for elem in elems.iter_mut() {
                *elem = recursive_create_array(context, class, dimensions, dim_index + 1);
            }

            let mut descriptor = ResolvedDescriptor::Class(class);
            let mut class = class;
            let levels_left = (dimensions.len() - dim_index) - 1;
            for _ in 0..levels_left {
                class = context.array_class_for(descriptor);
                descriptor = ResolvedDescriptor::Array(class);
            }

            Some(Object::obj_array(context, class, &elems))
        }

        let object = recursive_create_array(self.context, class, &dimensions, 0)
            .expect("dim_count did not == 0");
        self.stack_push(Value::Object(Some(object)));

        Ok(ControlFlow::Continue)
    }

    fn op_if_null(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let obj = self.stack_pop().object();

        if obj.is_none() {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }

    fn op_if_non_null(&mut self, position: usize) -> Result<ControlFlow, Error> {
        let obj = self.stack_pop().object();

        if obj.is_some() {
            self.ip = position;
        } else {
            self.ip += 1;
        }

        Ok(ControlFlow::ManualContinue)
    }
}
