#[macro_export]
macro_rules! read_u8 {
    ($context:expr, $data:expr) => {
        $data
            .read_u8()
            .map_err(|e| crate::runtime::error::Error::from_class_file_error($context, e.into()))?
    };
}

#[macro_export]
macro_rules! read_u16_be {
    ($context:expr, $data:expr) => {
        $data
            .read_u16_be()
            .map_err(|e| crate::runtime::error::Error::from_class_file_error($context, e.into()))?
    };
}

#[macro_export]
macro_rules! read_u32_be {
    ($context:expr, $data:expr) => {
        $data
            .read_u32_be()
            .map_err(|e| crate::runtime::error::Error::from_class_file_error($context, e.into()))?
    };
}
