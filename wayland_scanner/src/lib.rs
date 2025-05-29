#[cfg(test)]
mod tests {
    #[test]
    fn does_it_work() {
        scanner_macro::generate_client_protocols!("/usr/share/wayland/");
    }
}
