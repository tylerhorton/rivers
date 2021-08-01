pub trait Transport {
    fn consume(&self, topic: &str) -> (&[u8], &[u8]);
    fn produce(&self, topic: &str, key: Vec<u8>, value: Vec<u8>);
}
