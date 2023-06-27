//use bevy::app::AppExit;
//use bevy::input::common_conditions::input_toggle_active;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use sprite_test::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TEST".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameCameraPlugin)
        //.add_plugin(GameUiPlugin)
        .add_plugin(AnimationPlugin)
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .init_resource::<Animations>()
        .register_type::<TextureAtlasSprite>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(spawn_background)
        .add_startup_system(start_music)
        .run();
}

fn start_music(audio: Res<Audio>, assets: Res<AssetServer>) {
    audio.play_with_settings(
        assets.load("sounds/nightmare-on-imaginationland-8040.ogg"),
        PlaybackSettings {
            repeat: true,
            volume: 0.3,
            speed: 1.0,
        },
    );
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    let size = 1080.0 * PIXEL_TO_WORLD;
    for i in -7..7 {
        for j in -7..7 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(i as f32 * size, j as f32 * size, 1.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    texture: assets.load("map.png"),
                    ..default()
                },
                Name::new("Background"),
            ));
        }
    }
}
