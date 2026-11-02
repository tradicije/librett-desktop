use librett_storage_sqlite::SqliteTournamentRepository;
use std::{collections::HashMap, sync::Mutex};

pub(crate) struct Database(pub(crate) Mutex<SqliteTournamentRepository>);

pub(crate) struct PrintReports(pub(crate) Mutex<HashMap<String, String>>);
