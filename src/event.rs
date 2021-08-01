pub struct Event<K, V> {
    pub key: Option<K>,
    pub value: Option<V>,
}

impl<K, V> Event<K, V> {
    pub fn new(key: Option<K>, value: Option<V>) -> Self {
        Event { key, value }
    }
}
