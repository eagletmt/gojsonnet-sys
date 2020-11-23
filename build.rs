fn main() {
    gobuild::Build::new()
        .buildmode(gobuild::BuildMode::CArchive)
        .file("./vendor/go-jsonnet/c-bindings")
        .compile("gojsonnet");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::copy(
        "vendor/go-jsonnet/c-bindings/internal.h",
        out_dir.join("internal.h"),
    )
    .expect("Unable to copy internal.h");
    println!("cargo:rustc-link-lib=stdc++");

    let bindings = bindgen::Builder::default()
        .header(out_dir.join("libgojsonnet.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
