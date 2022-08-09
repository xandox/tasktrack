use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::OptionalExtension;
use rusqlite::ToSql;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use crate::time_ranges::{dt_to_str, now_str, DateTime, TimeRange};

pub(crate) struct Database {
    connection: Connection,
}

static START_VALUE: i64 = 1;
static STOP_VALUE: i64 = 0;

static CREATE_TASK_TABLE: &'static str = "
CREATE TABLE IF NOT EXISTS Task (
    task_id TEXT PRIMARY KEY,
    url TEXT,
    title TEXT,
    workpackage TEXT,
    objective TEXT,
    last_update TEXT NOT NULL
);
";

static CREATE_TASK_TIME_RANGES: &'static str = "
CREATE TABLE IF NOT EXISTS TaskTimeRanges (
    task_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    start_or_stop INTEGER NOT NULL,
    FOREIGN KEY(task_id) REFERENCES Task(task_id)
);
CREATE INDEX IF NOT EXISTS RangesTimestamp ON TaskTimeRanges (
    timestamp
);
";

pub enum ActivationStatus {
    AlreadyActive,
    Activated,
    Deactivated(String),
}

pub struct Task {
    pub task_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub workpackage: Option<String>,
    pub objective: Option<String>,
}

struct StrToSql {
    value: String,
}

impl StrToSql {
    fn new(v: String) -> Self {
        StrToSql { value: v }
    }
}

impl rusqlite::ToSql for StrToSql {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.value.to_sql()
    }
}

impl Database {
    pub fn open() -> Result<Self> {
        let filename = Self::get_db_file();
        let directory = filename.parent().unwrap();
        std::fs::create_dir_all(directory)?;
        let connection = Connection::open(filename)?;

        connection.execute("PRAGMA foreign_keys = ON;", ())?;
        connection.execute(CREATE_TASK_TABLE, ())?;
        connection.execute(CREATE_TASK_TIME_RANGES, ())?;

        Ok(Self {
            connection: connection,
        })
    }

    fn get_db_file() -> PathBuf {
        let data_dir = dirs::data_local_dir().unwrap();
        let company_name = "xsoft";
        let application = "tasktrack";
        let filename = "data.db";

        data_dir.join(company_name).join(application).join(filename)
    }

    pub fn get_current_task_id(&self) -> Result<Option<String>> {
        const SQL: &'static str = "
            SELECT task_id, timestamp, start_or_stop FROM TaskTimeRanges WHERE timestamp = (SELECT MAX(timestamp) FROM TaskTimeRanges);
        ";
        let mut stmp = self.connection.prepare(SQL)?;
        let row: Option<(String, i64)> = stmp
            .query_row((), |r| {
                let task_id: String = r.get(0)?;
                let start_or_stop: i64 = r.get(2)?;
                Ok((task_id, start_or_stop))
            })
            .optional()?;

        if let Some((task_id, start_or_stop)) = row {
            if start_or_stop == START_VALUE {
                return Ok(Some(task_id));
            }
        }

        Ok(None)
    }

    pub fn list_tasks(&self, top_n: Option<usize>) -> Result<Vec<String>> {
        const SQL_NO_LIMIT: &'static str = "
            SELECT task_id FROM Task ORDER BY last_update DESC;
        ";
        const SQL_LIMIT: &'static str = "
            SELECT task_id FROM Task ORDER BY last_update DESC LIMIT :top_n;
        ";

        let mut stmt = self.connection.prepare(if top_n.is_some() {
            SQL_LIMIT
        } else {
            SQL_NO_LIMIT
        })?;

        let row_map = |r: &rusqlite::Row| r.get(0);

        let res_iter = if let Some(top_n) = top_n {
            stmt.query_map(named_params! {":top_n": top_n}, row_map)?
        } else {
            stmt.query_map((), row_map)?
        };

        let mut result = Vec::new();
        for task_id in res_iter {
            result.push(task_id?);
        }
        Ok(result)
    }

