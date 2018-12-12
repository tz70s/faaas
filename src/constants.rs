// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

// Deployment url, to language backend.
pub static DEPLOYMENT_URL: &'static str = "http://127.0.0.1:5999/deploy";

// Invocation url, for http-based param propagation.
pub static INVOCATION_URL: &'static str = "http://127.0.0.1:5999/invoke";

// Unix domain socket path, for uds-based param propagation.
pub static UDS_SOCKET_PATH: &'static str =
    "/tmp/microcall/action.sock";

// Runtime directory, path to storing functions.
pub static RUNTIME_DIRECTORY: &'static str = "/tmp/microcall";
