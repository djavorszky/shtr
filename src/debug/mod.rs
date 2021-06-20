use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const LABEL: &str = "debug_timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct DebugStage;

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup.system())
            .add_stage_after(
                CoreStage::Update,
                DebugStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::steps_per_second(1.0).with_label(LABEL))
                    .with_system(update_fps.system()),
            );
    }

    fn name(&self) -> &str {
        "Debug Plugin"
    }
}

struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },

            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server
                                .load("fonts/firecode-regular-nerd-font-complete.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server
                                .load("fonts/firecode-regular-nerd-font-complete.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
                text.sections[1].style.color = match average as u32 {
                    0..=28 => Color::RED,
                    29..=58 => Color::GOLD,
                    _ => Color::GREEN,
                };
            }
        }
    }
}
