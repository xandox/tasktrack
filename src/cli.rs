use chrono::{Date, NaiveDate, Utc};

#[derive(Debug, Clone, Copy)]
pub struct CliDate(pub Date<Utc>);

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
        let result = NaiveDate::parse_from_str(&str, "%d-%m-%Y");
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
    Current,
    List(ListArgs),
    New(NewArgs),
    Activate(ActivateArgs),
    Edit(NewArgs),
    Report(ReportArgs),
    Show(ShowArgs),
}

#[derive(clap::Parser)]
pub struct ListArgs {
    #[clap(short, long, value_parser, value_name = "INT")]
    pub num_tasks: Option<usize>,
}

#[derive(clap::Parser)]
pub struct ActivateArgs {
    #[clap(value_parser)]
    pub task_id: String,
}

#[derive(clap::Parser)]
pub struct ShowArgs {
    #[clap(value_parser)]
    pub task_id: String,
}

#[derive(clap::Parser)]
pub struct ReportArgs {
    #[clap(value_parser, value_name = "DATE")]
    pub since: CliDate,
    #[clap(value_parser, value_name = "DATE")]
    pub till: CliDate,
}

#[derive(clap::Parser)]
pub struct NewArgs {
    #[clap(value_parser)]
    pub task_id: String,

    #[clap(short, long, value_parser, value_name = "URL")]
    pub url: Option<String>,

    #[clap(short, long, value_parser, value_name = "TEXT")]
    pub title: Option<String>,

    #[clap(short, long, value_parser, value_name = "WP")]
    pub workpackage: Option<String>,

    #[clap(short, long, value_parser, value_name = "OBJECTIVE")]
    pub objective: Option<String>,
}
