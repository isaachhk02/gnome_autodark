mod daemon;
fn main() {
    println!("GNOME Auto-Dark Mode by hhk02");
    daemon::start_daemon();
}
