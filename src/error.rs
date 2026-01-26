use std::path::PathBuf;

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
use thiserror::Error as ThisError;

/// Enumeration of all the errors reported by Clorinde.
#[derive(Debug, ThisError, Diagnostic)]
pub enum Error {
    /// An error while trying to connect to a database.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Connection(#[from] crate::conn::error::Error),
    /// An error while trying to read PostgreSQL query files.
    #[error(transparent)]
    #[diagnostic(transparent)]
    ReadQueries(#[from] crate::read_queries::error::Error),
    /// An error while trying to parse PostgreSQL query files.
    #[error(transparent)]
    #[diagnostic(transparent)]
    ParseQueries(#[from] crate::parser::error::Error),
    /// An error while trying to validate PostgreSQL query files.
    #[error(transparent)]
    #[diagnostic(transparent)]
    ValidateQueries(#[from] Box<crate::validation::error::Error>),
    /// An error while manipulating a container managed by Clorinde.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Container(#[from] crate::container::error::Error),
    /// An error while trying to prepare PostgreSQL queries.
    #[error(transparent)]
    #[diagnostic(transparent)]
    PrepareQueries(#[from] crate::prepare_queries::error::Error),
    /// An error while reading PostgreSQL schema files.
    #[error(transparent)]
    #[diagnostic(transparent)]
    LoadSchema(#[from] crate::load_schema::error::Error),
    /// An error while trying to write the generated crate to its destination.
    #[error(transparent)]
    #[diagnostic(transparent)]
    PersistCrate(#[from] PersistError),
    /// An error while trying to read the config flle
    #[error(transparent)]
    #[diagnostic(transparent)]
    Config(#[from] crate::config::ConfigError),
    /// An error while introspecting database schema
    #[error("Schema introspection failed: {0}")]
    SchemaIntrospection(String),
}

impl Error {
    #[must_use]
    pub fn report(self) -> String {
        let mut buff = String::new();
        if GraphicalReportHandler::new()
            .with_theme(GraphicalTheme::unicode_nocolor())
            .render_report(&mut buff, &self)
            .is_err()
        {
            format!("Error: {self}")
        } else {
            buff
        }
    }
}

#[derive(Debug, ThisError, Diagnostic)]
#[error("Could not perform IO on file `{file_path}`: ({err})")]
pub struct PersistError {
    pub(crate) file_path: PathBuf,
    pub(crate) err: std::io::Error,
}

impl PersistError {
    pub fn wrap(path: impl Into<PathBuf>) -> impl FnOnce(std::io::Error) -> PersistError {
        |err| PersistError {
            file_path: path.into(),
            err,
        }
    }
}
