use schematic::{config_enum, Config};

config_enum!(
    #[derive(Default)]
    pub enum HasherOptimization {
        #[default]
        Accuracy,
        Performance,
    }
);

config_enum!(
    #[derive(Default)]
    pub enum HasherWalkStrategy {
        Glob,
        #[default]
        Vcs,
    }
);

#[derive(Config)]
pub struct HasherConfig {
    #[setting(default = 2500)]
    pub batch_size: u16,

    pub optimization: HasherOptimization,

    pub walk_strategy: HasherWalkStrategy,

    #[setting(default = true)]
    pub warn_on_missing_inputs: bool,
}