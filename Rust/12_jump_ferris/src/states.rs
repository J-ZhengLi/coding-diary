#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Started,
    LoadingPlatforms,
    InitPlayer,
    LoadingPlayer,
    Running,
    Editing,
    Paused,
    Over,
}
