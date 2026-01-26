use std::sync::OnceLock;

pub fn level_paths() -> &'static Vec<&'static str> {
    static LEVEL_PATHS: OnceLock<Vec<&'static str>> = OnceLock::new();
    LEVEL_PATHS.get_or_init(|| vec!["assets/worlds/level_01.ron", "assets/worlds/level_02.ron"])
}
