use std::path::Path;

use cargo::ops::compile;

fn main() {
    println!("Hello, world!");

    let manifest_path = Path::new("Cargo.toml");
    let config = cargo::util::config::Config::default().unwrap();
    let ws = cargo::core::Workspace::new(manifest_path, &config).unwrap();

    let options =
        cargo::ops::CompileOptions::new(&config, cargo::core::compiler::CompileMode::Build)
            .unwrap();

    let res = compile(&ws, &options).unwrap();

    res.binaries.iter().for_each(|bin| {
        println!("bin: {}", bin.path.display());
    });
}
