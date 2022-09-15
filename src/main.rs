mod cli;
mod database;
mod error;
mod time_ranges;

use num_traits::FromPrimitive;
use std::collections::HashMap;

use database::{ActivationStatus, Database};

use cli::*;

use clap::Parser;

type CmdResult = error::Result<i32>;

fn main() -> error::Result<()> {
    let cli = CLI::parse();
    let db = Database::open()?;

    let return_code = match cli.command {
        Command::Current => current_task(&db)?,
        Command::List(args) => list_tasks(&db, args)?,
        Command::New(args) => new_task(&db, args)?,
        Command::Edit(args) => edit_task(&db, args)?,
        Command::Show(args) => show_task(&db, args)?,
        Command::Activate(args) => activate_task(db, args)?,
        Command::Report(args) => report(&db, args)?,
        Command::AddRange(args) => add_range(db, args)?,
        Command::VacationAdd(args) => add_vacation(db, args)?,
        Command::VacationRemove(args) => remove_vacation(db, args)?,
        Command::VacationList(args) => list_vacations(db, args)?,
    };
    std::process::exit(return_code);
}

fn add_vacation(db: Database, args: VacationAddArgs) -> CmdResult {
    let since = args.since.start_datetime();
    let till = args.till.end_datetime();
    db.add_vacation(since, till)?;
    Ok(0)
}

fn remove_vacation(db: Database, args: VacationRemoveArgs) -> CmdResult {
    db.delete_vacation(args.id)?;
    Ok(0)
}

fn list_vacations(db: Database, args: VacationListArgs) -> CmdResult {
    use prettytable::{format::FormatBuilder, Cell, Row, Table};
    let since = args
        .since
        .map(|d| d.start_datetime())
        .unwrap_or(time_ranges::from_timestamp(0));
    let till = args
        .till
        .map(|d| d.end_datetime())
        .unwrap_or(time_ranges::now());
    let vacations = db.list_vacations(since, till)?;
    if vacations.is_empty() {
        if args.since.is_some() || args.till.is_some() {
            println!("*** No vacations found in specified period");
        } else {
            println!("*** No vacations found");
        }
        return Ok(1);
    } else {
        let format = FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .padding(0, 0)
            .build();

        let mut table = Table::new();
        table.set_format(format);

        for vacation in vacations {
            let mut row = Vec::new();
            row.push(Cell::new(&vacation.0.to_string()));
            row.push(Cell::new(&vacation.1.date().format("%d.%m.%Y").to_string()));
            row.push(Cell::new(&vacation.2.date().format("%d.%m.%Y").to_string()));
            table.add_row(Row::new(row));
        }

        table.printstd();
    }
    Ok(0)
}

fn add_range(mut db: Database, args: AddRangeArgs) -> CmdResult {
    if db.get_task(&args.task_id)?.is_none() {
        println!("*** No task with id {}. ***", args.task_id);
        return Ok(1);
    }
    if let Some(since) = args.since {
        db.update_time_ranges(
            &args.task_id,
            database::START_VALUE,
            Some(since.start_datetime()),
        )?;
        println!("Add start point to task with id {}.", args.task_id);
    }
    if let Some(till) = args.till {
        db.update_time_ranges(
            &args.task_id,
            database::STOP_VALUE,
            Some(till.end_datetime()),
        )?;
        println!("Add end point to task with id {}.", args.task_id);
    }
    Ok(0)
}

struct TaskReport {
    task_id: String,
    total_hours: f64,
    month_range: (chrono::Month, chrono::Month),
    month_hours: HashMap<chrono::Month, f64>,
}

