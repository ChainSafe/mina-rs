mod error;
mod subcommands;

use structopt::StructOpt;
use subcommands::daemon;

#[derive(StructOpt, Debug)]
#[structopt(name = "mina-rs cli to interact with the mina network")]
enum Commands {
    #[structopt(
        name = "daemon",
        about = "Starts the mina daemon and initiates syncing with the network"
    )]
    Daemon {
        // add flags for daemon here
    },
}

fn main() -> anyhow::Result<()> {
    use Commands::*;
    let opt = Commands::from_args();

    match opt {
        Daemon {} => daemon()?,
    }

    Ok(())
}
