//! Paths anchored at the workspace root (parent of `backend/`), independent of process cwd.

use std::path::PathBuf;

pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("backend crate must live one level below workspace root")
        .to_path_buf()
}

pub fn data_file(name: &str) -> PathBuf {
    workspace_root().join("data").join(name)
}

pub fn log_file(name: &str) -> PathBuf {
    workspace_root().join("logs").join(name)
}
