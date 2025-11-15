use crate::systems::enemy::renderer::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub fire_timer: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct WaveEndedText;

#[derive(Component)]
pub struct HUDText;

#[derive(Bundle)]
pub struct HUDBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
    location: Node,
    ui: HUDText,
}

impl HUDBundle {
    pub fn new(text: String) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            location: Node {
                top: Val::Px(10.0),
                justify_self: JustifySelf::Center,
                ..default()
            },
            ui: HUDText,
        }
    }
}

#[derive(Component)]
#[component(on_add = on_enemy_spawning)]
pub struct Spawning {
    pub timer: f32,
}

#[derive(Component)]
#[component(on_add = on_enemy_spawned)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub mesh: Mesh2d,
    pub mesh_material2d: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub player: Player,
}
