use clap::{Parser, Subcommand, Args, ValueEnum};

/// Top level struct for the application
/// holds the command enum
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // NOTE: provides top level version and about commands
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

}

/// enum containing all commands available to the application
/// 
#[derive(Subcommand, Debug)]
pub enum Commands {
 
    /// Prints contents of file to stdout
    Print(PrintArgs),
    /// Counts the number of lines in file
    NumLines(NumLinesArgs),
    /// Provides stats about working environment
    EnvVars(EnvVarsArgs),
    /// Generates test user(s)
    Generate(GenerateArgs),
}

/// input is the file specified by the user at the command line
#[derive(Args, Debug)]
pub struct PrintArgs {
    #[arg(short = 'i', long = "input", default_value = "-", help = "file to be used as input")]
    pub input: Option<String>,
}

/// see PrintArgs comment
/// TODO research how input could be defined elsewhere and shared across commands
#[derive(Args, Debug)]
pub struct NumLinesArgs {
    #[arg(short = 'i', long = "input", default_value = "-", help = "file to be used as input")]
    pub input: Option<String>,
}

/// TODO not sure what to define here
#[derive(Args, Debug)]
pub struct EnvVarsArgs {
    // TODO add args. These might include OS info, cwd, metadata on cwd, etc.
    #[arg()]
    pub cur_directory: String,
}


#[derive(Args, Debug)]
pub struct GenerateArgs {
    // TODO add args. name, email, age
    // TODO add --count and --format flags
    #[arg(
        short = 'n', 
        long = "name", 
        help = "user's name. First, full, doesn't matter",
        default_value = "cvh",
    )]
    pub name: String,

    #[arg(
        short = 'e', 
        long = "email", 
        help = "user's email address. Ping me bro",
        default_value = "cvh@gmail.com",
    )]
    pub email: String,
}
