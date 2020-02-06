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
        .header("vendor/wrapper.h")
        .whitelist_type("*CGlucose*")
        .whitelist_function("cglucose_init")
        .whitelist_function("cglucose_assume")
        .whitelist_function("cglucose_solve")
        .whitelist_function("cglucose_val")
        .whitelist_function("cglucose_add_to_clause")
        .whitelist_function("cglucose_commit_clause")
        .whitelist_function("cglucose_clean_clause")
        .whitelist_function("cglucose_solver_nodes")
        .whitelist_function("cglucose_nb_learnt")
        .whitelist_function("cglucose_set_random_seed")
        .rustfmt_bindings(true)
        // .enable_cxx_namespaces()
        .clang_arg("-Ivendor/")
        // .clang_arg(r"-std=c++11")
        // .clang_arg("-xc++")
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
        .file("vendor/wrapper.cpp")
        .flag_if_supported("-D__STDC_LIMIT_MACROS")
        .flag_if_supported("-D__STDC_FORMAT_MACROS")
        .flag_if_supported("-DNDEBUG")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-w")
        .opt_level(3)
        .compile("glucose");
}