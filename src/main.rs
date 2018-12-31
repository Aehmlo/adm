use structopt::StructOpt;

mod config;
mod error;
mod turn;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    #[structopt(raw(setting = "structopt::clap::AppSettings::DisableVersion"))]
    /// Manage device power states from the command line.
    Turn {
        /// The device to modify.
        device: String,
        /// The desired state of the device (on or off).
        state: String,
        /// Send requests quickly, without waiting for confirmation.
        #[structopt(short, long)]
        fast: bool,
    },
    /// Manage configuration settings/files.
    Config {
        #[structopt(subcommand)]
        command: config::ConfigCommand,
    },
}

fn main() -> Result<(), error::Error> {
    match Command::from_args() {
        Command::Turn {
            device,
            state,
            fast,
        } => {
            turn::turn(device, state, fast)?;
            Ok(())
        }
        Command::Config { command } => {
            config::config(command)?;
            Ok(())
        }
    }
}
