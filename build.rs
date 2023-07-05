extern crate bindgen;
extern crate curl_sys;

use autotools::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

// Upstream version and name of the tarball file in the vendor directory
static WANDIO_VERSION: &str = "wandio-4.2.5-1";

// Extract wandio source to the auto generated build output directory
fn extract_wandio(build_output_dir: &str) -> std::io::Result<()> {
    Command::new("tar")
        .arg("-xf")
        .arg(format!("vendor/{WANDIO_VERSION}.tar.gz"))
        .arg("-C")
        .arg(build_output_dir)
        .status()
        .unwrap();

    Ok(())
}

// Run the bootstrap shell script that generates the configure script
fn run_bootstrap(build_output_dir: &str) -> std::io::Result<()> {
    Command::new("sh")
        .current_dir(format!("{}/{WANDIO_VERSION}", build_output_dir))
        .arg("bootstrap.sh")
        .status()
        .unwrap();

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Map the Rust auto generated build output directory to a friendly name
    let build_output_dir = env::var("OUT_DIR").unwrap();

    // Map the Rust auto generated build directory for curl to a friendly name
    let curl_root = env::var("DEP_CURL_ROOT").unwrap();

    println!("about to run extract");
    // Extract the wandio  tar file, must be done before setting "libdir_path"
    extract_wandio(&build_output_dir)?;

    // Map the directory name where wandio has been extracted too
    let libdir_path = PathBuf::from(format!("{}/{WANDIO_VERSION}", build_output_dir))
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // Run the build bootstrap
    run_bootstrap(&build_output_dir)?;

    // Run configure and make via the autotools crate
    let mut conf = Config::new(&libdir_path);
    conf.enable_static()
        .cflag("-DCURL_STATICLIB")
        .cflag(format!("-I{curl_root}/build/curl/lib/"))
        .ldflag(format!("-L{curl_root}/build/curl/lib/"))
        .disable_shared()
        .insource(true)
        .with("http", None);
    conf.build();

    // Map the directory where wandio's library's are located
    let wandio_libdir = format!("{build_output_dir}/{WANDIO_VERSION}/lib");

    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{wandio_libdir}/"))
        .clang_arg(format!("-I{wandio_libdir}/.libs/"))
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings out to file
    bindings
        .write_to_file(PathBuf::from(build_output_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Regenerate if changed
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=wandio");
    println!("cargo:rustc-link-search={wandio_libdir}");

    Ok(())
}
