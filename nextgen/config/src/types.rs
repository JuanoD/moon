use crate::project::TaskConfig;
use crate::shapes::InputPath;
use moon_common::Id;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

pub type FileGroupsMap = FxHashMap<Id, Vec<InputPath>>;

pub type ProjectsSourcesMap = FxHashMap<Id, String>;

pub type ProjectsAliasesMap = FxHashMap<String, Id>;

pub type TasksConfigsMap = BTreeMap<Id, TaskConfig>;
