extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
// use std::process::Command;

fn main() {
    // Command::new("sh").args(&["pre_build.sh"])
    //                     .status().unwrap();
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
    build();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(){
    let bindings = bindgen::Builder::default()
        .header("vendor/wrapper.hpp")
        .whitelist_type(".*Solver.*")
        .whitelist_type("Lit")
        .whitelist_type("*Clause*")
        .whitelist_type("Var")
        .whitelist_type("*SolverHelper*")
        .whitelist_function("getASolver")
        .rustfmt_bindings(true)
        .enable_cxx_namespaces()
        .clang_arg("-Ivendor/")
        .clang_arg(r"-std=c++11")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glucose_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build(){
    cc::Build::new()
        .include("vendor/")
        .cpp(true)
        .file("vendor/simp/SimpSolver.cc")
        .file("vendor/simp/SolverHelper.cc")
        .file("vendor/utils/System.cc")
        .file("vendor/utils/Options.cc")
        .file("vendor/core/Solver.cc")
        .flag_if_supported("-D__STDC_LIMIT_MACROS")
        .flag_if_supported("-D__STDC_FORMAT_MACROS")
        .flag_if_supported("-DNDEBUG")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-w")
        .opt_level(3)
        .compile("glucose");
}