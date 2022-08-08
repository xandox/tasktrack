use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::OptionalExtension;
use std::os::unix::fs::chroot;
use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use crate::time_ranges::{now_str, TimeRange};

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

    pub fn get_task(&self, task_id: &str) -> Result<Task> {
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

    pub fn update_task(
        &self,
        task_id: &str,
        url: Option<&str>,
        title: Option<&str>,
        wp: Option<&str>,
        o: Option<&str>,
    ) -> Result<bool> {
        const SQL: &'static str = "
            UPDATE Task SET 
                url = :url,
                title = :title,
                workpackage = :workpackage,
                objective = :objective,
                last_update = :now 
            WHERE task_id = :task_id;
        ";

        let mut stmt = self.connection.prepare(SQL)?;
        let nrows = stmt.execute(named_params! {
        ":task_id": task_id,
        ":url": url,
        ":title": title,
        ":workpackage": wp,
        ":objective": o,
        ":now": now_str()})?;
        Ok(nrows == 1)
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

    pub fn task_time_ranges(&self, task_id: &str) -> Result<Vec<TimeRange>> {
        const SQL: &'static str = "
            SELECT timestamp, start_or_stop FROM TaskTimeRanges WHERE task_id = :task_id;
        ";
        let mut stmt = self.connection.prepare(SQL)?;
        let row_iter = stmt.query_map(named_params! {":task_id": task_id}, |r| {
            Ok((r.get(0)?, r.get(1)?))
        })?;

        let mut ranges = Vec::new();

        for row in row_iter {
            let (dt, sos): (String, i64) = row?;
            let datetime = chrono::DateTime::parse_from_rfc3339(&dt)
                .unwrap()
                .with_timezone(&chrono::Utc);

            if sos == START_VALUE {
                ranges.push(TimeRange {
                    start: Some(datetime),
                    end: None,
                })
            } else {
                ranges.last_mut().unwrap().end = Some(datetime);
            }
        }

        Ok(ranges)
    }
}
