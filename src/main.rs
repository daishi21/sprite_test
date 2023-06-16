use sprite_test::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EditorPlugin::default())
        .add_plugin(CamerasPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationsPlugin)
        .init_resource::<Animations>()
        .register_type::<TextureAtlasSprite>()
        .add_startup_system(spawn_background)
        .run()
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
