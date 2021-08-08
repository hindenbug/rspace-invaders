use bevy::prelude::*;

// Entity, Component, System, Resource

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut _windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn a sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(200.0, 100.0)),
        ..Default::default()
    });

    // // position window
    // let window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(2000, 3000))
}
