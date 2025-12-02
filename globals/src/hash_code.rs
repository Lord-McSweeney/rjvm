use rjvm_core::{Class, Object};

pub fn calc_hash_code(object: Object) -> i32 {
    let object_class = object.class();

    let ptr_obj = Object::as_ptr(object) as usize;
    let ptr_cls = Class::as_ptr(object_class) as usize;

    let mut result = (ptr_cls << 8) + ptr_obj;
    result >>= 3;
    result ^= 0xed0f87;
    result ^= (91 + (result & 0xFF)) << 24;
    result += 143;

    result as i32
}
