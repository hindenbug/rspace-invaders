use bevy::prelude::*;

// Entity, Component, System, Resource

struct Player;

pub struct Materials {
    player_materials: Handle<ColorMaterial>,
}

struct WindowSize {
    height: f32,
    width: f32,
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create resources
    commands.insert_resource(Materials {
        player_materials: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
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
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        sprite: Sprite::new(Vec2::new(200.0, 100.0)),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + 75. / 2. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
