use super::*;
use librett_application::BackupInfo;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn io_error<T>(result: std::io::Result<T>) -> Result<T, ApplicationError> {
    result.map_err(|_| ApplicationError::Storage)
}
fn backup_name(name: &str) -> Result<(), ApplicationError> {
    if name.is_empty()
        || name.len() > 160
        || !name.ends_with(".sqlite")
        || !name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.'))
        || name.contains("..")
    {
        return Err(ApplicationError::InvalidBackup);
    }
    Ok(())
}

// Compare executable schema too: integrity checks do not detect missing tables,
// changed constraints, views or injected triggers in an otherwise valid database.
fn schema(
    connection: &Connection,
) -> rusqlite::Result<Vec<(String, String, String, Option<String>)>> {
    let mut statement = connection.prepare(
        "SELECT type,name,tbl_name,sql FROM sqlite_schema WHERE name NOT GLOB 'sqlite_*' ORDER BY type,name",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;
    rows.collect()
}
impl SqliteTournamentRepository {
    pub fn create_backup(
        &self,
        directory: &Path,
        prefix: &str,
    ) -> Result<BackupInfo, ApplicationError> {
        io_error(fs::create_dir_all(directory))?;
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ApplicationError::Storage)?
            .as_secs();
        let name = format!("librett-{prefix}-{stamp}-{}.sqlite", Uuid::new_v4());
        backup_name(&name)?;
        let target = directory.join(&name);
        let mut destination = Connection::open(&target).map_err(|_| ApplicationError::Storage)?;
        let copied = rusqlite::backup::Backup::new(&self.connection, &mut destination)
            .and_then(|backup| backup.run_to_completion(256, Duration::from_millis(10), None));
        if copied.is_err() {
            drop(destination);
            let _ = fs::remove_file(&target);
            return Err(ApplicationError::Storage);
        }
        drop(destination);
        let metadata = io_error(fs::metadata(&target))?;
        self.record_file_action("created", &name)?;
        Ok(BackupInfo {
            name,
            size: metadata.len(),
            modified: stamp,
        })
    }
    pub fn backups(directory: &Path) -> Result<Vec<BackupInfo>, ApplicationError> {
        if !directory.exists() {
            return Ok(vec![]);
        }
        let mut items = vec![];
        for entry in io_error(fs::read_dir(directory))? {
            let entry = io_error(entry)?;
            let name = entry.file_name().to_string_lossy().to_string();
            if backup_name(&name).is_err() || !name.starts_with("librett-") {
                continue;
            }
            let metadata = io_error(entry.metadata())?;
            if !metadata.is_file()
                || entry
                    .file_type()
                    .map_err(|_| ApplicationError::Storage)?
                    .is_symlink()
            {
                continue;
            }
            items.push(BackupInfo {
                name,
                size: metadata.len(),
                modified: metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map_or(0, |t| t.as_secs()),
            });
        }
        items.sort_by(|a, b| {
            b.modified
                .cmp(&a.modified)
                .then_with(|| b.name.cmp(&a.name))
        });
        Ok(items)
    }
    pub fn automatic_backup(&self, directory: &Path) -> Result<(), ApplicationError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ApplicationError::Storage)?
            .as_secs();
        let items = Self::backups(directory)?;
        if !items
            .iter()
            .any(|b| b.name.starts_with("librett-auto-") && now.saturating_sub(b.modified) < 86400)
        {
            self.create_backup(directory, "auto")?;
        }
        let items = Self::backups(directory)?;
        for old in items
            .iter()
            .filter(|b| b.name.starts_with("librett-auto-"))
            .skip(14)
        {
            io_error(fs::remove_file(directory.join(&old.name)))?;
        }
        Ok(())
    }
    pub fn backup_path(directory: &Path, name: &str) -> Result<PathBuf, ApplicationError> {
        backup_name(name)?;
        let path = directory.join(name);
        if !name.starts_with("librett-")
            || !path.is_file()
            || io_error(fs::symlink_metadata(&path))?
                .file_type()
                .is_symlink()
        {
            return Err(ApplicationError::InvalidBackup);
        }
        Ok(path)
    }
    pub fn restore_backup(&mut self, directory: &Path, name: &str) -> Result<(), ApplicationError> {
        let path = Self::backup_path(directory, name)?;
        self.restore_path(directory, &path)
    }
    pub fn import_backup(
        &mut self,
        directory: &Path,
        bytes: &[u8],
    ) -> Result<(), ApplicationError> {
        if bytes.len() > 256 * 1024 * 1024 || !bytes.starts_with(b"SQLite format 3\0") {
            return Err(ApplicationError::InvalidBackup);
        }
        io_error(fs::create_dir_all(directory))?;
        let path = directory.join(format!("import-{}.sqlite", Uuid::new_v4()));
        io_error(fs::write(&path, bytes))?;
        let result = self.restore_path(directory, &path);
        let _ = fs::remove_file(&path);
        result
    }
    fn restore_path(&mut self, directory: &Path, path: &Path) -> Result<(), ApplicationError> {
        // Validate a read-only source before creating a writable staging copy.
        let source = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|_| ApplicationError::InvalidBackup)?;
        source
            .execute_batch("PRAGMA trusted_schema=OFF;")
            .map_err(|_| ApplicationError::InvalidBackup)?;
        let version: i64 = source
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(|_| ApplicationError::InvalidBackup)?;
        let app: i64 = source
            .query_row("PRAGMA application_id", [], |r| r.get(0))
            .map_err(|_| ApplicationError::InvalidBackup)?;
        let integrity: String = source
            .query_row("PRAGMA quick_check", [], |r| r.get(0))
            .map_err(|_| ApplicationError::InvalidBackup)?;
        if !(1..=22).contains(&version) || ![0, 1279415380].contains(&app) || integrity != "ok" {
            return Err(ApplicationError::InvalidBackup);
        }
        let recognizable:bool=source.query_row("SELECT EXISTS(SELECT 1 FROM pragma_table_info('tournaments') WHERE name='name') AND EXISTS(SELECT 1 FROM pragma_table_info('categories') WHERE name='tournament_id')",[],|r|r.get(0)).map_err(|_|ApplicationError::InvalidBackup)?;
        if !recognizable {
            return Err(ApplicationError::InvalidBackup);
        }
        let stage = directory.join(format!("stage-{}.sqlite", Uuid::new_v4()));
        let result = (|| {
            let mut staged = Connection::open(&stage).map_err(|_| ApplicationError::Storage)?;
            staged
                .execute_batch("PRAGMA trusted_schema=OFF;")
                .map_err(|_| ApplicationError::InvalidBackup)?;
            rusqlite::backup::Backup::new(&source, &mut staged)
                .and_then(|b| b.run_to_completion(256, Duration::from_millis(10), None))
                .map_err(|_| ApplicationError::Storage)?;
            let repository =
                Self::initialize(staged).map_err(|_| ApplicationError::InvalidBackup)?;
            let reference = Self::initialize(
                Connection::open_in_memory().map_err(|_| ApplicationError::Storage)?,
            )
            .map_err(|_| ApplicationError::Storage)?;
            if schema(&repository.connection).map_err(|_| ApplicationError::InvalidBackup)?
                != schema(&reference.connection).map_err(|_| ApplicationError::Storage)?
            {
                return Err(ApplicationError::InvalidBackup);
            }
            let violations: bool = repository
                .connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM pragma_foreign_key_check)",
                    [],
                    |r| r.get(0),
                )
                .map_err(|_| ApplicationError::InvalidBackup)?;
            if violations {
                return Err(ApplicationError::InvalidBackup);
            }
            repository
                .list()
                .map_err(|_| ApplicationError::InvalidBackup)?;
            self.create_backup(directory, "before-restore")?;
            rusqlite::backup::Backup::new(&repository.connection, &mut self.connection)
                .and_then(|b| b.run_to_completion(256, Duration::from_millis(10), None))
                .map_err(|_| ApplicationError::Storage)?;
            self.connection
                .execute_batch("PRAGMA foreign_keys=ON;")
                .map_err(|_| ApplicationError::Storage)?;
            self.record_file_action(
                "restored",
                &path.file_name().unwrap_or_default().to_string_lossy(),
            )?;
            Ok(())
        })();
        let _ = fs::remove_file(stage);
        result
    }
}
