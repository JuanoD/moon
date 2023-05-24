use crate::portable_path::PortablePath;
use schematic::{config_enum, Config, ValidateError};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_yaml::Value;

fn validate_env_file<D, C>(
    env_file: &TaskOptionEnvFile,
    _data: &D,
    _ctx: &C,
) -> Result<(), ValidateError> {
    if let TaskOptionEnvFile::File(file) = env_file {
        match file {
            PortablePath::EnvVar(_) => {
                return Err(ValidateError::new(
                    "environment variables are not supported",
                ));
            }
            PortablePath::ProjectGlob(_) | PortablePath::WorkspaceGlob(_) => {
                return Err(ValidateError::new("globs are not supported"));
            }
            _ => {}
        };
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum TaskOptionAffectedFiles {
    Args,
    Env,
    Enabled(bool),
}

impl<'de> Deserialize<'de> for TaskOptionAffectedFiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Value::deserialize(deserializer)? {
            Value::Bool(value) => Ok(TaskOptionAffectedFiles::Enabled(value)),
            Value::String(value) if value == "args" || value == "env" => Ok(if value == "args" {
                TaskOptionAffectedFiles::Args
            } else {
                TaskOptionAffectedFiles::Env
            }),
            _ => Err(de::Error::custom("expected `args`, `env`, or a boolean")),
        }
    }
}

config_enum!(
    #[serde(untagged, expecting = "expected a boolean or a file system path")]
    pub enum TaskOptionEnvFile {
        Enabled(bool),
        File(PortablePath),
    }
);

config_enum!(
    #[derive(Default)]
    pub enum TaskMergeStrategy {
        #[default]
        Append,
        Prepend,
        Replace,
    }
);

config_enum!(
    #[derive(Default)]
    pub enum TaskOutputStyle {
        #[default]
        Buffer,
        BufferOnlyFailure,
        Hash,
        None,
        Stream,
    }
);

#[derive(Debug, Clone, Config)]
pub struct TaskOptionsConfig {
    pub affected_files: Option<TaskOptionAffectedFiles>,

    #[setting(default = true)]
    pub cache: bool,

    #[setting(validate = validate_env_file)]
    pub env_file: Option<TaskOptionEnvFile>,

    pub merge_args: TaskMergeStrategy,

    pub merge_deps: TaskMergeStrategy,

    pub merge_env: TaskMergeStrategy,

    pub merge_inputs: TaskMergeStrategy,

    pub merge_outputs: TaskMergeStrategy,

    pub output_style: TaskOutputStyle,

    pub persistent: bool,

    pub retry_count: u8,

    #[setting(default = true)]
    pub run_deps_in_parallel: bool,

    #[setting(default = true, rename = "runInCI")]
    pub run_in_ci: bool,

    pub run_from_workspace_root: bool,

    #[setting(default = true)]
    pub shell: bool,
}