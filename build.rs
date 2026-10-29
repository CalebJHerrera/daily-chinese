use std::path::Path;

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(root).join("src/ui/app.slint");

    slint_build::compile(path).unwrap();
}
