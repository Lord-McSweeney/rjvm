// System trait

pub trait SystemBackend {
    fn exit(&self, exit_code: i32) -> !;
}
