// Filesystem trait

pub trait FilesystemBackend {
    fn to_absolute_path(&self, path: &str) -> String;

    fn to_canonical_path(&self, path: &str) -> String;

    fn file_exists(&self, path: &str) -> Result<bool, ()>;

    fn write_by_descriptor(&self, descriptor: i32, data: &[u8]);
}
