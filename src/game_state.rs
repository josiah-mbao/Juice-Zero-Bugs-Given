use bevy::prelude::*;

// This enum represents the main states of our game
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame, // For now, we start directly in the game
    GameOver,
    #[allow(dead_code)]
    Paused,
    Statistics,
}

// Resource to track the winner
#[derive(Resource, Default)]
pub struct Winner {
    pub player_id: Option<u8>,
    pub is_human_winner: Option<bool>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArenaType {
    #[default]
    Default,
    DataRace,
    UndefinedBehavior,
    BufferOverflow,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BossType {
    #[default]
    NullPointer,
    UndefinedBehavior,
    DataRace,
    UseAfterFree,
    BufferOverflow,
}

#[derive(Resource, Default)]
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub boss: BossType,
    pub player2_is_human: bool,
    pub arena: ArenaType,
}

#[derive(Resource, Debug, Clone)]
pub struct PlayerProgress {
    pub unlocked_bosses: Vec<BossType>,
    pub statistics: Statistics,
}

#[derive(Debug, Clone)]
pub struct Statistics {
    // Global stats
    pub total_fights: u32,
    pub total_wins: u32,
    pub current_win_streak: u32,
    pub best_win_streak: u32,

    // Per-boss stats
    pub boss_stats: std::collections::HashMap<BossType, BossStatistics>,
}

#[derive(Debug, Clone, Default)]
pub struct BossStatistics {
    pub wins: u32,
    pub losses: u32,
    pub best_combo: u32,
    pub fastest_victory_seconds: Option<f32>,
}

impl Default for PlayerProgress {
    fn default() -> Self {
        Self {
            unlocked_bosses: vec![BossType::NullPointer], // Start with first boss unlocked
            statistics: Statistics::default(),
        }
    }
}

impl Default for Statistics {
    fn default() -> Self {
        let mut boss_stats = std::collections::HashMap::new();

        // Initialize stats for all bosses
        let all_bosses = [
            BossType::NullPointer,
            BossType::UndefinedBehavior,
            BossType::DataRace,
            BossType::UseAfterFree,
            BossType::BufferOverflow,
        ];

        for boss in all_bosses {
            boss_stats.insert(boss, BossStatistics::default());
        }

        Self {
            total_fights: 0,
            total_wins: 0,
            current_win_streak: 0,
            best_win_streak: 0,
            boss_stats,
        }
    }
}

impl PlayerProgress {
    pub fn is_boss_unlocked(&self, boss: BossType) -> bool {
        self.unlocked_bosses.contains(&boss)
    }

    pub fn unlock_boss(&mut self, boss: BossType) {
        if !self.is_boss_unlocked(boss) {
            self.unlocked_bosses.push(boss);
            // Save progress when unlocking a boss
            Self::save_progress(self);
        }
    }

    pub fn record_fight_start(&mut self, _boss: BossType) {
        self.statistics.total_fights += 1;
        // Save after each fight
        Self::save_progress(self);
    }

    pub fn record_victory(
        &mut self,
        boss: BossType,
        fight_duration_seconds: f32,
        final_combo: u32,
    ) {
        self.statistics.total_wins += 1;
        self.statistics.current_win_streak += 1;
        if self.statistics.current_win_streak > self.statistics.best_win_streak {
            self.statistics.best_win_streak = self.statistics.current_win_streak;
        }

        if let Some(boss_stat) = self.statistics.boss_stats.get_mut(&boss) {
            boss_stat.wins += 1;
            if final_combo > boss_stat.best_combo {
                boss_stat.best_combo = final_combo;
            }
            if let Some(current_fastest) = boss_stat.fastest_victory_seconds {
                if fight_duration_seconds < current_fastest {
                    boss_stat.fastest_victory_seconds = Some(fight_duration_seconds);
                }
            } else {
                boss_stat.fastest_victory_seconds = Some(fight_duration_seconds);
            }
        }

        Self::save_progress(self);
    }

    pub fn record_defeat(&mut self, boss: BossType) {
        self.statistics.current_win_streak = 0;

        if let Some(boss_stat) = self.statistics.boss_stats.get_mut(&boss) {
            boss_stat.losses += 1;
        }

        Self::save_progress(self);
    }

    pub fn get_next_boss(&self, current: BossType) -> Option<BossType> {
        let all_bosses = [
            BossType::NullPointer,
            BossType::UndefinedBehavior,
            BossType::DataRace,
            BossType::UseAfterFree,
            BossType::BufferOverflow,
        ];

        let current_index = all_bosses.iter().position(|&b| b == current)?;
        let next_index = current_index + 1;

        if next_index < all_bosses.len() {
            Some(all_bosses[next_index])
        } else {
            None // Last boss beaten
        }
    }

