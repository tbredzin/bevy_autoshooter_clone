use crate::components::{Background, Player, UIText};
use crate::resources::GAME_AREA;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Background
    //TODO: DO BETTER

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(GAME_AREA.width()*2.0, GAME_AREA.height()*2.0))),
    //     MeshMaterial2d(materials.add(Color::WHITE)),
    //     Transform::from_xyz(GAME_AREA.min.x, GAME_AREA.max.y, 0.0),
    //     Background,
    // ));

    // Player
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player { fire_timer: 0.0 },
    ));

    // Game UI - Centered at top
    commands.spawn((
        Text::new("Wave: 1 | XP: 0 | Level: 1 | HP: 100/100"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            top: Val::Px(10.0),
            justify_self: JustifySelf::Center,
            ..default()
        },
        UIText,
    ));

    println!("GAME_AREA: {GAME_AREA:?}");
}
