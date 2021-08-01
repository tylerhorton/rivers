pub trait FromBytes
where
    Self: Sized,
{
    fn from_bytes(data: &[u8]) -> Option<Self>;
}

impl FromBytes for u8 {
    fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            None
        } else {
            Some(data[0])
        }
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl ToBytes for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![(*self & 0x00FF) as u8, ((*self & 0xFF00) >> 8) as u8]
    }
}
