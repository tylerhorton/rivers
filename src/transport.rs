pub trait Transport {
    fn consume(&self, topic: &str) -> (Option<&[u8]>, Option<&[u8]>);
    fn produce(&self, topic: &str, key: Option<Vec<u8>>, value: Option<Vec<u8>>);
}
