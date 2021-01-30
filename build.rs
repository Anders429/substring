extern crate autocfg;

fn main() {
    let ac = autocfg::new();
    ac.emit_rustc_version(1, 6);
    ac.emit_rustc_version(1, 20);
}
