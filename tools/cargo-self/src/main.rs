use std::{fs::canonicalize, path::PathBuf};

use cargo::ops::compile;

// use async_openai::{
//     types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
//     Client,
// };

// use cargo::{ops::compile, util::rustc};

#[tokio::main]
async fn main() {
    println!("Hello, Cargo Self v0.2.0!");

    // let manifest_path_buf = PathBuf::from("./Cargo.toml");
    let manifest_path = canonicalize(PathBuf::from("./Cargo.toml")).unwrap();
    // let a = Box::new(std::io::stdout());
    // let buf = BufWriter::new(Vec::new());
    // Shell::;

    // let shell = Shell::from_write(Box::new(buf));
    // let shell = Shell::default();
    let config = cargo::util::config::Config::default().unwrap();

    // config.registry_cache_path();

    // cargo::core::Workspace::new(manifest_path, config)
    let ws = cargo::core::Workspace::new(&manifest_path, &config).unwrap();

    let options =
        cargo::ops::CompileOptions::new(&config, cargo::core::compiler::CompileMode::Build)
            .unwrap();

    // set example to named workspace member

    // println!("options.spec: {}", options.spec);

    // let client = Client::new();

    // let request = CreateChatCompletionRequestArgs::default()
    //     .max_tokens(512u16)
    //     .model("gpt-3.5-turbo")
    //     .messages([
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::System)
    //             .content("You are a helpful assistant.")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Who won the world series in 2020?")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::Assistant)
    //             .content("The Los Angeles Dodgers won the World Series in 2020.")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Where was it played?")
    //             .build()
    //             .unwrap(),
    //     ])
    //     .build()
    //     .unwrap();

    // let response = client.chat().create(request).await.unwrap();

    ws.members()
        // .filter(|member| member.library().is_some())
        .for_each(|member| {
            println!("member: {}", member.name());
        });

    // rustc::Rustc::new(path, wrapper, workspace_wrapper, rustup_rustc, cache_location, config)

    let res = compile(&ws, &options).unwrap();

    res.binaries.iter().for_each(|bin| {
        println!("bin: {}", bin.path.display());
    });
}
