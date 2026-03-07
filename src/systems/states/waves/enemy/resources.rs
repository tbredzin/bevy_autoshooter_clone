use crate::systems::animations::animation::{SpriteAnimation, Spritesheet};
use crate::systems::states::waves::components::Direction;
use crate::systems::states::waves::enemy::kinds::EnemyKind;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

const BASIC_MOB_SPRITESHEET_PATH: &str = "spritesheet/enemy/Pink_Monster.png";
const FAST_MOB_SPRITESHEET_PATH: &str = "spritesheet/enemy/Owlet_Monster_Walk_6.png";
const TANK_MOB_SPRITESHEET_PATH: &str = "spritesheet/enemy/Dude_Monster_Run_6.png";
const MINIBOSS_MOB_SPRITESHEET_PATH: &str = "spritesheet/enemy/miniboss.png";
const SPLITTER_MOB_SPRITESHEET_PATH: &str = "spritesheet/enemy/blob2.png";
const SHADOW_SPRITE_PATH: &str = "spritesheet/player/shadow_sprite.png";

#[derive(Resource)]
pub struct EnemyAnimations {
    map: HashMap<(EnemyKind, Direction), Handle<SpriteAnimation>>,
    pub shadow_texture: Handle<Image>,
}

impl EnemyAnimations {
    pub fn get(&self, kind: EnemyKind, dir: Direction) -> Option<Handle<SpriteAnimation>> {
        self.map.get(&(kind, dir)).cloned()
    }

    pub fn get_image_handle(assets: &AssetServer, kind: EnemyKind) -> Handle<Image> {
        match kind {
            EnemyKind::Basic => assets.load(BASIC_MOB_SPRITESHEET_PATH),
            EnemyKind::Fast => assets.load(FAST_MOB_SPRITESHEET_PATH),
            EnemyKind::Tank => assets.load(TANK_MOB_SPRITESHEET_PATH),
            EnemyKind::MiniBoss => assets.load(MINIBOSS_MOB_SPRITESHEET_PATH),
            EnemyKind::Boss => assets.load(MINIBOSS_MOB_SPRITESHEET_PATH),
            EnemyKind::Splitter | EnemyKind::SmallSplitter => {
                assets.load(SPLITTER_MOB_SPRITESHEET_PATH)
            }
            _ => assets.load(BASIC_MOB_SPRITESHEET_PATH),
        }
    }

    pub fn get_layout(
        texture_atlas_layout: &mut Assets<TextureAtlasLayout>,
        kind: EnemyKind,
    ) -> Handle<TextureAtlasLayout> {
        match kind {
            EnemyKind::Basic => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                8,
                1,
                None,
                None,
            )),
            EnemyKind::Fast | EnemyKind::Tank => texture_atlas_layout.add(
                TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 1, None, None),
            ),
            EnemyKind::MiniBoss | EnemyKind::Boss => texture_atlas_layout.add(
                TextureAtlasLayout::from_grid(UVec2::splat(64), 6, 1, None, None),
            ),
            EnemyKind::Splitter | EnemyKind::SmallSplitter => texture_atlas_layout.add(
                TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 3, None, None),
            ),
            EnemyKind::Ranged => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                8,
                1,
                None,
                None,
            )),
        }
    }
}

impl FromWorld for EnemyAnimations {
    fn from_world(world: &mut World) -> Self {
        let direction_rows: &[(Direction, bool)] = &[
            (Direction::SOUTH, true),
            (Direction::WEST, true),
            (Direction::NORTHWEST, true),
            (Direction::SOUTHWEST, true),
            (Direction::NORTH, false),
            (Direction::NORTHEAST, false),
            (Direction::EAST, false),
            (Direction::SOUTHEAST, false),
        ];
        let mut map = HashMap::new();
        for kind in EnemyKind::iterator() {
            for (dir, reversed) in direction_rows {
                let image = { Self::get_image_handle(world.resource::<AssetServer>(), kind) };
                let layout = {
                    Self::get_layout(
                        world.resource_mut::<Assets<TextureAtlasLayout>>().as_mut(),
                        kind,
                    )
                };
                let animation = match kind {
                    EnemyKind::Splitter => SpriteAnimation {
                        spritesheet: Spritesheet {
                            image,
                            layout,
                            first: 0,
                            last: 22,
                            flip_x: *reversed,
                            custom_size: Some(Vec2::splat(32.)),
                        },
                        frame_interval: Duration::from_millis(120),
                        repeat: true,
                    },
                    EnemyKind::SmallSplitter => SpriteAnimation {
                        spritesheet: Spritesheet {
                            image,
                            layout,
                            first: 0,
                            last: 22,
                            flip_x: *reversed,
                            custom_size: Some(Vec2::splat(16.)),
                        },
                        frame_interval: Duration::from_millis(120),
                        repeat: true,
                    },
                    EnemyKind::MiniBoss => SpriteAnimation {
                        frame_interval: Duration::from_millis(120),
                        spritesheet: Spritesheet {
                            first: 0,
                            image,
                            layout,
                            last: 5,
                            flip_x: !*reversed,
                            custom_size: None,
                        },
                        repeat: true,
                    },
                    EnemyKind::Boss => SpriteAnimation {
                        frame_interval: Duration::from_millis(120),
                        spritesheet: Spritesheet {
                            first: 0,
                            image,
                            layout,
                            last: 5,
                            flip_x: !*reversed,
                            custom_size: Some(Vec2::splat(96.)),
                        },
                        repeat: true,
                    },
                    EnemyKind::Basic | EnemyKind::Ranged => SpriteAnimation {
                        frame_interval: Duration::from_millis(120),
                        spritesheet: Spritesheet {
                            first: 0,
                            image,
                            layout,
                            last: 7,
                            flip_x: *reversed,
                            custom_size: None,
                        },
                        repeat: true,
                    },
                    EnemyKind::Fast => SpriteAnimation {
                        frame_interval: Duration::from_millis(60),
                        spritesheet: Spritesheet {
                            first: 0,
                            image,
                            layout,
                            last: 5,
                            flip_x: *reversed,
                            custom_size: None,
                        },
                        repeat: true,
                    },
                    EnemyKind::Tank => SpriteAnimation {
                        frame_interval: Duration::from_millis(120),
                        spritesheet: Spritesheet {
                            first: 0,
                            image,
                            layout,
                            last: 5,
                            flip_x: *reversed,
                            custom_size: None,
                        },
                        repeat: true,
                    },
                };

                map.insert(
                    (kind, *dir),
                    world
                        .resource_mut::<Assets<SpriteAnimation>>()
                        .add(animation),
                );
            }
        }

        EnemyAnimations {
            map,
            shadow_texture: world.resource::<AssetServer>().load(SHADOW_SPRITE_PATH),
        }
    }
}
