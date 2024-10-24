#[derive(Debug)]
pub enum Error {
    Native(NativeError),
}

#[derive(Debug)]
pub enum NativeError {
    ClassNotFound,
}
