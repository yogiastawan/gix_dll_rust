use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("gix_dll").build_target("all").build();

    let bindings = bindgen::Builder::default()
        .header("gix_dll/src/include/gix_dll/gix_dll.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );
    // println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=gix-dllist");
}
