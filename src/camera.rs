use crate::{player::player_movement, prelude::*};
use bevy::{
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(camera_follow.after(player_movement));
    }
}

fn camera_follow(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player) = player.get_single() {
        let mut camera = camera.single_mut();
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: RENDER_WIDTH as u32,
        height: RENDER_HEIGHT as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(20.0);
    camera.camera.target = RenderTarget::Image(image_handle.clone());

    commands.spawn((
        camera,
        MainCamera,
        VisibilityBundle::default(),
        UiCameraConfig { show_ui: false },
    ));

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(16.0, 9.0))));

    let material_handle = materials.add(ColorMaterial {
        texture: Some(image_handle.clone()),
        ..default()
    });

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
        Name::new("Base Render"),
    ));

    commands.insert_resource(MainRender(image_handle));

    let mut camera = Camera2dBundle::default();
    camera.camera.order = 999;
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 16.0,
        min_height: 9.0,
    };

    commands.spawn((
        camera,
        post_processing_pass_layer,
        FinalCamera,
        UiCameraConfig { show_ui: true },
    ));
}
