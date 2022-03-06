use frunk::{HCons, HList, HNil};

#[derive(Clone, Debug)]
pub struct Event {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Event {
    pub fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
        Event { key, value }
    }
}

impl FromEvent for Event {
    fn from_event(e: &mut Event) -> Self {
        e.clone()
    }
}

impl IntoEvent for Event {
    fn into_event(self, e: &mut Event) {
        *e = self;
    }
}

pub trait FromEvent {
    fn from_event(e: &mut Event) -> Self;
}

pub trait IntoEvent {
    fn into_event(self, e: &mut Event);
}

macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

macro_rules! impl_from_event {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<$($ty,)*> FromEvent for ($($ty,)*)
        where
            $( $ty: FromEvent,)*
        {
            fn from_event(e: &mut Event) -> Self {
                ($($ty::from_event(e),)*)
            }
        }
    };
}

macro_rules! impl_to_event {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<$($ty,)*> IntoEvent for ($($ty,)*)
        where
            $( $ty: IntoEvent,)*
        {
            fn into_event(self, e: &mut Event) {
                let l: HList![$($ty,)*] = self.into();
                l.into_event(e);
            }
        }
    };
}

impl<H, T> IntoEvent for HCons<H, T>
where
    H: IntoEvent,
    T: IntoEvent,
{
    fn into_event(self, e: &mut Event) {
        self.head.into_event(e);
        self.tail.into_event(e);
    }
}

impl IntoEvent for HNil {
    fn into_event(self, _e: &mut Event) {}
}

all_the_tuples!(impl_from_event);
all_the_tuples!(impl_to_event);
