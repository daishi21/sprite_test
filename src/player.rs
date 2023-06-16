use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

fn spawn_player(mut commands: Commands, animaitons: Res<Animations>) {
    let Some((texture_atlas, animation)) = animaitons.get(Animation::PlayerIdle) else {error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(80.0 * PIXEL_TO_WORLD, 80.0 * PIXEL_TO_WORLD)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player {
            //health: 100.0,
            //max_health: 100.0,
            speed: 5.0,
            //damage: 5.0,
            facing: Facing::Right,
        },
        animation,
        FrameTime(0.0),
    ));
}

pub fn move_player(
    mut player: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * player.speed;
        player.facing = Facing::Left;
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * player.speed;
        player.facing = Facing::Right;
    }
}
