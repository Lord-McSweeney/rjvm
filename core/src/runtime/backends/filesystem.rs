// Filesystem trait

pub trait FilesystemBackend {
    // Path-related methods.
    fn to_absolute_path(&self, path: &str) -> String;

    fn to_canonical_path(&self, path: &str) -> String;

    // File data methods.
    fn file_exists(&self, path: &str) -> Result<bool, ()>;

    // Methods to do a file operation, given a descriptor.
    fn write_by_descriptor(&self, descriptor: u32, data: &[u8]);

    fn read_by_descriptor(&self, descriptor: u32, buffer: &mut [u8]) -> Result<(), ()>;

    fn available_bytes(&self, descriptor: u32) -> u64;

    // Methods to get a descriptor from a path.
    fn writeable_descriptor_from_path(&self, path: &str) -> Result<u32, ()>;

    fn readable_descriptor_from_path(&self, path: &str) -> Result<u32, ()>;
}
