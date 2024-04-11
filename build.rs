use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, utils};

const LIB_NAME: &str = "Couenne";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    let want_system = utils::want_system(LIB_NAME);
    
    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["IPOPT".to_string()];

        let (_, coinflags_other) = coinbuilder::get_metadata_from("Bonmin");
        coinflags.extend(coinflags_other);

        coinbuilder::print_metadata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        src_dir.to_string(),
        format!("{}/util", src_dir),
        format!("{}/expression", src_dir),
        format!("{}/expression/operators", src_dir),
        format!("{}/expression/operators/bounds", src_dir),
        format!("{}/expression/partial", src_dir),
        format!("{}/expression", src_dir),
        format!("{}/standardize", src_dir),
        format!("{}/problem", src_dir),
        format!("{}/problem/depGraph", src_dir),
        format!("{}/bound_tightening", src_dir),
        format!("{}/bound_tightening/twoImpliedBT", src_dir),
        format!("{}/convex", src_dir),
        format!("{}/branch", src_dir),
        format!("{}/disjunctive", src_dir),
        format!("{}/interfaces", src_dir),
        format!("{}/heuristics", src_dir),
        format!("{}/cut/sdpcuts", src_dir),
        format!("{}/cut/crossconv", src_dir),
        format!("{}/main", src_dir),
    ];

    let mut lib_sources = include_str!("couenne_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

// heuristics
// branch

    let mut coinflags = vec!["Couenne".to_string()];

    let (include_other, coinflags_other) = coinbuilder::get_metadata_from("Bonmin");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.define("COUENNELIB_BUILD", None);
    config
        .define("F77_FUNC(name,NAME)", Some("name ## _"))
        .define("F77_FUNC(name,NAME)", Some("name ## _"));
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);
}
