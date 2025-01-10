// const VERSION: &str = env!("CARGO_YDB_SLO_TESTS_VERSION");
// const SHA: &str = env!("GIT_HASH");

// static HELP_TEMPLATE: &str = "\
// {before-help}{name} {version}
// {author}
// {about}
//
// {usage-heading}
//   {usage}
//
// {all-args}{after-help}";

use crate::args;
use clap::{Parser, Subcommand};

#[derive(Clone, Parser)]
// #[command(
//     version = VERSION,
//     help_template(HELP_TEMPLATE),
// )]
pub struct SloTestsCli {
    #[command(subcommand)]
    pub command: Command,

    /// YDB endpoint to connect to
    pub endpoint: String,

    /// YDB database to connect to
    pub db: String,

    /// table name to create
    #[arg(long = "table-name", short, default_value_t = String::from("testingTable"))]
    pub table_name: String,

    /// write timeout milliseconds
    #[arg(long = "write-timeout", default_value_t = 10000)]
    pub write_timeout: i64,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// creates table in database
    Create(args::CreateArgs),

    /// drops table in database
    Cleanup,

    /// runs workload (read and write to table with sets RPS)
    Run(args::RunArgs),
}
