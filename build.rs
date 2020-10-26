fn main() {
    if autocfg::new().probe_rustc_version(1, 6) {
        autocfg::emit("no_std");
    }
}
