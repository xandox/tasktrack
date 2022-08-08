mod cli;
mod database;
mod error;
mod time_ranges;
use database::{ActivationStatus, Database};

use cli::*;

use clap::Parser;

type CmdResult = error::Result<()>;

fn main() -> error::Result<()> {
    let cli = CLI::parse();
    let db = Database::open()?;

    match cli.command {
        Command::Current => current_task(&db)?,
        Command::List(args) => list_tasks(&db, args)?,
        Command::New(args) => new_task(&db, args)?,
        Command::Edit(args) => edit_task(&db, args)?,
        Command::Show(args) => show_task(&db, args)?,
        Command::Activate(args) => activate_task(db, args)?,
        _ => (),
    }
    /*
    match cli.command {
        Command::ListTasks => list_tasks(&db)?,
        Command::AddTask(args) => add_task(&db, args)?,
        Command::ActivateTask(args) => activate_task(&db, args)?,
        Command::ActiveTask => active_task(&db)?,
        Command::DeactivateTask => deactivate_task(&db)?,
    }
    */
    Ok(())
}

fn activate_task(mut db: Database, args: ActivateArgs) -> CmdResult {
    if !db.is_task_exist(&args.task_id)? {
        println!("*** Task with id {} does not exist. ***", args.task_id);
        return Ok(());
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
    Ok(())
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
            Ok(())
        }
        Err(error::Error::SQL(err)) => {
            if is_primary_key_error(&err) {
                println!("*** Task with id {} already exists. ***", args.task_id);
                Ok(())
            } else {
                Err(error::Error::SQL(err))
            }
        }
        Err(err) => Err(err),
    }
}

fn edit_task(db: &Database, args: NewArgs) -> CmdResult {
    if !db.update_task(
        &args.task_id,
        args.url.as_ref().map(|s| s.as_str()),
        args.title.as_ref().map(|s| s.as_str()),
        args.workpackage.as_ref().map(|s| s.as_str()),
        args.objective.as_ref().map(|s| s.as_str()),
    )? {
        println!(
            "*** Task with id {} has not updated. Probably it does not exist. ***",
            args.task_id
        );
    } else {
        println!("Task with id {} has updated", args.task_id);
    }
    Ok(())
}

fn show_task(db: &Database, args: ShowArgs) -> CmdResult {
    let task = db.get_task(&args.task_id)?;
    let none = "None".to_owned();
    println!("Task: {}", task.task_id);
    println!("\tTitle: {}", task.title.as_ref().unwrap_or(&none));
    println!("\tURL: {}", task.url.as_ref().unwrap_or(&none));
    println!(
        "\tWorkpackage: {}",
        task.workpackage.as_ref().unwrap_or(&none)
    );
    println!("\tObjective: {}", task.objective.as_ref().unwrap_or(&none));
    Ok(())
}

fn list_tasks(db: &Database, args: ListArgs) -> CmdResult {
    let tasks = db.list_tasks(args.num_tasks)?;
    if tasks.is_empty() {
        println!("*** No task created yet ***");
    } else {
        for task_id in tasks {
            println!("{}", task_id);
        }
    }
    Ok(())
}

fn current_task(db: &Database) -> CmdResult {
    if let Some(task_id) = db.get_current_task_id()? {
        println!("Current task: {}", task_id);
    } else {
        println!("*** No current task ***");
    }
    Ok(())
}

/*
fn list_tasks(db: &Database) -> error::Result<()> {
    let tasks = db.get_all_tasks()?;
    if tasks.is_empty() {
        println!("*** No tasks found ***\nTry add some with add-task.")
    } else {
        for task in tasks.iter() {
            println!("{}", task.task_id);
        }
    }
    Ok(())
}

fn add_task(db: &Database, args: AddTask) -> error::Result<()> {
    if db.is_task_exist(&args.task_id)? {
        println!("Task already exists: {}.", args.task_id);
        return Ok(());
    }
    db.insert_new_task(&args.task_id)?;
    println!("ok");
    Ok(())
}

fn active_task(db: &Database) -> error::Result<()> {
    if let Some(task_id) = db.get_active_task()? {
        println!("Current task is {}.", task_id);
    } else {
        println!("*** No active task ***");
    }
    Ok(())
}

fn deactivate_task(db: &Database) -> error::Result<()> {
    db.deactivete_current_task()?;
    println!("Ok");
    Ok(())
}

fn activate_task(db: &Database, args: ActivateTask) -> error::Result<()> {
    if !db.is_task_exist(&args.task_id)? {
        println!("Unknown task: {}.\nAdd it first.", args.task_id);
        return Ok(());
    }
    let active_task = db.get_active_task()?;
    if let Some(task_id) = active_task {
        if task_id == args.task_id {
            println!("Already active.");
            return Ok(());
        } else {
            db.deactivete_current_task()?;
            println!("Deactivating task: {}.", task_id);
        }
    }

    db.set_active_task(&args.task_id)?;
    println!("Activated.");

    Ok(())
}
*/
