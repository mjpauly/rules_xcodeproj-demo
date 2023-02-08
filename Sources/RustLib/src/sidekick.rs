#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_from_sidekick();
    }
}

fn print_from_sidekick() {
    println!("Hello from sidekick Rust module");
}
