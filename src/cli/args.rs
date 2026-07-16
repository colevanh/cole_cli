use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /* 
    #[arg(short = 'i', long = "input", default_value = "-", help = "file to be used as input")]
    pub input: Option<String>,

    #[arg(short = 'n', long = "name", default_value = "Cole Van H", help = "The user's first name")]
    pub name: String,

    #[arg(short = 'e', long = "email", default_value = "colevanh@gmail.com", help = "The user's email address")]
    pub email: String,

    #[arg(value_parser = clap::value_parser!(u16).range(10..), short, long, default_value = "20")]
    pub age: u16,
    */

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /* 
    #[command(about = "Print the contents of a file")]
    Print,
    #[command(about = "Print the number of lines in a file", author = "Cole")]
    NumLines,
    EnvVars,
    */
    
    /// Prints contents of file to stdout
    Print(PrintArgs),
    /// Counts the number of lines in file
    NumLines(NumLinesArgs),
    /// Provides stats about working environment
    EnvVars(EnvVarsArgs),
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    #[arg(short = 'i', long = "input", default_value = "-", help = "file to be used as input")]
    pub input: Option<String>,
}

#[derive(Args, Debug)]
pub struct NumLinesArgs {
    #[arg(short = 'i', long = "input", default_value = "-", help = "file to be used as input")]
    pub input: Option<String>,
}

#[derive(Args, Debug)]
pub struct EnvVarsArgs {
    // TODO add args. These might include OS info, cwd, metadata on cwd, etc.
}