    pub fn save_progress(progress: &PlayerProgress) {
        use std::fs;

        let save_path = "player_progress.json";

        // Convert BossType to strings for serialization, removing duplicates
        let mut seen = std::collections::HashSet::new();
        let unlocked_bosses: Vec<String> = progress
            .unlocked_bosses
            .iter()
            .filter(|boss| seen.insert(*boss))
            .map(|boss| match boss {
                BossType::NullPointer => "NullPointer".to_string(),
                BossType::UndefinedBehavior => "UndefinedBehavior".to_string(),
                BossType::DataRace => "DataRace".to_string(),
                BossType::UseAfterFree => "UseAfterFree".to_string(),
                BossType::BufferOverflow => "BufferOverflow".to_string(),
            })
            .collect();

        // Convert boss statistics to serializable format
        let boss_statistics: std::collections::HashMap<String, serde_json::Value> = progress
            .statistics
            .boss_stats
            .iter()
            .map(|(boss, stats)| {
                let boss_name = match boss {
                    BossType::NullPointer => "NullPointer",
                    BossType::UndefinedBehavior => "UndefinedBehavior",
                    BossType::DataRace => "DataRace",
                    BossType::UseAfterFree => "UseAfterFree",
                    BossType::BufferOverflow => "BufferOverflow",
                };
                (
                    boss_name.to_string(),
                    serde_json::json!({
                        "wins": stats.wins,
                        "losses": stats.losses,
                        "best_combo": stats.best_combo,
                        "fastest_victory_seconds": stats.fastest_victory_seconds
                    }),
                )
            })
            .collect();

        let save_data = serde_json::json!({
            "unlocked_bosses": unlocked_bosses,
            "statistics": {
                "total_fights": progress.statistics.total_fights,
                "total_wins": progress.statistics.total_wins,
                "current_win_streak": progress.statistics.current_win_streak,
                "best_win_streak": progress.statistics.best_win_streak,
                "boss_statistics": boss_statistics
            }
        });

        if let Ok(json_string) = serde_json::to_string_pretty(&save_data) {
            if fs::write(save_path, json_string).is_ok() {
                tracing::info!("Player progress saved successfully");
            } else {
                tracing::warn!("Failed to write player progress to file");
            }
        } else {
            tracing::warn!("Failed to serialize player progress");
        }
    }

