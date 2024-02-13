use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;

use crate::assets::AudioAssets;
use crate::GameState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), play_music)
            .add_systems(OnExit(GameState::InGame), stop_music);
    }
}

fn play_music(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.music_1.clone(),
        settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.3))),
    });
}

fn stop_music(mut commands: Commands, query: Query<Entity, With<AudioSink>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
