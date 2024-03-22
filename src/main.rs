use clap::{Parser, Subcommand};

pub(crate) mod commands;
mod requests {
    pub mod request;
}

const FOLDER_PATH: &str = "./requests";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}


#[derive(Debug, Subcommand)]
enum Command {
    /// Adds a new request
    New {
        /// Required - name of the request
        name: String,

        /// Optional - url
        #[clap(short = 'u')]
        url: Option<String>,

        /// Optional - method
        #[clap(short = 'm')]
        method: Option<String>,

        /// Optional - body
        #[clap(short = 'b')]
        body: Option<String>,
    },
    /// Lists all saved requets
    Ls,
    /// Edits a requests
    Edit {
        /// Required - name of the request
        #[clap(short = 'n')]
        name: String,

        /// Optional - url
        #[clap(short = 'u')]
        url: Option<String>,

        /// Optional - method
        #[clap(short = 'm')]
        method: Option<String>,

        /// Optional - body
        #[clap(short = 'b')]
        body: Option<String>,
    },
    /// Adds or edits a header
    HeaderAdd {
        /// Required - name of the request
        name: String,
        /// Required - key and value of the header, seperated by ':'
        header: String,
    },
    /// Removes a header
    HeaderRemove {
        /// Required - name of the request
        name: String,
        /// Required - key of the value to remove
        header: String,
    },
    /// Prints the selected reqeust
    Print {
        /// Required - name of the request
        name: String,
    },
    /// Runs the selected request
    Run {
        /// Required - name of the request
        name: String,
    },
    /// Removes the selected request
    Remove {
        /// Required - name of the request
        name: String,
    },
    /// Renames the selected request
    Rename {
        /// Required - name of the request
        name: String,
        /// Required - new name of the request
        new_name: String,
    },
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::New {
            name,
            url,
            method,
            body,
        } => commands::new::invoke(name, url, method, body),
        Command::Ls => commands::ls::invoke(),
        Command::Edit {
            name,
            url,
            method,
            body,
        } => commands::edit::invoke(name, url, method, body),
        Command::HeaderAdd { name, header } => commands::header_add::invoke(name, header),
        Command::HeaderRemove { name, header } => commands::header_remove::invoke(name, header),
        Command::Print { name } => commands::print::invoke(name),
        Command::Run { name } => commands::run::invoke(name),
        Command::Remove { name } => commands::remove::invoke(name),
        Command::Rename { name, new_name } => commands::rename::invoke(name, new_name),
    }
}
