#[allow(dead_code)]
mod protocol {
    pub mod wl {
        pub mod display {
            pub struct Display;
            pub struct DeleteIdEvent {}
            pub struct ErrorEvent {}
        }

        pub mod registry {
            pub struct Registry;
            pub struct GlobalEvent {}
            pub struct GlobalRemoveEvent {}
        }
    }
    pub enum Event {
        WlDisplayDeleteId(wl::display::DeleteIdEvent),
        WlDisplayError(wl::display::ErrorEvent),
        WlRegistryGlobal(wl::registry::GlobalEvent),
        WlRegistryGlobalRemove(wl::registry::GlobalRemoveEvent),
    }
}
