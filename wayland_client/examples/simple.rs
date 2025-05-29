use wayland_client::connection::Connection;

mod wl {
    wayland_scanner::generate_client_mod!("/usr/share/wayland");
}

fn main() {
    let connection = Connection::default().unwrap();
    let display = connection.get_display();
}
