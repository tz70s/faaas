// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use std::fs;
use uuid::Uuid;
use std::fs::File;
use std::io::prelude::*;

enum LanguageRuntimeFs {
    NodeJsV6,
    NodeJsV8,
    PythonV2,
    PythonV3,
    JavaV8,
    DynamicLibrary
}

impl LanguageRuntimeFs {
    fn to_str(&self) -> &str {
        match *self {
            LanguageRuntimeFs::NodeJsV6 => "node-js-v6",
            LanguageRuntimeFs::NodeJsV8 => "node-js-v8",
            LanguageRuntimeFs::PythonV2 => "python-v3",
            LanguageRuntimeFs::PythonV3 => "python-v2",
            LanguageRuntimeFs::JavaV8 => "java-v8",
            LanguageRuntimeFs::DynamicLibrary => "dynamic-library"
        }
    }
}

static RUNTIME_FS: &'static str = "tmp";

pub fn create_runtime_fs() {
    match fs::create_dir(RUNTIME_FS) {
        // FIXME: Ignore all error currently, we should only deal with permission denied error.
        Err(err) => {},
        Ok(_) => {}
    };
}

pub fn mount_nodejs_v8() {
    match fs::create_dir(format!("{}/{}", RUNTIME_FS, LanguageRuntimeFs::NodeJsV8.to_str())) {
        // FIXME: Ignore all error currently, we should only deal with permission denied error.
        Err(err) => {},
        Ok(_) => {}
    }
}

pub fn mount_language_codes(id: &Uuid, content: &str) {
    let directory = format!("{}/{}/{}", RUNTIME_FS, LanguageRuntimeFs::NodeJsV8.to_str(), id);
    fs::create_dir(&directory);
    let mut file = File::create(format!("{}/index.js", directory)).unwrap();
    file.write_all(content.as_ref());
}

pub fn clean_up() {
    fs::remove_dir_all(RUNTIME_FS).unwrap();
}
