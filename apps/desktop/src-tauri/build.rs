fn main() {
    // generate_context! embeds cached copies of the icon assets. Track their
    // originals so Cargo recompiles the app when generated icons change.
    println!("cargo:rerun-if-changed=icons");
    println!("cargo:rerun-if-changed=Info.plist");
    tauri_build::build();
}
