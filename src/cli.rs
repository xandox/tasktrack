use chrono::{Date, DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, Copy)]
pub struct CliDate(pub Date<Utc>);

impl CliDate {
    pub fn start_datetime(&self) -> DateTime<Utc> {
        self.0.and_hms(0, 0, 0)
    }

    pub fn end_datetime(&self) -> DateTime<Utc> {
        self.0.and_hms(23, 59, 59)
    }
}

impl clap::builder::ValueParserFactory for CliDate {
    type Parser = CliDateValueParser;

    fn value_parser() -> Self::Parser {
        CliDateValueParser
    }
}

#[derive(Clone, Debug)]
pub struct CliDateValueParser;

impl clap::builder::TypedValueParser for CliDateValueParser {
    type Value = CliDate;
    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let str = value.to_string_lossy().into_owned();
        let result = NaiveDate::parse_from_str(&str, "%d.%m.%Y");
        match result {
            Ok(date) => Ok(CliDate(Date::from_utc(date, Utc))),
            Err(err) => Err(clap::Error::raw(clap::ErrorKind::Format, err)),
        }
    }
}

#[derive(clap::Parser)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    #[doc = "Show current active task"]
    Current,
    #[doc = "List tasks"]
    List(ListArgs),
    #[doc = "Create new task"]
    New(NewArgs),
    #[doc = "Activate task"]
    Activate(ActivateArgs),
    #[doc = "Edit task description"]
    Edit(EditArgs),
    #[doc = "Generate report"]
    Report(ReportArgs),
    #[doc = "Show task description"]
    Show(ShowArgs),
    #[doc = "Manulay add task time range"]
    AddRange(AddRangeArgs),
    #[doc = "Add vacation"]
    VacationAdd(VacationAddArgs),
    #[doc = "Remove vacation"]
    VacationRemove(VacationRemoveArgs),
    #[doc = "List vacations"]
    VacationList(VacationListArgs),
}

#[derive(clap::Parser)]
pub struct VacationListArgs {
    #[clap(short, long, value_parser)]
    pub since: Option<CliDate>,
    #[clap(short, long, value_parser)]
    pub till: Option<CliDate>,
}

#[derive(clap::Parser)]
pub struct VacationAddArgs {
    #[clap(short, long, value_parser)]
    pub since: CliDate,
    #[clap(short, long, value_parser)]
    pub till: CliDate,
}

#[derive(clap::Parser)]
pub struct VacationRemoveArgs {
    #[clap(short, long)]
    pub id: i64,
}

#[derive(clap::Parser)]
pub struct ListArgs {
    #[clap(short, long, value_parser, value_name = "INT")]
    #[doc = "If set first *num_tasks*"]
    pub num_tasks: Option<usize>,
}

#[derive(clap::Parser)]
pub struct ActivateArgs {
    #[clap(value_parser)]
    #[doc = "Task id"]
    pub task_id: String,
}

#[derive(clap::Parser)]
pub struct ShowArgs {
    #[clap(value_parser)]
    #[doc = "Task id"]
    pub task_id: String,
}

#[derive(clap::Parser)]
pub struct AddRangeArgs {
    #[clap(value_parser)]
    #[doc = "Task id"]
    pub task_id: String,
    #[clap(short, long, value_parser, value_name = "SINCE_DATE")]
    #[doc = "Date since generate report. Format %d-%m-%Y"]
    pub since: Option<CliDate>,
    #[clap(short, long, value_parser, value_name = "TILL_DATE")]
    #[doc = "Date till generate report. Format %d-%m-%Y"]
    pub till: Option<CliDate>,
}

#[derive(clap::Parser)]
pub struct ReportArgs {
    #[clap(value_parser, value_name = "SINCE_DATE")]
    #[doc = "Date since generate report. Format %d-%m-%Y"]
    pub since: CliDate,
    #[clap(value_parser, value_name = "TILL_DATE")]
    #[doc = "Date till generate report. Format %d-%m-%Y"]
    pub till: CliDate,
}

#[derive(clap::Parser)]
pub struct NewArgs {
    #[clap(value_parser)]
    #[doc = "Task id"]
    pub task_id: String,

    #[clap(short, long, value_parser, value_name = "URL")]
    #[doc = "Jira issue url"]
    pub url: Option<String>,

    #[clap(short, long, value_parser, value_name = "TEXT")]
    #[doc = "Some short text description"]
    pub title: Option<String>,

    #[clap(short, long, value_parser, value_name = "WP")]
    #[doc = "Workpackage"]
    pub workpackage: Option<String>,

    #[clap(short, long, value_parser, value_name = "OBJECTIVE")]
    #[doc = "Objective"]
    pub objective: Option<String>,
}

#[derive(clap::Parser, Debug)]
pub struct EditArgs {
    #[clap(value_parser)]
    #[doc = "Task id"]
    pub task_id: String,

    #[clap(short, long, value_parser, value_name = "URL")]
    #[doc = "Set url to new value"]
    pub url: Option<String>,

    #[clap(long, value_parser)]
    #[doc = "Drop url value"]
    pub drop_url: bool,

    #[clap(short, long, value_parser, value_name = "TEXT")]
    #[doc = "Set title to new value"]
    pub title: Option<String>,

    #[clap(long, value_parser)]
    #[doc = "Drop title value"]
    pub drop_title: bool,

    #[clap(short, long, value_parser, value_name = "WP")]
    #[doc = "Set workpackage to new value"]
    pub workpackage: Option<String>,

    #[clap(long, value_parser)]
    #[doc = "Drop workpackage value"]
    pub drop_workpackage: bool,

    #[clap(short, long, value_parser, value_name = "OBJECTIVE")]
    #[doc = "Set objective to new value"]
    pub objective: Option<String>,

    #[clap(long, value_parser)]
    #[doc = "Drop objective value"]
    pub drop_objective: bool,
}
