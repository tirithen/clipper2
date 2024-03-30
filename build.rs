use std::env;

fn main() {
    println!("cargo:rerun-if-changed=clipper");
    if cfg!(feature = "update-bindings") {
        println!("cargo:rerun-if-changed=generated");
    }

    cc::Build::new()
        .cpp(true)
        .opt_level(3)
        .file("clipper2/clipper.engine.cpp")
        .file("clipper2/clipper.offset.cpp")
        .file("clipper2/clipper.rectclip.cpp")
        .file("clipper2/wrapper.cpp")
        .flag_if_supported("-std:c++17") // MSVC
        .flag_if_supported("-std=c++17") // GCC, Clang, etc.
        .compile("clipper2");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {}
        _ => unimplemented!(
            "target_os: {}, target_env: {}",
            target_os.as_str(),
            target_env.as_str()
        ),
    }

    #[cfg(feature = "generate-bindings")]
    {
        let bindings = bindgen::Builder::default()
            .header("clipper2/wrapper.h")
            .allowlist_type("Point")
            .allowlist_type("PathsC")
            .allowlist_type("FillRuleC")
            .allowlist_function("union_c")
            .allowlist_function("free_paths_c")
            .allowlist_function("get_points")
            .allowlist_function("get_path_starts")
            .allowlist_function("get_num_paths")
            .size_t_is_usize(true)
            .generate()
            .expect("unable to generate bindings");

        let out_path = if cfg!(feature = "update-bindings") {
            std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated")
        } else {
            std::path::PathBuf::from(env::var("OUT_DIR").unwrap())
        };

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("couldn't write bindings!");
    }
}
