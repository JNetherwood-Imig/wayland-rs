// use wayland_client::connection::Connection;

// mod wl {
//     wayland_scanner::generate_client_mod!("/usr/share/wayland");
// }

fn main() {
    // let connection = Connection::default().unwrap();
    // let display = connection.get_display();
}

#[cfg(test)]
mod tests {
    #[test]
    use wayland_client::Fixed;
    fn fixed() {
        let fix = Fixed::from(1.234f64);
        assert_eq!(fix.into::<i32>(), 1);
        assert_eq!(fix.into::<u32>(), 1);
        assert_eq!(fix.into::<f32>(), 1.234);
        assert_eq!(fix.into::<f64>(), 1.234);
    }

    fn negative_fixed() {
        let negative_fix = Fixed::from(-43.21);
        assert_eq!(negative_fix.into::<i32>(), -43);
        assert_eq!(negative_fix.into::<u32>(), 0);
        assert_eq!(negative_fix.into::<f32>(), -43.21);
        assert_eq!(negative_fix.into::<f64>(), -43.21);
    }
}
