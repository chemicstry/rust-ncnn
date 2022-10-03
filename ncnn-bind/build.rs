extern crate bindgen;
use cmake::Config;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn output_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn ncnn_src_dir() -> PathBuf {
    output_dir().join("ncnn-src")
}

fn fetch() -> io::Result<()> {
    let target_dir = ncnn_src_dir();

    if target_dir.exists() {
        return Ok(());
    }

    let tag = "20220729";

    let status = Command::new("git")
        .arg("clone")
        .arg("--recursive")
        .arg("--depth=1")
        .arg("-b")
        .arg(tag)
        .arg("https://github.com/Tencent/ncnn")
        .arg(&target_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

fn build() -> io::Result<()> {
    let mut config = Config::new(ncnn_src_dir());
    config.define("NCNN_BUILD_TOOLS", "OFF");
    config.define("NCNN_BUILD_EXAMPLES", "OFF");
    config.define("NCNN_BUILD_BENCHMARK", "OFF");
    config.define("CMAKE_BUILD_TYPE", "Release");

    if cfg!(feature = "vulkan") {
        config.define("NCNN_VULKAN", "ON");
    }

    if cfg!(feature = "dynamic") {
        config.define("NCNN_SHARED_LIB", "ON");
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.display());

    Ok(())
}

fn search_include(include_paths: &[PathBuf], header: &str) -> String {
    for dir in include_paths {
        let include = dir.join(header);
        if fs::metadata(&include).is_ok() {
            return include.as_path().to_str().unwrap().to_string();
        }
    }
    format!("/usr/include/{}", header)
}

fn main() {
    println!("cargo:rerun-if-env-changed=NCNN_DIR");

    let include_paths: Vec<PathBuf> = if let Ok(ncnn_dir) = env::var("NCNN_DIR") {
        // use prebuild ncnn dir
        let dir = PathBuf::from(ncnn_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            dir.join("lib").to_string_lossy()
        );

        vec![dir.join("include").join("ncnn")]
    } else {
        // fetch from github and build
        fetch().unwrap();
        build().unwrap();

        println!(
            "cargo:rustc-link-search=native={}",
            output_dir().join("lib").to_string_lossy()
        );

        vec![output_dir().join("include").join("ncnn")]
    };

    if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=dylib=ncnn");
    } else {
        println!("cargo:rustc-link-lib=static=ncnn");
    }

    if !cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=pthread");
    }

    let mut builder = bindgen::Builder::default();

    let files = vec!["c_api.h"];
    for file in files {
        builder = builder.header(search_include(&include_paths, file));
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(output_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
