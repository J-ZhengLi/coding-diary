#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Started,
    LoadingPlatforms,
    Running,
    Paused,
    Over,
}
