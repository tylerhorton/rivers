pub trait Serializer<T>: Fn(&T) -> Vec<u8> {}
impl<T, F> Serializer<T> for F where F: Fn(&T) -> Vec<u8> {}

pub trait Deserializer<T>: Fn(&[u8]) -> Result<T, String> {}
impl<T, F> Deserializer<T> for F where F: Fn(&[u8]) -> Result<T, String> {}

pub(crate) fn serialize<T, S: Serializer<T>>(t: &Option<T>, serializer: &S) -> Option<Vec<u8>> {
    t.as_ref().map(|k| (serializer)(k))
}

pub(crate) fn deserialize<T, D: Deserializer<T>>(
    data: Option<&[u8]>,
    deserializer: &D,
) -> Option<T> {
    match data {
        Some(bytes) => match (deserializer)(bytes) {
            Ok(t) => Some(t),
            Err(e) => {
                println!("{}", e);
                None
            }
        },
        None => None,
    }
}
