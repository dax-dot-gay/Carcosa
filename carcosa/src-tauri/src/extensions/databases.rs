use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use getset::CloneGetters;
use parking_lot::RwLock;
use redb::{
    Key, MultimapTable, MultimapTableDefinition, MultimapTableHandle, ReadOnlyMultimapTable,
    ReadOnlyTable, ReadableDatabase, Table, TableDefinition, TableHandle, Value,
};
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum TableName {
    Unique { name: String },
    Multimap { name: String },
}

impl From<TableName> for String {
    fn from(value: TableName) -> Self {
        match value {
            TableName::Unique { name } => format!("unique:{name}"),
            TableName::Multimap { name } => format!("multimap:{name}"),
        }
    }
}

impl TryFrom<String> for TableName {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some((kind, name)) = value.split_once(":") {
            match kind {
                "unique" => Ok(Self::Unique {
                    name: name.to_string(),
                }),
                "multimap" => Ok(Self::Multimap {
                    name: value.to_string(),
                }),
                other => Err(crate::Error::TableKind(other.to_string())),
            }
        } else {
            Ok(Self::Unique { name: value })
        }
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self.clone()).as_str())
    }
}

impl TableName {
    pub fn unique(name: impl Into<String>) -> Self {
        Self::Unique { name: name.into() }
    }

    pub fn multimap(name: impl Into<String>) -> Self {
        Self::Multimap { name: name.into() }
    }

    pub fn name(&self) -> String {
        match self.clone() {
            TableName::Unique { name } => name,
            TableName::Multimap { name } => name,
        }
    }

    pub fn is_unique(&self) -> bool {
        if let Self::Unique { .. } = self.clone() {
            true
        } else {
            false
        }
    }

    pub fn is_multimap(&self) -> bool {
        if let Self::Multimap { .. } = self.clone() {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, CloneGetters)]
pub struct Database {
    #[getset(get_clone = "pub")]
    name: String,

    #[getset(get_clone = "pub")]
    path: PathBuf,

    database: Arc<RwLock<redb::Database>>,
}

impl Database {
    pub fn open(name: impl Into<String>, path: impl AsRef<Path>) -> crate::Result<Self> {
        let name = name.into();
        let path = path.as_ref().to_path_buf();
        let db = redb::Database::create(path.clone())?;
        Ok(Self {
            name,
            path,
            database: Arc::new(RwLock::new(db)),
        })
    }

    pub fn list_tables(&self) -> crate::Result<Vec<TableName>> {
        let lock = self.database.read();
        let txn = lock.begin_read()?;
        let mut result = Vec::new();
        result.extend(
            txn.list_tables()?
                .map(|v| TableName::unique(v.name().to_string())),
        );
        result.extend(
            txn.list_multimap_tables()?
                .map(|v| TableName::multimap(v.name().to_string())),
        );
        txn.close()?;

        Ok(result)
    }

    pub fn read<K: Key + 'static, V: Value + 'static, Output>(
        &self,
        table: impl Into<String>,
        transaction: impl FnOnce(ReadOnlyTable<K, V>) -> Output,
    ) -> crate::Result<Output> {
        let lock = self.database.read();
        let txn = lock.begin_read()?;
        let table = txn.open_table::<K, V>(TableDefinition::new(table.into().as_str()))?;
        let result = transaction(table);
        txn.close()?;
        Ok(result)
    }

    pub fn read_multimap<K: Key + 'static, V: Key + 'static, Output>(
        &self,
        table: impl Into<String>,
        transaction: impl FnOnce(ReadOnlyMultimapTable<K, V>) -> Output,
    ) -> crate::Result<Output> {
        let lock = self.database.read();
        let txn = lock.begin_read()?;
        let table =
            txn.open_multimap_table::<K, V>(MultimapTableDefinition::new(table.into().as_str()))?;
        let result = transaction(table);
        txn.close()?;
        Ok(result)
    }

    pub fn write<K: Key + 'static, V: Value + 'static, Output, Error: std::error::Error>(
        &self,
        table: impl Into<String>,
        transaction: impl FnOnce(Table<K, V>) -> Result<Output, Error>,
    ) -> crate::Result<Result<Output, Error>> {
        let lock = self.database.write();
        let txn = lock.begin_write()?;
        let table = txn.open_table::<K, V>(TableDefinition::new(table.into().as_str()))?;
        match transaction(table) {
            Ok(out) => {
                txn.commit()?;
                Ok(Ok(out))
            }
            Err(err) => {
                txn.abort()?;
                Ok(Err(err))
            }
        }
    }

    pub fn write_multimap<K: Key + 'static, V: Key + 'static, Output, Error: std::error::Error>(
        &self,
        table: impl Into<String>,
        transaction: impl FnOnce(MultimapTable<K, V>) -> Result<Output, Error>,
    ) -> crate::Result<Result<Output, Error>> {
        let lock = self.database.write();
        let txn = lock.begin_write()?;
        let table =
            txn.open_multimap_table::<K, V>(MultimapTableDefinition::new(table.into().as_str()))?;
        match transaction(table) {
            Ok(out) => {
                txn.commit()?;
                Ok(Ok(out))
            }
            Err(err) => {
                txn.abort()?;
                Ok(Err(err))
            }
        }
    }
}

pub type DbState = Arc<RwLock<HashMap<String, Database>>>;

pub trait DatabasesExt<R: Runtime> {
    fn database_state(&self) -> DbState;
    fn open_database(
        &self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> crate::Result<Database>;
    fn get_database(&self, name: impl Into<String>) -> Option<Database>;
    fn close_database(&self, name: impl Into<String>) -> ();
    fn clear_databases(&self) -> ();
    fn list_databases(&self) -> Vec<String>;
}

impl<R: Runtime, T: Manager<R>> DatabasesExt<R> for T {
    fn database_state(&self) -> Arc<RwLock<HashMap<String, Database>>> {
        if let Some(existing) = self.try_state::<DbState>() {
            existing.inner().clone()
        } else {
            self.manage::<DbState>(Arc::new(RwLock::new(HashMap::new())));
            self.state::<DbState>().inner().clone()
        }
    }
    fn open_database(
        &self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> crate::Result<Database> {
        let name = name.into();
        let path = path.as_ref().to_path_buf();
        let state = self.database_state();
        let mut registry = state.write();
        if let Some(existing) = registry.get(&name).cloned() {
            Ok(existing)
        } else {
            let opened = Database::open(name.clone(), path.clone())?;
            let _ = registry.insert(name, opened.clone());
            Ok(opened)
        }
    }

    fn get_database(&self, name: impl Into<String>) -> Option<Database> {
        let name = name.into();
        let state = self.database_state();
        let registry = state.read();
        registry.get(&name).cloned()
    }

    fn close_database(&self, name: impl Into<String>) -> () {
        let name = name.into();
        let state = self.database_state();
        let mut registry = state.write();
        let _ = registry.remove(&name);
    }

    fn clear_databases(&self) -> () {
        let state = self.database_state();
        let mut registry = state.write();
        registry.clear();
    }

    fn list_databases(&self) -> Vec<String> {
        let state = self.database_state();
        let state_map = state.read();
        state_map.keys().cloned().collect()
    }
}
