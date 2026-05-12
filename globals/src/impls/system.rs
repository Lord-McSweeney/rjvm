use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::Cell;
use rjvm_core::{Array, Context, Error, NativeMethod, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/System.arraycopy.(Ljava/lang/Object;ILjava/lang/Object;II)V", array_copy),
        ("java/lang/System.identityHashCode.(Ljava/lang/Object;)I", identity_hash_code),
    ];

    context.register_native_mappings(mappings);
}

// java/lang/System: static void arraycopy(Object, int, Object, int, int)
fn array_copy(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let source_arr = args[0].object();
    let Some(source_arr) = source_arr else {
        return Err(context.null_pointer_exception());
    };

    let source_start = args[1].int();

    let dest_arr = args[2].object();
    let Some(dest_arr) = dest_arr else {
        return Err(context.null_pointer_exception());
    };

    let dest_start = args[3].int();

    let length = args[4].int();

    if source_start < 0 || dest_start < 0 || length < 0 {
        return Err(context.array_index_oob_exception());
    }

    let source_start = source_start as usize;
    let dest_start = dest_start as usize;
    let length = length as usize;

    let (Some(_), Some(dest_value_type)) = (
        source_arr.class().array_value_type(),
        dest_arr.class().array_value_type(),
    ) else {
        return Err(context.array_store_exception());
    };

    let source_array_data = source_arr.array_data();
    let dest_array_data = dest_arr.array_data();

    if source_start + length > source_array_data.len()
        || dest_start + length > dest_array_data.len()
    {
        return Err(context.array_index_oob_exception());
    }

    match (source_array_data, dest_array_data) {
        (Array::ByteArray(source_data), Array::ByteArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::CharArray(source_data), Array::CharArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::DoubleArray(source_data), Array::DoubleArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::FloatArray(source_data), Array::FloatArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::IntArray(source_data), Array::IntArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::LongArray(source_data), Array::LongArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::ShortArray(source_data), Array::ShortArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::ObjectArray(source_data), Array::ObjectArray(dest_data)) => {
            let Some(dest_value_class) = dest_value_type.class() else {
                unreachable!()
            };

            let mut temp_arr = Vec::with_capacity(length);

            for i in 0..length {
                let source_idx = source_start + i;
                temp_arr.push(source_data[source_idx].get());
            }

            for i in 0..length {
                let obj = temp_arr[i];
                if let Some(obj) = obj {
                    if !obj.class().check_cast(dest_value_class) {
                        return Err(context.array_store_exception());
                    }
                }

                let dest_idx = dest_start + i;
                dest_data[dest_idx].set(obj);
            }
        }
        (_, _) => {
            return Err(context.array_store_exception());
        }
    }

    Ok(None)
}

#[inline(never)]
fn primitive_array_copy<T: Copy + Default>(
    source_data: &Box<[Cell<T>]>,
    dest_data: &Box<[Cell<T>]>,
    source_start: usize,
    dest_start: usize,
    length: usize,
) {
    #[inline(never)]
    fn copy_nonoverlapping<T: Copy + Default>(
        source_data: &[Cell<T>],
        dest_data: &[Cell<T>],
        source_start: usize,
        dest_start: usize,
        length: usize,
    ) {
        // TODO optimize this

        for i in 0..length {
            let source_idx = source_start + i;
            let dest_idx = dest_start + i;
            dest_data[dest_idx].set(source_data[source_idx].get());
        }
    }

    #[inline(never)]
    fn copy_overlapping<T: Copy + Default>(
        source_data: &[Cell<T>],
        dest_data: &[Cell<T>],
        source_start: usize,
        dest_start: usize,
        length: usize,
    ) {
        // TODO: Can we avoid the temporary allocation?

        let temp_arr = vec![Cell::new(T::default()); length];

        for i in 0..length {
            let source_idx = source_start + i;
            temp_arr[i].set(source_data[source_idx].get());
        }

        for i in 0..length {
            let dest_idx = dest_start + i;
            dest_data[dest_idx].set(temp_arr[i].get());
        }
    }

    let overlapping = if core::ptr::eq(&**source_data, &**dest_data) {
        let dst_start_in_source = source_start <= dest_start && source_start + length > dest_start;
        let src_start_in_dest = dest_start <= source_start && dest_start + length > source_start;

        dst_start_in_source || src_start_in_dest
    } else {
        // Not the same array, can't be overlapping
        false
    };

    if overlapping {
        copy_overlapping(source_data, dest_data, source_start, dest_start, length);
    } else {
        copy_nonoverlapping(source_data, dest_data, source_start, dest_start, length);
    }
}

fn identity_hash_code(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let object = args[0].object();

    // `identityHashCode(null)` is `0`
    let result = object.map(crate::hash_code::calc_hash_code).unwrap_or(0);

    Ok(Some(Value::Integer(result)))
}
