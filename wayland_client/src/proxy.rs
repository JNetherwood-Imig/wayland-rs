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

    pub(crate) fn equals(&self, other: &Self) -> bool {
        self.id == other.id
    }

    pub(crate) const fn interface() -> &'static str {
        I::INTERFACE
    }

    pub(crate) const fn max_version() -> u32 {
        I::MAX_VERSION
    }

    pub(crate) const fn version() -> u32 {
        V
    }
}
