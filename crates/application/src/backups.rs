#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupInfo {
    pub name: String,
    pub size: u64,
    pub modified: u64,
}
