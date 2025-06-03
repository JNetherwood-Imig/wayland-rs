#[macro_export]
macro_rules! generate_client_mod {
    () => {
        scanner_macro::generate_client_protocols!();
    };
}

mod __cargo_expand_test_mod {
    scanner_macro::generate_client_protocols!();
}

#[cfg(test)]
mod tests {
    mod protocol {
        crate::generate_client_mod!();
    }
    #[test]
    fn does_it_work() {}
}
