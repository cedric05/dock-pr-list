use clap::{Parser, Subcommand};

/// Simple slip command to better organize github repositories
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(global = true, short, long, env_os = "DOCKER_USERNAME")]
    pub username: Option<String>,

    #[clap(global = true, short, long, env_os = "DOCKER_PASSWORD")]
    pub password: Option<String>,

    #[clap(global = true, short, long, env_os = "DOCKER_REGISTRY")]
    pub registry: Option<String>,

    #[clap(global = true, short, long)]
    pub insecure: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

impl Args {}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// clone repository
    Ls {
        image: String,
    },

    Search {
        image: String,
    },
}
