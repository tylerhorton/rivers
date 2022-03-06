use crate::event::{Event, FromEvent, IntoEvent};

#[derive(Clone)]
pub struct Key(pub Vec<u8>);
impl FromEvent for Key {
    fn from_event(e: &mut Event) -> Self {
        Self(e.key.clone())
    }
}
impl IntoEvent for Key {
    fn into_event(self, e: &mut Event) {
        e.key = self.0;
    }
}

#[derive(Clone)]
pub struct Value(pub Vec<u8>);
impl FromEvent for Value {
    fn from_event(e: &mut Event) -> Self {
        Self(e.value.clone())
    }
}
impl IntoEvent for Value {
    fn into_event(self, e: &mut Event) {
        e.value = self.0;
    }
}
