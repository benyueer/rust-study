use clap::{Parser, Subcommand};

/// diff two http requests and compare the difference of the responses



#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
    /// diff two API requests and compare the difference of the responses
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
    #[clap(short, long, value_parser)]
    pub profile: String,

    #[clap(short, long, value_parser = pares_key_value, number_of_values = 1)]
    extra_params: Vec<KeyValue>
}


#[derive(Debug, Clone)]
pub(crate) struct KeyValue {
    key: String,
    value: String
}

fn pares_key_value(s: &str) -> Result<KeyValue, ()> {
    todo!()
}