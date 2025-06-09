pub trait Interface {
    const INTERFACE: &'static str;
    const MAX_VERSION: u32;
}

pub struct Proxy<I: Interface, const V: u32> {
    id: u32,
    _interface: std::marker::PhantomData<I>,
}

impl<I: Interface, const V: u32> Proxy<I, V> {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id: id,
            _interface: std::marker::PhantomData,
        }
    }
}
