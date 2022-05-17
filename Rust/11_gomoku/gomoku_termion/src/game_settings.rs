pub struct GameSettings {
    pub fps: u16,
    pub board_size: (u16, u16),
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fps: 30,
            board_size: (15, 15),
        }
    }
}
