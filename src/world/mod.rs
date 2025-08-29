use avian3d::prelude::*;
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

use crate::world::lighting::spawn_light;

mod lighting;

#[derive(Resource)]
struct ColliderMeshHandle(Handle<Mesh>);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_map(asset_server: Res<AssetServer>, mut commands: Commands) {
    let map_gltf_scene: Handle<Scene> = asset_server.load("maps/fps_tps_map.glb#Scene0");

    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        SceneRoot(map_gltf_scene),
    ));
}

pub fn load_map_collider(asset_server: Res<AssetServer>, mut commands: Commands) {
    // NOTE: glb is binary from gltf
    let map_gltf: Handle<Mesh> =
        asset_server.load("maps/fps_tps_map_colliders.glb#Mesh0/Primitive0");

    commands.insert_resource(ColliderMeshHandle(map_gltf));
}

fn spawn_colliders_from_map_colliders(
    mut commands: Commands,
    collider_mesh_handle_resource: Res<ColliderMeshHandle>,
    meshes: Res<Assets<Mesh>>,
    mut is_built: Local<bool>,
) {
    if *is_built {
        return;
    }

    let Some(gltf) = meshes.get(&collider_mesh_handle_resource.0) else {
        info!(
            "Gltf not yet found in gltf assets resource. This most likely means the gltf model hasnt finished loading yet."
        );
        return;
    };

    commands.spawn((
        Collider::trimesh_from_mesh(gltf).expect("PLEASE"),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RigidBody::Static,
    ));

    info!("setting is_built to true, this system wont be used anymore!");
    *is_built = true;
}

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        Friction::new(1.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3 {
                x: 50.0,
                y: 0.5,
                z: 50.0,
            },
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })),
        RigidBody::Static,
    ));
}
