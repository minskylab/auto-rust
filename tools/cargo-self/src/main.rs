use std::{fs::canonicalize, path::PathBuf};

use cargo::{core::Shell, ops::compile};

fn main() {
    println!("Hello, Cargo Self v0.2.0!");

    // let manifest_path_buf = PathBuf::from("./Cargo.toml");
    let manifest_path = canonicalize(PathBuf::from("./Cargo.toml")).unwrap();
    // let a = Box::new(std::io::stdout());
    // let buf = BufWriter::new(Vec::new());
    // Shell::;

    // let shell = Shell::from_write(Box::new(buf));
    // let shell = Shell::default();
    let config =
        // cargo::util::config::Config::new(shell, manifest_path.to_owned(), manifest_path.to_owned());
        cargo::util::config::Config::default().unwrap();

    // config.registry_cache_path();

    // cargo::core::Workspace::new(manifest_path, config)
    let ws = cargo::core::Workspace::new(&manifest_path, &config).unwrap();

    let options =
        cargo::ops::CompileOptions::new(&config, cargo::core::compiler::CompileMode::Build)
            .unwrap();

    let res = compile(&ws, &options).unwrap();

    res.binaries.iter().for_each(|bin| {
        println!("bin: {}", bin.path.display());
    });
}
