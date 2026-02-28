use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(
    name = "synapse",
    version,
    propagate_version = true,
    about = "Synapse - An open-source, extensible Coding Agent"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Start an interactive chat session (default).
    Chat,
    /// Execute a single prompt non-interactively.
    Run {
        /// The prompt to execute.
        prompt: String,
    },
    /// Start the HTTP/WebSocket API server.
    Server {
        /// Port to listen on.
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        None | Some(Command::Chat) => {
            tracing::warn!("synapse chat: not yet implemented");
        }
        Some(Command::Run { prompt }) => {
            tracing::warn!("synapse run: not yet implemented (prompt: {prompt})");
        }
        Some(Command::Server { port }) => {
            tracing::warn!("synapse server: not yet implemented (port: {port})");
        }
    }

    Ok(())
}
