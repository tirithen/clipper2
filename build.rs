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
        .flag_if_supported("-std=c++17")
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
            .allowlist_type("PolygonsC")
            .allowlist_type("ClipTypeC")
            .allowlist_type("JoinTypeC")
            .allowlist_type("EndTypeC")
            .allowlist_type("PathTypeC")
            .allowlist_type("VertexC")
            .allowlist_type("PathC")
            .allowlist_type("PolygonC")
            .allowlist_function("inflate_c")
            .allowlist_function("intersect_c")
            .allowlist_function("union_c")
            .allowlist_function("difference_c")
            .allowlist_function("xor_c")
            .allowlist_function("free_path_c")
            .allowlist_function("free_polygon_c")
            .allowlist_function("free_polygons_c")
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
