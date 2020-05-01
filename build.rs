extern crate bindgen;

use std::env;
use std::path::PathBuf;

use std::path::Path;
use std::{fs, io};

fn scan_dir(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file: PathBuf = entry.path();

                match file.to_str() {
                    Some(filename) => {
                        // only append files ending with ".c" or ".cu"
                        if filename.ends_with(".c") || filename.ends_with(".cu") {
                            files.push(file);
                        }
                    }
                    None => {}
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let src_dir = String::from("zfp-0.5.5");

    let mut src_files: Vec<PathBuf> = Vec::new();

    #[cfg(feature = "cuda")]
    let mut cuda_files: Vec<PathBuf> = Vec::new();

    let _ = scan_dir(Path::new(&format!("{}/src", src_dir)), &mut src_files);

    #[cfg(feature = "cuda")]
    let _ = scan_dir(
        Path::new(&format!("{}/src/cuda_zfp", src_dir)),
        &mut cuda_files,
    );

    //build zfp with cc
    #[cfg(not(feature = "cuda"))]
    cc::Build::new()
        .flag("-O3")
        .include(format!("{}/include", src_dir))
        .files(src_files)
        .compile("zfp");

    //build zfp with cc with CUDA acceleration
    #[cfg(feature = "cuda")]
    cc::Build::new()
        .flag("-O3")
        .define("ZFP_WITH_CUDA", "ON")
        .cuda(true)
        .include(format!("{}/include", src_dir))
        .files(src_files)
        .files(cuda_files)
        .compile("zfp");

    /*println!("cargo:rustc-link-search=native={:?}/lib", zfp);
    println!("cargo:rustc-link-search=native={:?}/lib64", zfp);*/
    println!("cargo:rustc-link-lib=zfp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .blacklist_type("max_align_t")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // add the location of zfp header files
        .clang_arg("-I")
        .clang_arg(format!("{}/include", src_dir))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
