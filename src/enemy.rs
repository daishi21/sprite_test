use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_slime)
            .add_startup_system(spawn_naga)
            .add_system(slime_movement)
            .add_system(naga_movement);
    }
}

fn spawn_slime(mut commands: Commands, animaitons: Res<Animations>) {
    let Some((texture_atlas, animation)) = animaitons.get(Animation::SlimeIdle) else {error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(0.0, 5.0, 2.0),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(80.0 * PIXEL_TO_WORLD, 80.0 * PIXEL_TO_WORLD)),
                ..Default::default()
            },
            ..Default::default()
        },
        Slime {
            speed: 1.5,
            health: 100.0,
            damage: 10.0,
            facing: Facing::Right,
        },
        Name::new("Slime"),
        Collider::capsule(Vec2::new(0.0, 0.55), Vec2::new(0.0, -0.50), 0.5),
        GamePlayEntity,
        animation,
        FrameTime(0.0),
    ));
}

fn spawn_naga(mut commands: Commands, animaitons: Res<Animations>) {
    let Some((texture_atlas, animation)) = animaitons.get(Animation::NagaIdle) else {error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(0.0, -5.0, 2.0),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(80.0 * PIXEL_TO_WORLD, 80.0 * PIXEL_TO_WORLD)),
                ..Default::default()
            },
            ..Default::default()
        },
        Naga {
            speed: 1.5,
            health: 100.0,
            damage: 10.0,
            facing: Facing::Right,
        },
        Name::new("Naga"),
        Collider::capsule(Vec2::new(0.0, 0.55), Vec2::new(0.0, -0.50), 0.5),
        GamePlayEntity,
        animation,
        FrameTime(0.0),
    ));
}

fn naga_movement(
    player: Query<&Transform, (With<Player>, Without<Naga>)>,
    mut naga: Query<(&mut Transform, &mut Naga)>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    for (mut transform, mut naga) in &mut naga {
        let direction = (player_transform.translation.truncate()
            - transform.translation.truncate())
        .normalize();

        transform.translation += (direction * time.delta_seconds() * naga.speed).extend(0.);

        if direction.x.abs() > direction.y.abs() {
            if direction.x < 0.0 {
                naga.facing = Facing::Left;
            } else if direction.x > 0.0 {
                naga.facing = Facing::Right;
            }
        } else {
            if direction.y > 0.0 {
                naga.facing = Facing::Up;
            } else if direction.y < 0.0 {
                naga.facing = Facing::Down;
            }
        }
        // println!("Enemy Facing {:?}", enemy.facing);
    }
}

fn slime_movement(
    player: Query<&Transform, (With<Player>, Without<Slime>)>,
    mut slime: Query<(&mut Transform, &mut Slime)>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    for (mut transform, mut slime) in &mut slime {
        let direction = (player_transform.translation.truncate()
            - transform.translation.truncate())
        .normalize();

        transform.translation += (direction * time.delta_seconds() * slime.speed).extend(0.);

        if direction.x.abs() > direction.y.abs() {
            if direction.x < 0.0 {
                slime.facing = Facing::Left;
            } else if direction.x > 0.0 {
                slime.facing = Facing::Right;
            }
        } else {
            if direction.y > 0.0 {
                slime.facing = Facing::Up;
            } else if direction.y < 0.0 {
                slime.facing = Facing::Down;
            }
        }
        // println!("Enemy Facing {:?}", enemy.facing);
    }
}

/*
fn enemy_movement(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    for (mut transform, mut enemy) in &mut enemy {
        let direction = (player_transform.translation.truncate()
            - transform.translation.truncate())
        .normalize();

        transform.translation += (direction * time.delta_seconds() * enemy.speed).extend(0.);

        if direction.x.abs() > direction.y.abs() {
            if direction.x < 0.0 {
                enemy.facing = Facing::Left;
            } else if direction.x > 0.0 {
                enemy.facing = Facing::Right;
            }
        } else {
            if direction.y > 0.0 {
                enemy.facing = Facing::Up;
            } else if direction.y < 0.0 {
                enemy.facing = Facing::Down;
            }
        }
        // println!("Enemy Facing {:?}", enemy.facing);
    }
}
*/
