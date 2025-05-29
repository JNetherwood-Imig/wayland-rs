#[macro_export]
macro_rules! generate_client_mod {
    ($path: literal) => {
        scanner_macro::generate_client_protocols!($path);
    };
}

mod _test_mod {
    scanner_macro::generate_client_protocols!("/usr/share/wayland/");
}

#[cfg(test)]
mod tests {
    mod wl {
        crate::generate_client_mod!("/usr/share/wayland/");
    }
    #[test]
    fn does_it_work() {}
}
