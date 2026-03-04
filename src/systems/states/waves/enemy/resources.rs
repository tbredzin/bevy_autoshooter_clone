use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::count_frames;
use crate::systems::animations::components::AnimationDuration;
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
            EnemyKind::Fast => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                6,
                1,
                None,
                None,
            )),
            EnemyKind::Tank => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                6,
                1,
                None,
                None,
            )),
            EnemyKind::MiniBoss => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::splat(64),
                6,
                1,
                None,
                None,
            )),
            _ => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
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
        let configs: &[(EnemyKind, usize, u64, bool)] = &[
            (EnemyKind::Basic, 0, 120, true),
            (EnemyKind::Ranged, 0, 120, true),
            (EnemyKind::Fast, 0, 30, true),
            (EnemyKind::Splitter, 0, 120, true),
            (EnemyKind::SmallSplitter, 0, 120, true),
            (EnemyKind::Tank, 0, 120, true),
            (EnemyKind::MiniBoss, 0, 120, true),
            (EnemyKind::Boss, 0, 120, true),
        ];

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
        for (kind, row, frame_ms, looping) in configs {
            for (dir, reversed) in direction_rows {
                let image = { Self::get_image_handle(world.resource::<AssetServer>(), *kind) };
                let layout = {
                    Self::get_layout(
                        world.resource_mut::<Assets<TextureAtlasLayout>>().as_mut(),
                        *kind,
                    )
                };
                let nb_frames =
                    count_frames(world.resource::<Assets<TextureAtlasLayout>>(), &layout);
                let mut anim =
                    SpriteAnimation::from_row(image.clone(), layout.clone(), *row, nb_frames)
                        .reversed(*reversed)
                        .with_duration(AnimationDuration::PerFrame(Duration::from_millis(
                            *frame_ms,
                        )));
                if *looping {
                    anim = anim.looping();
                }
                map.insert(
                    (*kind, *dir),
                    world.resource_mut::<Assets<SpriteAnimation>>().add(anim),
                );
            }
        }

        EnemyAnimations {
            map,
            shadow_texture: world.resource::<AssetServer>().load(SHADOW_SPRITE_PATH),
        }
    }
}
