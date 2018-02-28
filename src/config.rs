// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

// Deployment url, to language backend.
pub static DEPLOY_URL: &'static str = "http://127.0.0.1:5999/deploy";

// Invocation url, for http-based param propagation.
pub static INVOKE_URL: &'static str = "http://127.0.0.1:5999/invoke";

// Unix domain socket path, for uds-based param propagation.
pub static UDS_SOCKET_PATH: &'static str =
    "/Users/tzuchiao/workspace/rust/faaas/actions/node-js-v8-uds/action.sock";

// Runtime directory, path to storing functions.
pub static RUNTIME_DIRECTORY: &'static str = "/Users/tzuchiao/workspace/rust/faaas/tmp";
