pub mod player {
    use connect4_lib::game::PlayerType;
    use connect4_lib::ai;

    pub const AI_EASY: &'static str = "ai_easy";
    pub const AI_EASY2: &'static str = "ai_easy2";
    pub const AI_MID: &'static str = "ai_mid";
    pub const AI_MID2: &'static str = "ai_mid2";
    pub const AI_HARD: &'static str = "ai_hard";
    pub const AI_HARD2: &'static str = "ai_hard2";
    pub const LOCAL: &'static str = "local";
    pub const REMOTE: &'static str = "remote";
    pub const AI: &'static str = "ai";

    pub fn string_to_enum(player: &str) -> PlayerType {
        match player {
            AI_EASY => PlayerType::AI(ai::EASY_AI),
            AI_EASY2 => PlayerType::AI(ai::EASY_AI),
            AI_MID => PlayerType::AI(ai::MID_AI),
            AI_MID2 => PlayerType::AI(ai::MID_AI),
            AI_HARD => PlayerType::AI(ai::HARD_AI),
            AI_HARD2 => PlayerType::AI(ai::HARD_AI),
            AI => PlayerType::AI(ai::HARD_AI),
            LOCAL => PlayerType::Local,
            REMOTE => PlayerType::Remote,
            _ => PlayerType::Local,
        }
    }
}

pub mod game {
    pub const CONNECT4: &'static str = "connect4";
    pub const TOTO: &'static str = "toto";
    pub const CUSTOM: &'static str = "custom";
}
