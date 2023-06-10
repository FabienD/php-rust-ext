#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;

use serde_json;

#[php_function]
pub fn json_prettify(json: &str) -> String {
    match serde_json::to_string_pretty(json) {
        Ok(pretty_json) => return pretty_json,
        Err(_) => return String::from("Error"),
    };
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}