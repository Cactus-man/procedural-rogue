use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_freecam::FreeCamPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use procedural_rogue::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Proelium".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin,
            YamlAssetPlugin::<ItemType>::new(&["item.yaml"]),
            MovementPlugin::default(),
            FreeCamPlugin {
                spawn_camera: true,
                ..Default::default()
            },
            ConsolePlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(AttributeSet::new_prefilled())
        .add_asset::<ItemType>()
        .add_systems(Startup, item_types)
        .add_systems(Startup, spawn_player_and_light)
        .add_systems(Update, draw_velocity_and_origin)
        .run();
}

fn item_types(asset_server: Res<AssetServer>) {
    let _ = asset_server.load_folder("items");
}

fn spawn_player_and_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Cube::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::SEA_GREEN,
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32_000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh,
            material,
            ..Default::default()
        })
        .insert((Move::default(), Player::default(), Name::new("Player")));
}

fn draw_velocity_and_origin(query: Query<(&Transform, &Move), With<Player>>, mut gizmos: Gizmos) {
    let (transform, movement) = query.single();

    let position = transform.translation;
    let projection = position - Vec3::new(0.0, position.y, 0.0);

    gizmos.ray(position, -projection.normalize(), Color::CYAN);
    gizmos.ray(position, movement.velocity(), Color::FUCHSIA);
}
