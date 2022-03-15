use sdl2::mixer::*;
use std::path::Path;
use legion::*;
use crossbeam::channel::*;

use super::render::components::renderdelta::*;

/// Default search path for sounds
pub const SOUNDS_PATH: &str = "./sounds";

/// Default volume level for all channels
pub const DEFAULT_VOLUME_PERCENT: i32 = 50;

/// Resource with all sound files loaded in memory.
pub struct AudioReses {
    // pub bang: CooledChunk,
}

/// Sound chunk with attached cooldown timer
pub struct CooledChunk {
    pub chunk: Chunk,
    pub cooldown: f32,
}

/// Event that is triggered when some system wants to play sound
#[derive(Debug)]
pub enum PlayAudio {
    // BangSound,
}

impl AudioReses {
    pub fn init() -> Result<Self, String> {
        let sounds = load_audio_chunks(Path::new(SOUNDS_PATH))?;
        Channel::all().set_volume(DEFAULT_VOLUME_PERCENT);
        Ok(sounds)
    }
}

impl CooledChunk {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        Ok(CooledChunk {
            chunk: Chunk::from_file(path)?,
            cooldown: 0.0,
        })
    }
}

/// Load all wav files from given directory
pub fn load_audio_chunks(directory: &Path) -> Result<AudioReses, String> {
    Ok(AudioReses {
        // bang: CooledChunk::from_file(directory.join("bangMedium.wav"))?,
    })
}

/// System that play sounds and control their cooldowns.
/// The system is designed to be thread local, so use `with_thread_local` when
/// register it.
#[system]
pub fn audio(
    #[state] mut sounds: &mut AudioReses,
    #[resource] audio_chan: &Receiver<PlayAudio>,
    #[resource] delta: &RenderDelta,
) {
    let dt = delta.0.as_secs_f32();
    // Play sounds
    play_sounds(&mut sounds, &audio_chan)
        .unwrap_or_else(|err| println!("Failed to play sound: {}", err));
    // Update cooldowns
    // update_cooldown(&mut sounds.bang, dt);
}

fn play_sounds(sounds: &mut AudioReses, audio_chan: &Receiver<PlayAudio>) -> Result<(), String> {
    for event in audio_chan.try_iter() {
        match event {
            // PlayAudio::BangSound => play_with_cooldown(&mut sounds.bang, 0.4)?,
        }
    }
    Ok(())
}

fn play_with_cooldown(ch: &mut CooledChunk, dt: f32) -> Result<(), String> {
    if ch.cooldown == 0.0 {
        ch.cooldown = dt;
        Channel::all().play(&ch.chunk, 0).map(|_| ())?;
    }
    Ok(())
}

fn update_cooldown(ch: &mut CooledChunk, dt: f32) {
    if ch.cooldown > 0.0 {
        ch.cooldown -= dt;
        if ch.cooldown < 0.0 {
            ch.cooldown = 0.0;
        }
    }
}
