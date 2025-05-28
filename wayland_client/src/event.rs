pub struct Registry;
pub struct Callback;
pub enum Event<'a> {
    RegistryGlobal {
        registry: Registry,
        name: u32,
        interface: &'a str,
        version: u32,
    },
    RegistryGlobalRemove {
        registry: Registry,
        name: u32,
    },
    CallbackDone {
        callback: Callback,
        callback_data: u32,
    },
}
