mod animations;
mod cameras;
mod player;

pub mod prelude {

    pub use bevy::prelude::*;
    pub use bevy_editor_pls::prelude::*;
    pub use std::collections::HashMap;

    pub use crate::animations::AnimationsPlugin;
    pub use crate::cameras::CamerasPlugin;
    pub use crate::player::PlayerPlugin;

    pub const WIDTH: f32 = 948.0;
    pub const HEIGHT: f32 = 533.0;
    pub const RENDER_WIDTH: f32 = 960.;
    pub const RENDER_HEIGHT: f32 = 540.;
    pub const PIXEL_TO_WORLD: f32 = 30. / 960.;

    #[derive(Debug, Hash, PartialEq, Eq)]
    pub enum Animation {
        PlayerIdle,
        PlayerUp,
        PlayerDown,
        PlayerLeft,
        PlayerRight,
    }

    #[derive(Component)]
    pub struct FrameTime(pub f32);

    #[derive(Resource)]
    pub struct MainRender(pub Handle<Image>);

    #[derive(Component)]
    pub struct MainCamera;

    #[derive(Component)]
    pub struct FinalCamera;

    #[derive(Component, Clone, Copy)]
    pub struct SpriteAnimation {
        pub len: usize,
        pub frame_time: f32,
    }

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
        // pub health: f32,
        // pub max_health: f32,
        // pub damage: f32,
        pub facing: Facing,
    }

    pub enum Facing {
        Left,
        Right,
    }

    #[derive(Resource)]
    pub struct Animations {
        pub map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
    }

    impl Animations {
        pub fn add(
            &mut self,
            id: Animation,
            handle: Handle<TextureAtlas>,
            animation: SpriteAnimation,
        ) {
            self.map.insert(id, (handle, animation));
        }
        pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
            self.map.get(&id).cloned()
        }
    }

    impl FromWorld for Animations {
        fn from_world(world: &mut World) -> Self {
            let mut map = Animations {
                map: HashMap::new(),
            };
            world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {
                let asset_server = world.resource::<AssetServer>();
                let idel_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_player.png"),
                    Vec2::new(69., 80.),
                    1,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerIdle,
                    texture_atles.add(idel_atlas),
                    SpriteAnimation {
                        len: 1,
                        frame_time: 1. / 5.,
                    },
                );

                let left_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_left.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerLeft,
                    texture_atles.add(left_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );

                let right_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_right.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerRight,
                    texture_atles.add(right_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );

                let up_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_up.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerUp,
                    texture_atles.add(up_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );
                let down_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_down.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerDown,
                    texture_atles.add(down_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );
            });

            map
        }
    }
}
