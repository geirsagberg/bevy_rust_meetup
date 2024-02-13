use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};

use crate::assets::TextureAssets;
use crate::materials::ParallaxMaterial;
use crate::GameState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ParallaxMaterial>::default())
            .add_systems(OnExit(GameState::Loading), spawn_background);
    }
}

fn spawn_background(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParallaxMaterial>>,
    images: Res<Assets<Image>>,
) {
    let image_size = images
        .get(&texture_assets.background_1)
        .unwrap()
        .size()
        .as_vec2();
    let mesh: Mesh2dHandle = meshes.add(Mesh::from(Quad::new(image_size))).into();

    commands
        .spawn(SpriteBundle {
            texture: texture_assets.background_1.clone(),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: texture_assets.background_2.clone(),
                transform: Transform::from_xyz(0., 0., 0.1),
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: materials.add(ParallaxMaterial {
                    texture: Some(texture_assets.background_3.clone()),
                    speed: 0.02,
                }),
                transform: Transform::from_xyz(0., 0., 0.2),
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: materials.add(ParallaxMaterial {
                    texture: Some(texture_assets.background_4.clone()),
                    speed: 0.03,
                }),
                transform: Transform::from_xyz(0., 0., 0.3),
                ..default()
            });

            parent.spawn(MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: materials.add(ParallaxMaterial {
                    texture: Some(texture_assets.background_5.clone()),
                    speed: 0.04,
                }),
                transform: Transform::from_xyz(0., 0., 0.4),
                ..default()
            });
        });
}
