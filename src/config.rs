use clap::{crate_version, App, Arg};
use thiserror::Error;

/// commandline configuration options
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub node_count: usize,
    pub base_port_number: usize,
}

/// returned Error for configuration failures
#[derive(Error, Debug)]
pub enum ConfigError {
    /// A mandatory item was not supplied
    #[error("value for {0} must be supplied")]
    MissingValueError(String),

    /// Error parsing a value
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

/// load and parse commandline config options
pub fn load_configuration() -> Result<Config, ConfigError> {
    let matches = App::new("Bad Network")
        .about("an unreliable network for testing")
        .version(crate_version!())
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .required(true)
                .takes_value(true)
                .help("The number of nodes"),
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .required(true)
                .takes_value(true)
                .help("base port number"),
        )
        .get_matches();

    let node_count: usize = matches
        .value_of("count")
        .ok_or_else(|| ConfigError::MissingValueError("node_count".to_string()))?
        .parse()?;

    let base_port_number: usize = matches
        .value_of("base")
        .ok_or_else(|| ConfigError::MissingValueError("base_port_number".to_string()))?
        .parse()?;

    Ok(Config {
        node_count,
        base_port_number,
    })
}
