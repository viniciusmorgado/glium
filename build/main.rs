use std::env;
use std::fs::File;
use std::path::Path;

mod textures;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&dest);

    textures::build_texture_file(&mut File::create(&dest.join("textures.rs")).unwrap());
    println!("cargo:rerun-if-changed=build/main.rs");
}
