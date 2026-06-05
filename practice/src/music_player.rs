/*

trait 的本质是：定义类型之间共享的行为契约

不用 trait：每个类型都是孤岛，代码耦合度高，难以测试

用 trait：系统灵活可扩展，易于测试，符合开闭原则


🎵 音乐播放器
实现一个迷你音乐播放器的核心逻辑（不需要真实播放音频，模拟状态即可）。

要求包含：
枚举 PlayState
Stopped - 停止状态

Playing { track_index: usize } - 播放状态，记录当前播放的歌曲索引

Paused { track_index: usize, position_secs: u32 } - 暂停状态，记录歌曲索引和暂停位置（秒）

枚举 RepeatMode
None - 不循环，播放完最后一首自动停止

One - 单曲循环，当前歌曲播放完重复同一首

All - 列表循环，播放完最后一首回到第一首

结构体 Track
歌曲信息，包含：

title: String - 歌名

artist: String - 艺术家

duration_secs: u32 - 时长（秒）

需要实现 Display trait，输出格式如："Queen - Bohemian Rhapsody [05:54]"

结构体 Player
播放器，包含：

playlist: Vec<Track> - 曲库

state: PlayState - 当前播放状态

repeat_mode: RepeatMode - 当前循环模式

Trait Playable
定义播放器行为，包含以下方法：

play(&mut self, track_index: usize) -> Result<(), String> - 播放指定索引的歌曲

pause(&mut self) -> Result<(), String> - 暂停当前播放

stop(&mut self) -> Result<(), String> - 停止播放

next(&mut self) -> Result<(), String> - 下一首（根据循环模式决定）

previous(&mut self) -> Result<(), String> - 上一首

is_playing(&self) -> bool - 带默认实现，判断是否正在播放

状态流转规则（必须合理）
pause() 在 Stopped 状态下应返回错误

play() 可以从 Stopped、Paused 状态启动/恢复播放

stop() 可以从 Playing 或 Paused 状态停止

next() / previous() 需要根据 RepeatMode 决定行为

所有非法操作应返回 Err 并给出清晰的错误信息

其他要求
为 Player 实现 Display trait，打印当前播放状态（包括歌曲信息、暂停位置等）

RepeatMode 需要派生 Debug

main 函数中演示：
创建至少 4 首歌曲的曲库

遍历打印所有歌曲信息

正常流程演示：播放 → 暂停 → 继续 → 下一首

演示不同循环模式的效果（单曲循环、列表循环）

非法操作演示：对 Stopped 状态调用 pause()，打印错误信息

演示 is_playing() 方法的正确输出

*/

use std::fmt;

enum PlayState {
    Stopped,
    Playing {
        track_index: usize,
    },
    Paused {
        track_index: usize,
        position_secs: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RepeatMode {
    None,
    One,
    All,
}

struct Track {
    title: String,
    artist: String,
    duration_secs: u32, // duration in seconds
}

struct Player {
    playlist: Vec<Track>,
    state: PlayState,
    repeat_mode: RepeatMode,
}

impl Track {
    fn new(title: &str, artist: &str, duration_secs: u32) -> Self {
        Track {
            title: title.to_string(),
            artist: artist.to_string(),
            duration_secs,
        }
    }

    fn format_duration(&self) -> String {
        let minutes = self.duration_secs / 60;
        let seconds = self.duration_secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} [{}]",
            self.artist,
            self.title,
            self.format_duration()
        )
    }
}

impl Player {
    fn new(playlist: Vec<Track>) -> Self {
        Player {
            playlist,
            state: PlayState::Stopped,
            repeat_mode: RepeatMode::None,
        }
    }

    fn current_track_index(&self) -> Option<usize> {
        match &self.state {
            PlayState::Playing { track_index } => Some(*track_index),
            PlayState::Paused { track_index, .. } => Some(*track_index),
            PlayState::Stopped => None,
        }
    }

    fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
        println!("Repeat mode set to: {:?}", mode);
    }

    fn get_next_track_index(&self, current_index: usize) -> Option<usize> {
        match self.repeat_mode {
            RepeatMode::One => Some(current_index),
            RepeatMode::All => {
                if current_index + 1 < self.playlist.len() {
                    Some(current_index + 1)
                } else {
                    Some(0) // loop back to start
                }
            }
            RepeatMode::None => {
                if current_index + 1 < self.playlist.len() {
                    Some(current_index + 1)
                } else {
                    None
                }
            }
        }
    }
}

// 假设未来有多种播放器
// struct Mp3Player { ... }
// struct StreamingPlayer { ... }

// 都可以实现同一个 Playable trait
trait Playable {
    fn play(&mut self, track_index: usize) -> Result<(), String>;
    fn pause(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn next(&mut self) -> Result<(), String>;
    fn previous(&mut self) -> Result<(), String>;

    // Method with default implementation
    fn is_playing(&self) -> bool {
        matches!(self.current_state(), PlayState::Playing { .. })
    }

    fn current_state(&self) -> &PlayState;
}

impl Playable for Player {
    fn play(&mut self, track_index: usize) -> Result<(), String> {
        if track_index >= self.playlist.len() {
            return Err(format!("Track index {} out of bounds", track_index));
        }

        match &self.state {
            PlayState::Stopped => {
                self.state = PlayState::Playing { track_index };
                println!("▶️  Playing: {}", self.playlist[track_index]);
            }
            PlayState::Paused {
                track_index: idx,
                position_secs,
            } => {
                if *idx == track_index {
                    println!(
                        "▶️  Resuming: {} from {}:{}",
                        self.playlist[track_index],
                        position_secs / 60,
                        position_secs % 60
                    );
                    self.state = PlayState::Playing { track_index: *idx };
                } else {
                    self.state = PlayState::Playing { track_index };
                    println!("▶️  Playing new track: {}", self.playlist[track_index]);
                }
            }
            PlayState::Playing { .. } => {
                println!("Already playing track: {}", self.playlist[track_index]);
            }
        }
        Ok(())
    }

