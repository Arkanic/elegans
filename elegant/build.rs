use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/nanotode/source/connectome.c")
        .file("src/nanotode/source/muscles.c")
        .file("src/nanotode/source/neural_rom.c")
        .compile("libnanotode.a");
//    println!("cargo:rustc-link-lib=libnanotode.a");

    let bindings = bindgen::Builder::default()
        .header("src/nanotode.h")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}