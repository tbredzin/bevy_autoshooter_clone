use crate::systems::states::waves::weapons::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::systems::states::waves::weapons::messages::{
    BulletSpawnedMessage, WeaponSpawnedMessage,
};
use bevy::color::palettes::basic::{AQUA, RED};
use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
pub fn render_bullet(
    mut commands: Commands,
    mut events: MessageReader<BulletSpawnedMessage>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        let (sprite, sound) = {
            match event.bullet.kind {
                Shotgun => (
                    Sprite::from_color(RED, event.transform.scale.truncate()),
                    assets.load("effects/bullet1.ogg"),
                ),
                Pistol => (
                    Sprite::from_color(BLUE, event.transform.scale.truncate()),
                    assets.load("effects/bullet2.ogg"),
                ),
                MachineGun => (
                    Sprite::from_color(AQUA, event.transform.scale.truncate()),
                    assets.load("effects/bullet3.ogg"),
                ),
            }
        };
        commands.entity(event.entity).insert((
            sprite,
            AudioPlayer::new(sound),
            PlaybackSettings::ONCE,
        ));
    }
}

pub fn render_weapon(
    mut commands: Commands,
    assets: ResMut<AssetServer>,
    mut events: MessageReader<WeaponSpawnedMessage>,
) {
    for event in events.read() {
        debug!("Weapon {:?} spawned: {:?}", event.name, event);
        commands
            .entity(event.entity)
            .insert((match event.weapon.kind {
                Shotgun => Sprite::from_color(RED, event.weapon.weapon_size),
                Pistol => Sprite::from_color(BLUE, event.weapon.weapon_size),
                MachineGun => Sprite::from_image(assets.load("sprites/wand.png")),
            },));
    }
}
