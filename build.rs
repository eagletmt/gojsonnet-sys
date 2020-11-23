fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let c_bindings = "vendor/go-jsonnet/c-bindings";
    println!("cargo:rerun-if-changed={}", c_bindings);
    println!("cargo:rustc-link-lib=static=gojsonnet");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    let status = std::process::Command::new("go")
        .current_dir(c_bindings)
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg(format!("-o={}/libgojsonnet.a", out_dir.display()))
        .status()
        .expect("Failed to launch Go compiler");
    if !status.success() {
        panic!("Failed to build libgojsonnet.a");
    }
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