fn report(db: &Database, args: ReportArgs) -> CmdResult {
    let since = args.since.start_datetime();
    let till = args.till.end_datetime();
    let ranges = db.select_time_ranges(None, Some(since), Some(till))?;
    let mut reports = Vec::new();
    let calendar = get_calendar(db)?;
    for (task_id, task_ranges) in ranges.iter() {
        let task_id = task_id.to_owned();
        let total_hours = time_ranges::working_houres_from_ranges(
            task_ranges,
            Some(since),
            Some(till),
            &calendar,
        );
        let month_hours = time_ranges::month_hours(task_ranges, Some(since), Some(till), &calendar);
        let months_vec: Vec<u32> = month_hours
            .iter()
            .map(|(k, _)| *k)
            .map(|m| m.number_from_month())
            .collect();
        let month_range = (
            *months_vec.iter().min().unwrap(),
            *months_vec.iter().max().unwrap(),
        );

        let month_range = (
            chrono::Month::from_u32(month_range.0).unwrap(),
            chrono::Month::from_u32(month_range.1).unwrap(),
        );
        reports.push(TaskReport {
            task_id: task_id,
            total_hours: total_hours,
            month_range: month_range,
            month_hours: month_hours,
        });
    }

    reports.sort_by(|a, b| a.task_id.cmp(&b.task_id));

    use prettytable::{cell, format::FormatBuilder, Cell, Row, Table};

    let format = FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .padding(0, 0)
        .build();

    let month = time_ranges::month_range(since, till);

    let none = "None".to_owned();
    let mut table = Table::new();
    table.set_format(format);
    let mut header: Vec<String> = vec![
        "Title",
        "URL",
        "Total hours",
        "Workpackage",
        "Objective",
        "Month range",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    header.extend(month.iter().map(|m| m.name()[..3].to_string()));
    table.add_row(Row::new(
        header.into_iter().map(|r| Cell::new(&r)).collect(),
    ));
    for tr in reports.iter() {
        let task = db.get_task(&tr.task_id)?;
        let task = match task {
            Some(task) => task,
            None => database::Task {
                task_id: tr.task_id.clone(),
                url: None,
                title: None,
                workpackage: None,
                objective: None,
            },
        };
        let mut row = Vec::new();
        row.push(cell!(format!(
            "[{}]{}",
            task.task_id,
            task.title.unwrap_or_else(|| none.clone())
        )));
        row.push(cell!(task.url.unwrap_or_else(|| none.clone())));
        row.push(cell!(format!("{:.2}", tr.total_hours)));
        row.push(cell!(task.workpackage.unwrap_or_else(|| none.clone())));
        row.push(cell!(task.objective.unwrap_or_else(|| none.clone())));
        let month_range = if tr.month_range.0 == tr.month_range.1 {
            tr.month_range.0.name()[..3].to_string()
        } else {
            format!(
                "{}-{}",
                tr.month_range.0.name()[..3].to_string(),
                tr.month_range.1.name()[..3].to_string()
            )
        };
        row.push(cell!(month_range));
        for m in month.iter() {
            if tr.month_hours.contains_key(m) {
                row.push(cell!(format!("{:.2}", tr.month_hours[m])));
            } else {
                row.push(cell!(""));
            }
        }
        table.add_row(Row::new(row));
    }
    table.printstd();

    Ok(0)
}

fn activate_task(mut db: Database, args: ActivateArgs) -> CmdResult {
    if !db.is_task_exist(&args.task_id)? {
        println!("*** Task with id {} does not exist. ***", args.task_id);
        return Ok(1);
    }
    let r = db.activate_task(&args.task_id)?;
    match r {
        ActivationStatus::AlreadyActive => println!("Task with id {} already active", args.task_id),
        ActivationStatus::Activated => println!("Task with id {} has activated", args.task_id),
        ActivationStatus::Deactivated(old_task_id) => println!(
            "Task with id {} has deactivated. Task with id {} has activated.",
            old_task_id, args.task_id
        ),
    }
    Ok(0)
}

fn is_primary_key_error(error: &rusqlite::Error) -> bool {
    use rusqlite::ffi::{Error, ErrorCode};
    const PRIMARY_KEY_ERROR: Error = Error {
        code: ErrorCode::ConstraintViolation,
        extended_code: 1555,
    };
    match error {
        rusqlite::Error::SqliteFailure(PRIMARY_KEY_ERROR, _) => true,
        _ => false,
    }
}

fn new_task(db: &Database, args: NewArgs) -> CmdResult {
    match db.new_task(
        &args.task_id,
        args.url.as_ref().map(|s| s.as_str()),
        args.title.as_ref().map(|s| s.as_str()),
        args.workpackage.as_ref().map(|s| s.as_str()),
        args.objective.as_ref().map(|s| s.as_str()),
    ) {
        Ok(()) => {
            println!("New task with id {} has created.", args.task_id);
            Ok(0)
        }
        Err(error::Error::SQL(err)) => {
            if is_primary_key_error(&err) {
                println!("*** Task with id {} already exists. ***", args.task_id);
                Ok(1)
            } else {
                Err(error::Error::SQL(err))
            }
        }
        Err(err) => Err(err),
    }
}

fn edit_task(db: &Database, args: EditArgs) -> CmdResult {
    let (found, was_fields) = db.update_task(
        &args.task_id,
        args.url.as_ref().map(|s| s.as_str()),
        args.title.as_ref().map(|s| s.as_str()),
        args.workpackage.as_ref().map(|s| s.as_str()),
        args.objective.as_ref().map(|s| s.as_str()),
        args.drop_url,
        args.drop_title,
        args.drop_workpackage,
        args.drop_objective,
    )?;

    if !was_fields {
        println!("*** Not values for update ***");
        return Ok(2);
    } else if !found {
        println!(
            "*** Task with id {} has not updated. Probably it does not exist. ***",
            args.task_id
        );
        return Ok(1);
    } else {
        println!("Task with id {} has updated", args.task_id);
        return show_task(
            db,
            ShowArgs {
                task_id: args.task_id,
            },
        );
    }
}

fn show_task(db: &Database, args: ShowArgs) -> CmdResult {
    let task = db.get_task(&args.task_id)?;
    match task {
        None => {
            println!("*** No task found with id {}. ***", args.task_id);
            return Ok(1);
        }
        Some(task) => {
            let none = "None".to_owned();
            println!("Task: {}", task.task_id);
            println!("\tTitle: {}", task.title.as_ref().unwrap_or(&none));
            println!("\tURL: {}", task.url.as_ref().unwrap_or(&none));
            println!(
                "\tWorkpackage: {}",
                task.workpackage.as_ref().unwrap_or(&none)
            );
            println!("\tObjective: {}", task.objective.as_ref().unwrap_or(&none));
            Ok(0)
        }
    }
}

fn list_tasks(db: &Database, args: ListArgs) -> CmdResult {
    use prettytable::{format::FormatBuilder, Cell, Row, Table};
    let current_task = db.get_current_task_id()?;
    let tasks = db.list_tasks(args.num_tasks)?;
    if tasks.is_empty() {
        println!("*** No task created yet ***");
        return Ok(1);
    } else {
        let format = FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .padding(0, 0)
            .build();
        let mut table = Table::new();
        table.set_format(format);

        for task in tasks {
            let mut row = Vec::new();
            if Some(&task.task_id) == current_task.as_ref() {
                row.push(Cell::new("*"));
            } else {
                row.push(Cell::new(""));
            }
            row.push(Cell::new(&task.task_id));
            if let Some(title) = task.title {
                row.push(Cell::new(&title));
            } else {
                row.push(Cell::new(""));
            }
            table.add_row(Row::new(row));
        }

        table.printstd();
        return Ok(0);
    }
}

fn get_calendar(
    db: &Database,
) -> error::Result<impl bdays::HolidayCalendar<time_ranges::DateTime>> {
    let vacations = db.get_vacations()?;
    Ok(time_ranges::CalendarCombination::holydays_and_vacations(
        vacations,
    ))
}

fn current_task(db: &Database) -> CmdResult {
    if let Some(task_id) = db.get_current_task_id()? {
        let time_ranges_map = db.select_time_ranges(Some(&task_id), None, None)?;
        let time_ranges = time_ranges_map.get(&task_id);
        let working_houers = match time_ranges {
            None => 0.0,
            Some(ranges) => {
                time_ranges::working_houres_from_ranges(ranges, None, None, &get_calendar(db)?)
            }
        };
        println!(
            "Current task: {}. You are working on it for {:.4} hours",
            task_id, working_houers
        );
        return Ok(0);
    } else {
        println!("*** No current task ***");
        return Ok(1);
    }
}
