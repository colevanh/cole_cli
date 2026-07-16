use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(display_name="ColeCLI")] // provides top level version and about commands
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

    #[arg()]
    pub my_file: Option<String>,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
 
    /// Prints contents of file to stdout
    Print(PrintArgs),
    /// Counts the number of lines in file
    NumLines(NumLinesArgs),
    /// Provides stats about working environment
    EnvVars(EnvVarsArgs),
    Generate(GenerateArgs),
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
    #[arg()]
    pub cur_directory: String,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    // TODO add args. name, email, age
    // TODO add --count and --format flags
}