    fn pause(&mut self) -> Result<(), String> {
        match &self.state {
            PlayState::Playing { track_index } => {
                let pos = 30; // Simulate pausing at 30 seconds
                self.state = PlayState::Paused {
                    track_index: *track_index,
                    position_secs: pos,
                };
                println!("⏸️  Paused at {}:{:02}", pos / 60, pos % 60);
                Ok(())
            }
            PlayState::Stopped => {
                Err("Cannot pause: Player is stopped. Use play() first.".to_string())
            }
            PlayState::Paused { .. } => Err("Already paused.".to_string()),
        }
    }

    fn stop(&mut self) -> Result<(), String> {
        match &self.state {
            PlayState::Playing { .. } | PlayState::Paused { .. } => {
                self.state = PlayState::Stopped;
                println!("⏹️  Stopped playback");
                Ok(())
            }
            PlayState::Stopped => Err("Already stopped.".to_string()),
        }
    }

    fn next(&mut self) -> Result<(), String> {
        match &self.state {
            PlayState::Stopped => Err("Cannot go to next track: Player is stopped.".to_string()),
            PlayState::Playing { track_index } | PlayState::Paused { track_index, .. } => {
                if let Some(next_idx) = self.get_next_track_index(*track_index) {
                    self.state = PlayState::Playing {
                        track_index: next_idx,
                    };
                    println!("⏭️  Next track: {}", self.playlist[next_idx]);
                    Ok(())
                } else {
                    self.state = PlayState::Stopped;
                    println!("End of playlist. Playback stopped.");
                    Ok(())
                }
            }
        }
    }

    fn previous(&mut self) -> Result<(), String> {
        match &self.state {
            PlayState::Stopped => {
                Err("Cannot go to previous track: Player is stopped.".to_string())
            }
            PlayState::Playing { track_index } | PlayState::Paused { track_index, .. } => {
                if *track_index > 0 {
                    let prev_idx = track_index - 1;
                    self.state = PlayState::Playing {
                        track_index: prev_idx,
                    };
                    println!("⏮️  Previous track: {}", self.playlist[prev_idx]);
                    Ok(())
                } else {
                    Err("Already at the first track.".to_string())
                }
            }
        }
    }

    fn current_state(&self) -> &PlayState {
        &self.state
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            PlayState::Stopped => {
                write!(f, "Player State: ⏹️  Stopped")
            }
            PlayState::Playing { track_index } => {
                let track = &self.playlist[*track_index];
                write!(
                    f,
                    "Player State: ▶️  Playing - {} - {} [{}]",
                    track.artist,
                    track.title,
                    track.format_duration()
                )
            }
            PlayState::Paused {
                track_index,
                position_secs,
            } => {
                let track = &self.playlist[*track_index];
                write!(
                    f,
                    "Player State: ⏸️  Paused at {}:{:02} - {} - {}",
                    position_secs / 60,
                    position_secs % 60,
                    track.artist,
                    track.title
                )
            }
        }
    }
}

pub fn run_music_player() {
    // Create playlist
    let playlist = vec![
        Track::new("Bohemian Rhapsody", "Queen", 354),
        Track::new("Stairway to Heaven", "Led Zeppelin", 482),
        Track::new("Imagine", "John Lennon", 183),
        Track::new("Hotel California", "Eagles", 390),
    ];

    let mut player = Player::new(playlist);

    // Print all tracks
    println!("📀 PLAYLIST:");
    for (i, track) in player.playlist.iter().enumerate() {
        println!("  {}. {}", i + 1, track);
    }
    println!();

    // Test normal flow
    println!("=== NORMAL FLOW ===");
    println!("Current: {}", player);
    let _ = player.play(0);
    println!("{}", player);
    let _ = player.pause();
    println!("{}", player);
    let _ = player.play(0);
    println!("{}", player);
    let _ = player.next();
    println!("{}", player);
    println!();

    // Test repeat modes
    println!("=== REPEAT MODES ===");
    player.set_repeat_mode(RepeatMode::One);
    println!("Current track will repeat when next is pressed");
    let _ = player.next(); // Should play same track
    println!("{}", player);

    player.set_repeat_mode(RepeatMode::All);
    println!("Will loop through all tracks");
    for _ in 0..6 {
        let _ = player.next();
        println!("{}", player);
    }
    println!();

    // Test illegal operations
    println!("=== ILLEGAL OPERATIONS ===");
    let _ = player.stop();
    println!("After stop: {}", player);

    match player.pause() {
        Ok(_) => println!("Pause succeeded"),
        Err(e) => println!("❌ Error: {}", e),
    }

    match player.next() {
        Ok(_) => println!("Next succeeded"),
        Err(e) => println!("❌ Error: {}", e),
    }

    match player.previous() {
        Ok(_) => println!("Previous succeeded"),
        Err(e) => println!("❌ Error: {}", e),
    }
    println!();

    // Test is_playing method
    println!("=== IS PLAYING CHECK ===");
    let _ = player.play(2);
    println!(
        "Is playing? {}",
        if player.is_playing() { "Yes" } else { "No" }
    );
    let _ = player.pause();
    println!(
        "Is playing? {}",
        if player.is_playing() { "Yes" } else { "No" }
    );
    let _ = player.stop();
    println!(
        "Is playing? {}",
        if player.is_playing() { "Yes" } else { "No" }
    );
}