    pub fn load_progress() -> PlayerProgress {
        use std::fs;
        use std::path::Path;

        let save_path = "player_progress.json";

        if Path::new(save_path).exists() {
            if let Ok(json_string) = fs::read_to_string(save_path) {
                if let Ok(save_data) = serde_json::from_str::<serde_json::Value>(&json_string) {
                    let mut progress = PlayerProgress::default();

                    // Load unlocked bosses
                    if let Some(unlocked_bosses_array) = save_data.get("unlocked_bosses") {
                        if let Some(unlocked_bosses_vec) = unlocked_bosses_array.as_array() {
                            progress.unlocked_bosses.clear();
                            for boss_str in unlocked_bosses_vec {
                                if let Some(boss_name) = boss_str.as_str() {
                                    let boss_type = match boss_name {
                                        "NullPointer" => BossType::NullPointer,
                                        "UndefinedBehavior" => BossType::UndefinedBehavior,
                                        "DataRace" => BossType::DataRace,
                                        "UseAfterFree" => BossType::UseAfterFree,
                                        "BufferOverflow" => BossType::BufferOverflow,
                                        _ => continue,
                                    };
                                    progress.unlocked_bosses.push(boss_type);
                                }
                            }
                        }
                    }

                    // Load statistics
                    if let Some(statistics_obj) = save_data.get("statistics") {
                        if let Some(total_fights) = statistics_obj.get("total_fights") {
                            if let Some(fights) = total_fights.as_u64() {
                                progress.statistics.total_fights = fights as u32;
                            }
                        }
                        if let Some(total_wins) = statistics_obj.get("total_wins") {
                            if let Some(wins) = total_wins.as_u64() {
                                progress.statistics.total_wins = wins as u32;
                            }
                        }
                        if let Some(current_streak) = statistics_obj.get("current_win_streak") {
                            if let Some(streak) = current_streak.as_u64() {
                                progress.statistics.current_win_streak = streak as u32;
                            }
                        }
                        if let Some(best_streak) = statistics_obj.get("best_win_streak") {
                            if let Some(streak) = best_streak.as_u64() {
                                progress.statistics.best_win_streak = streak as u32;
                            }
                        }

                        // Load boss statistics
                        if let Some(boss_stats_obj) = statistics_obj.get("boss_statistics") {
                            if let Some(boss_stats_map) = boss_stats_obj.as_object() {
                                for (boss_name, boss_data) in boss_stats_map {
                                    let boss_type = match boss_name.as_str() {
                                        "NullPointer" => BossType::NullPointer,
                                        "UndefinedBehavior" => BossType::UndefinedBehavior,
                                        "DataRace" => BossType::DataRace,
                                        "UseAfterFree" => BossType::UseAfterFree,
                                        "BufferOverflow" => BossType::BufferOverflow,
                                        _ => continue,
                                    };

                                    if let Some(boss_stat) =
                                        progress.statistics.boss_stats.get_mut(&boss_type)
                                    {
                                        if let Some(wins) = boss_data.get("wins") {
                                            if let Some(w) = wins.as_u64() {
                                                boss_stat.wins = w as u32;
                                            }
                                        }
                                        if let Some(losses) = boss_data.get("losses") {
                                            if let Some(l) = losses.as_u64() {
                                                boss_stat.losses = l as u32;
                                            }
                                        }
                                        if let Some(best_combo) = boss_data.get("best_combo") {
                                            if let Some(c) = best_combo.as_u64() {
                                                boss_stat.best_combo = c as u32;
                                            }
                                        }
                                        if let Some(fastest) =
                                            boss_data.get("fastest_victory_seconds")
                                        {
                                            if let Some(f) = fastest.as_f64() {
                                                boss_stat.fastest_victory_seconds = Some(f as f32);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    tracing::info!("Player progress loaded successfully");
                    return progress;
                }
            }
        }

        tracing::info!("No save file found, starting with default progress");
        PlayerProgress::default()
    }
}

impl GameConfig {
    pub fn save_config(config: &GameConfig) {
        use std::fs;

        let config_path = "game_config.json";

        let save_data = serde_json::json!({
            "difficulty": match config.difficulty {
                Difficulty::Easy => "Easy",
                Difficulty::Normal => "Normal",
                Difficulty::Hard => "Hard",
            },
            "boss": match config.boss {
                BossType::NullPointer => "NullPointer",
                BossType::UndefinedBehavior => "UndefinedBehavior",
                BossType::DataRace => "DataRace",
                BossType::UseAfterFree => "UseAfterFree",
                BossType::BufferOverflow => "BufferOverflow",
            },
            "arena": match config.arena {
                ArenaType::Default => "Default",
                ArenaType::DataRace => "DataRace",
                ArenaType::UndefinedBehavior => "UndefinedBehavior",
                ArenaType::BufferOverflow => "BufferOverflow",
            },
            "player2_is_human": config.player2_is_human
        });

        if let Ok(json_string) = serde_json::to_string_pretty(&save_data) {
            if fs::write(config_path, json_string).is_ok() {
                tracing::info!("Game config saved successfully");
            } else {
                tracing::warn!("Failed to write game config to file");
            }
        } else {
            tracing::warn!("Failed to serialize game config");
        }
    }

    pub fn load_config() -> GameConfig {
        use std::fs;
        use std::path::Path;

        let config_path = "game_config.json";

        if Path::new(config_path).exists() {
            if let Ok(json_string) = fs::read_to_string(config_path) {
                if let Ok(config_data) = serde_json::from_str::<serde_json::Value>(&json_string) {
                    let mut config = GameConfig::default();

                    if let Some(difficulty) = config_data.get("difficulty") {
                        if let Some(diff_str) = difficulty.as_str() {
                            config.difficulty = match diff_str {
                                "Easy" => Difficulty::Easy,
                                "Hard" => Difficulty::Hard,
                                _ => Difficulty::Normal,
                            };
                        }
                    }

                    if let Some(boss) = config_data.get("boss") {
                        if let Some(boss_str) = boss.as_str() {
                            config.boss = match boss_str {
                                "UndefinedBehavior" => BossType::UndefinedBehavior,
                                "DataRace" => BossType::DataRace,
                                "UseAfterFree" => BossType::UseAfterFree,
                                "BufferOverflow" => BossType::BufferOverflow,
                                _ => BossType::NullPointer,
                            };
                        }
                    }

                    if let Some(arena) = config_data.get("arena") {
                        if let Some(arena_str) = arena.as_str() {
                            config.arena = match arena_str {
                                "DataRace" => ArenaType::DataRace,
                                "UndefinedBehavior" => ArenaType::UndefinedBehavior,
                                "BufferOverflow" => ArenaType::BufferOverflow,
                                _ => ArenaType::Default,
                            };
                        }
                    }

                    if let Some(player2_human) = config_data.get("player2_is_human") {
                        if let Some(is_human) = player2_human.as_bool() {
                            config.player2_is_human = is_human;
                        }
                    }

                    tracing::info!("Game config loaded successfully");
                    return config;
                }
            }
        }

        tracing::info!("No config file found, starting with default config");
        GameConfig::default()
    }
}