    pub fn get_task(&self, task_id: &str) -> Result<Option<Task>> {
        const SQL: &'static str = "
            SELECT task_id, url, title, workpackage, objective FROM Task WHERE task_id = :task_id;
        ";
        let mut stmt = self.connection.prepare(SQL)?;
        stmt.query_row(named_params! {":task_id": task_id}, |r| {
            Ok(Task {
                task_id: r.get(0)?,
                url: r.get(1)?,
                title: r.get(2)?,
                workpackage: r.get(3)?,
                objective: r.get(4)?,
            })
        })
        .optional()
        .map_err(|e| e.into())
    }

    pub fn new_task(
        &self,
        task_id: &str,
        url: Option<&str>,
        title: Option<&str>,
        wp: Option<&str>,
        o: Option<&str>,
    ) -> Result<()> {
        const SQL: &'static str = "
        INSERT INTO Task (task_id, url, title, workpackage, objective, last_update)
        VALUES (:task_id, :url, :title, :workpackage, :objective, :now);
        ";

        let mut stmt = self.connection.prepare(SQL)?;
        stmt.insert(named_params! {
        ":task_id": task_id,
        ":url": url,
        ":title": title,
        ":workpackage": wp,
        ":objective": o,
        ":now": now_str()})
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn update_task_field(
        &self,
        task_id: &str,
        field: &str,
        value: Option<&str>,
        drop: bool,
    ) -> Result<(bool, bool)> {
        if value.is_none() && !drop {
            return Ok((false, false));
        }

        let query = format!(
            "UPDATE Task SET {} = ?, last_update = ? WHERE task_id = ?",
            field
        );
        let mut stmt = self.connection.prepare(&query)?;
        let nrows = if drop {
            stmt.execute([
                &rusqlite::types::Null as &dyn ToSql,
                &StrToSql::new(now_str()) as &dyn ToSql,
                &StrToSql::new(task_id.to_string()) as &dyn ToSql,
            ])?
        } else {
            stmt.execute([
                &StrToSql::new(value.unwrap().to_string()) as &dyn ToSql,
                &StrToSql::new(now_str()) as &dyn ToSql,
                &StrToSql::new(task_id.to_string()) as &dyn ToSql,
            ])?
        };

        Ok((nrows == 1, true))
    }

    pub fn update_task(
        &self,
        task_id: &str,
        url: Option<&str>,
        title: Option<&str>,
        wp: Option<&str>,
        o: Option<&str>,
        drop_url: bool,
        drop_title: bool,
        drop_wp: bool,
        drop_o: bool,
    ) -> Result<(bool, bool)> {
        let (found_url, was_url) = self.update_task_field(task_id, "url", url, drop_url)?;
        let (found_title, was_title) =
            self.update_task_field(task_id, "title", title, drop_title)?;
        let (found_wp, was_wp) = self.update_task_field(task_id, "workpackage", wp, drop_wp)?;
        let (found_o, was_o) = self.update_task_field(task_id, "objective", o, drop_o)?;
        Ok((
            found_url || found_title || found_wp || found_o,
            was_url || was_title || was_wp | was_o,
        ))
    }

    pub fn is_task_exist(&self, task_id: &str) -> Result<bool> {
        const SQL: &'static str = "
            SELECT task_id FROM Task WHERE task_id = :task_id;
        ";
        let mut stmt = self.connection.prepare(SQL)?;
        let result: Option<String> = stmt
            .query_row(named_params! {":task_id": task_id}, |r| r.get(0))
            .optional()?;
        Ok(result.is_some())
    }

    fn update_time_ranges(&mut self, task_id: &str, value: i64) -> Result<()> {
        const SQL_T: &'static str = "
            UPDATE Task SET
                last_update = :now
            WHERE task_id = :task_id;
        ";

        const SQL_R: &'static str = "
            INSERT INTO TaskTimeRanges (task_id, timestamp, start_or_stop) 
                VALUES (:task_id, :now, :value);
        ";

        let ns = now_str();

        let tx = self.connection.transaction()?;

        let task_updated = { tx.execute(SQL_T, named_params! {":task_id": task_id, ":now": ns})? };

        {
            let mut ranges_stmt = tx.prepare(SQL_R)?;
            ranges_stmt.insert(named_params! {":task_id": task_id, ":value": value, ":now": ns})?;
        }

        if task_updated != 1 {
            return Err(Error::RangesUpdateError);
        }

        tx.commit()?;

        Ok(())
    }

    pub fn activate_task(&mut self, task_id: &str) -> Result<ActivationStatus> {
        let current_task = self.get_current_task_id()?;
        if let Some(c_task_id) = current_task.as_ref() {
            if c_task_id == task_id {
                return Ok(ActivationStatus::AlreadyActive);
            } else {
                self.update_time_ranges(&c_task_id, STOP_VALUE)?;
            }
        }
        self.update_time_ranges(task_id, START_VALUE)?;
        if let Some(c_task_id) = current_task {
            return Ok(ActivationStatus::Deactivated(c_task_id));
        } else {
            Ok(ActivationStatus::Activated)
        }
    }

    pub fn select_time_ranges(
        &self,
        task_id: Option<&str>,
        start_date: Option<DateTime>,
        end_date: Option<DateTime>,
    ) -> Result<HashMap<String, Vec<TimeRange>>> {
        const SQL_BASE: &'static str = "
            SELECT task_id, timestamp, start_or_stop FROM TaskTimeRanges
        ";
        let where_block = if task_id.is_some() || start_date.is_some() || end_date.is_some() {
            let mut blocks = Vec::with_capacity(3);
            if task_id.is_some() {
                blocks.push("task_id = :task_id".to_owned());
            };

            if start_date.is_some() {
                blocks.push("timestamp >= :start".to_owned());
            };

            if end_date.is_some() {
                blocks.push("timestamp <= :end".to_owned());
            };
            format!(" WHERE {}", blocks.join(" AND "))
        } else {
            "".to_owned()
        };

        let sql = format!("{}{};", SQL_BASE, where_block);
        let mut stmt = self.connection.prepare(&sql)?;
        let mut params = Vec::new();
        let start_date = start_date.map(|dt| StrToSql::new(dt_to_str(&dt)));
        let end_date = end_date.map(|dt| StrToSql::new(dt_to_str(&dt)));
        let task_id = task_id.map(|s| StrToSql::new(s.to_owned()));
        if let Some(task_id) = task_id.as_ref() {
            params.push((":task_id", task_id as &dyn ToSql));
        }
        if let Some(start_date) = start_date.as_ref() {
            params.push((":start", start_date as &dyn ToSql));
        }

        if let Some(end_date) = end_date.as_ref() {
            params.push((":end", end_date as &dyn ToSql));
        }
        let row_iter = stmt.query_map(params.as_slice(), |r| {
            let task_id = r.get(0)?;
            let timestamp = r.get(1)?;
            let start_or_stop = r.get(2)?;
            Ok((task_id, timestamp, start_or_stop))
        })?;

        let mut result = HashMap::new();

        for row in row_iter {
            let (task_id, timestamp, sos): (String, String, i64) = row?;
            let datetime = chrono::DateTime::parse_from_rfc3339(&timestamp)
                .unwrap()
                .with_timezone(&chrono::Utc);

            if !result.contains_key(&task_id) {
                result.insert(task_id.clone(), Vec::new());
            }

            let records = result.get_mut(&task_id).unwrap();

            if sos == START_VALUE {
                records.push(TimeRange {
                    start: Some(datetime),
                    end: None,
                });
            } else {
                match records.last_mut() {
                    Some(last) => last.end = Some(datetime),
                    None => records.push(TimeRange {
                        start: None,
                        end: Some(datetime),
                    }),
                }
            }
        }

        Ok(result)
    }
}
