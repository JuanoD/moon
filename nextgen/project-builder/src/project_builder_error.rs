use miette::Diagnostic;
use moon_common::{Id, Style, Stylize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ProjectBuilderError {
    #[diagnostic(code(project::missing_source))]
    #[error("No project exists at path {}.", .0.style(Style::File))]
    MissingAtSource(String),

    #[diagnostic(code(project::missing_path))]
    #[error("No project could be located starting from path {}.", .0.style(Style::Path))]
    MissingFromPath(PathBuf),

    #[diagnostic(code(project::unknown))]
    #[error("No project has been configured with the ID {}.", .0.style(Style::Id))]
    UnconfiguredID(Id),
}
