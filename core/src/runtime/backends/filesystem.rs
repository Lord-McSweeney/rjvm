// Filesystem trait

pub trait FilesystemBackend {
    fn to_absolute_path(&self, path: &str) -> String;

    fn to_canonical_path(&self, path: &str) -> String;

    fn file_exists(&self, path: &str) -> Result<bool, ()>;

    fn write_by_descriptor(&self, descriptor: u32, data: &[u8]);

    fn read_by_descriptor(&self, descriptor: u32, buffer: &mut [u8]) -> Result<(), ()>;

    fn writeable_descriptor_from_path(&self, path: &str) -> Result<u32, ()>;

    fn readable_descriptor_from_path(&self, path: &str) -> Result<u32, ()>;
}
