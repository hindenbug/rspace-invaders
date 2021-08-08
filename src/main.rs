use bevy::prelude::*;

const TIME_STEP: f32 = 1. / 60.;
const PLAYER_STRIPE: &str = "playerShip1_blue.png";
const PLAYER_LASER_STRIPE: &str = "laserBlue01.png";

// Entity, Component, System, Resource

// Component
struct Player {
    speed: f32,
}

struct Laser {
    speed: f32,
}
//

// Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}

struct WindowSize {
    height: f32,
    width: f32,
}
//

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create resources
    let player_texture = asset_server.load(PLAYER_STRIPE);
    let player_laser = asset_server.load(PLAYER_LASER_STRIPE);

    commands.insert_resource(Materials {
        player_materials: materials.add(player_texture.into()),
        laser: materials.add(player_laser.into()),
    });

    commands.insert_resource(WindowSize {
        height: window.height(),
        width: window.width(),
    });

    // position window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(200, 300));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WindowSize>) {
    // sprite
    let bottom = -win_size.height / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            sprite: Sprite::new(Vec2::new(200.0, 100.0)),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + 75. / 2. + 5., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 500.0 });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let direction = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        transform.translation.x += direction * player.speed * TIME_STEP
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Player, &Transform)>,
) {
    if let Ok((_player, transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            let x = transform.translation.x;
            let y = transform.translation.y;

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser { speed: 500.0 });
        }
    }
}
