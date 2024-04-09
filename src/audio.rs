use crate::loading::AudioAssets;
use crate::player::FlapEvent;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin).add_systems(
            Update,
            play_flap_sound.run_if(not(in_state(GameState::Loading))),
        );
        // .add_systems(
        //     Update,
        //     control_flying_sound
        //         .after(set_movement_actions)
        //         .run_if(in_state(GameState::Playing)),
        // );
    }
}

// #[derive(Resource)]
// struct FlyingAudio(Handle<AudioInstance>);

// fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
//     audio.pause();
//     let handle = audio
//         .play(audio_assets.flying.clone())
//         .looped()
//         .with_volume(0.3)
//         .handle();
//     commands.insert_resource(FlyingAudio(handle));
// }

// fn control_flying_sound(
//     _actions: Res<Actions>,
//     audio: Res<FlyingAudio>,
//     mut audio_instances: ResMut<Assets<AudioInstance>>,
// ) {
//     if let Some(_instance) = audio_instances.get_mut(&audio.0) {
//         // match instance.state() {
//         // PlaybackState::Paused { .. } => {
//         //     if actions.player_movement.is_some() {
//         //         instance.resume(AudioTween::default());
//         //     }
//         // }
//         // PlaybackState::Playing { .. } => {
//         //     if actions.player_movement.is_none() {
//         //         instance.pause(AudioTween::default());
//         //     }
//         // }
//         // _ => {}
//         // }
//     }
// }

fn play_flap_sound(
    mut reader: EventReader<FlapEvent>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if reader.is_empty() {
        return;
    }
    reader.clear();

    audio
        .play(audio_assets.flap.clone())
        .start_from(0.025)
        .with_volume(0.5);
}
