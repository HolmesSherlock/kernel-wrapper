extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Control which binding is to be generated
    let gen_header = 2;

    // The location to place Rust bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if gen_header == 1 {
        // Get kernel header location from STD_KERNEL_PATH variable supplied by Makefile
        let std_kernel_path = match std::env::var("STD_KERNEL_PATH") {
            Ok(string) => string,
            Err(error) => {
                panic!("Missing environment variable STD_KERNEL_PATH, run from Makefile: {:?}", error);
            }
        };

        // Change current directory to the location of kernel headers
        // let kernel_path = std::path::Path::new(std_kernel_path.as_str());
        // assert!(std::env::set_current_dir(&kernel_path).is_ok());
        
        // Tell clang where to find kernel headers by passing -I <include dir> switch
        let mut clang_arg: String = "-I".to_owned();
        clang_arg.push_str(&std_kernel_path);
        clang_arg.push_str(&"/include");

        // Generate bindings for headers listed in kernel-wrapper.h
        let bindings = bindgen::Builder::default()
            .header("kernel-wrapper.h")
            .clang_arg(clang_arg)
            .generate()
            .expect("Unable to generate kernel bindings");

        bindings
            .write_to_file(out_path.join("binding.rs"))
            .expect("Couldn't write kernel bindings!");
    }

    else {
        // Generate bindings for headers listed in sample.h
        let bindings = bindgen::Builder::default()
            .header("sample.h")
            .generate()
            .expect("Unable to generate local bindings");

        bindings
            .write_to_file(out_path.join("binding.rs"))
            .expect("Couldn't write local bindings!");
    }
}
