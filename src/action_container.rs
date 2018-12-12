// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use uuid::Uuid;

use constants::RUNTIME_DIRECTORY;

#[allow(dead_code)]
enum LanguageRuntime {
    NodeJsV6,
    NodeJsV8,
    PythonV2,
    PythonV3,
    JavaV8,
    DynamicLibrary,
}

impl LanguageRuntime {
    fn to_str(&self) -> &str {
        match *self {
            LanguageRuntime::NodeJsV6 => "node-js-v6",
            LanguageRuntime::NodeJsV8 => "node-js-v8",
            LanguageRuntime::PythonV2 => "python-v3",
            LanguageRuntime::PythonV3 => "python-v2",
            LanguageRuntime::JavaV8 => "java-v8",
            LanguageRuntime::DynamicLibrary => "dynamic-library",
        }
    }
}

/// Remove existed file directory, and create a new one.
pub fn create_runtime_container() {
    // Check existed.
    if let Err(err) = fs::remove_dir_all(RUNTIME_DIRECTORY) {
        debug!(
            "Runtime directory removing failed, {}",
            err.description()
        )
    };

    if let Err(err) = fs::create_dir(RUNTIME_DIRECTORY) {
        error!(
            "error occurred while creating directory {}",
            err.description()
        );
    }
}

pub fn mount_nodejs_v8() {
    let file_directory = format!(
        "{}/{}",
        RUNTIME_DIRECTORY,
        LanguageRuntime::NodeJsV8.to_str()
    );

    if let Err(err) = fs::create_dir(file_directory) {
        error!(
            "error occurred while creating directory {}",
            err.description()
        );
    }
}

pub fn mount_language_codes(id: &Uuid, content: &str) {
    let directory = format!(
        "{}/{}/{}",
        RUNTIME_DIRECTORY,
        LanguageRuntime::NodeJsV8.to_str(),
        id
    );
    fs::create_dir(&directory).unwrap();
    let mut file = File::create(format!("{}/index.js", directory)).unwrap();
    file.write_all(content.as_ref()).unwrap();
}
