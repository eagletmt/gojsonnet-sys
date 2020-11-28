fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let c_bindings = "vendor/go-jsonnet/c-bindings";
    println!("cargo:rerun-if-changed={}", c_bindings);
    println!("cargo:rustc-link-lib=static=gojsonnet");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    let mut cmd = std::process::Command::new("go");
    if std::env::var("DOCS_RS").is_ok() {
        cmd.env("XDG_CACHE_HOME", "/tmp/.cache");
    }
    let status = cmd
        .current_dir(c_bindings)
        .arg("build")
        .arg("-mod=vendor")
        .arg("-buildmode=c-archive")
        .arg(format!("-o={}/libgojsonnet.a", out_dir.display()))
        .status()
        .expect("Failed to launch Go compiler");
    if !status.success() {
        panic!("Failed to build libgojsonnet.a");
    }
    println!("cargo:rustc-link-lib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("vendor/go-jsonnet/cpp-jsonnet/include/libjsonnet.h")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
