use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start monitoring a directory
    #[command(arg_required_else_help = true)]
    Watch {
        /// Directory to monitor (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Initialize a new project
    #[command(arg_required_else_help = true)]
    Init {
        /// Directory to initialize (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Project name (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Configure PMS settings
    Config {
        /// Set GitHub token
        #[arg(long)]
        token: Option<String>,

        /// Set Git username
        #[arg(long)]
        username: Option<String>,

        /// Set Git email
        #[arg(long)]
        email: Option<String>,
    },
} 