/// Generates Rust/Swift bridge code given declarations in the Rust sources
fn main() {
    let out_dir = std::path::PathBuf::from("./");

    // Rust sources to analyze are passed in as arguments
    let bridges: Vec<String> = std::env::args().skip(1).collect();

    // generate the C and Swift bridge code from the sources
    swift_bridge_build::parse_bridges(bridges).write_all_concatenated(out_dir, "gen");
}
