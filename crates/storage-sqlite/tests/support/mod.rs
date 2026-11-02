use rusqlite::Connection;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use uuid::Uuid;

/// Inspect persisted state or inject a failing trigger through an independent
/// connection, without exposing the repository's private connection.
pub fn connection(path: impl AsRef<Path>) -> Connection {
    let connection = Connection::open(path).unwrap();
    connection.busy_timeout(Duration::from_secs(5)).unwrap();
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .unwrap();
    connection
}

/// File-backed database for workflows that also need independent SQL checks.
pub struct TestDatabase {
    path: PathBuf,
}

impl TestDatabase {
    pub fn new() -> Self {
        Self {
            path: std::env::temp_dir()
                .join(format!("librett-integration-{}.sqlite", Uuid::new_v4())),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
