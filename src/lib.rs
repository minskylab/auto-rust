// extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;

#[proc_macro]
pub fn implement(_item: TokenStream) -> TokenStream {
    println!("implement: {:?}", _item);
    let resp = reqwest::blocking::get("https://httpbin.org/ip")
        .unwrap()
        .json::<HashMap<String, String>>()
        .unwrap();

    let ip = resp.get("origin").unwrap().to_owned();

    format!("{} {{\"{}\".into()}}", _item, ip).parse().unwrap()
}
