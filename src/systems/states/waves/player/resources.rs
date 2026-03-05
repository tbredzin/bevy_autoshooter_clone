use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::count_frames;
use crate::systems::animations::components::AnimationDuration;
use crate::systems::states::waves::components::Action::IDLE;
use crate::systems::states::waves::components::{Action, Direction};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{FromWorld, Resource, World};
use std::collections::HashMap;
use std::time::Duration;

pub const PLAYER_SPEED: f32 = 200.0;
const IDLE_SPRITESHEET_PATH: &str = "spritesheet/player/Idle_spritesheet_8x6.png";
const WALK_SPRITESHEET_PATH: &str = "spritesheet/player/walk_spritesheet_8x6.png";
const DASH_SPRITESHEET_PATH: &str = "spritesheet/player/dash_spritesheet_8x6.png";
const DEATH_SPRITESHEET_PATH: &str = "spritesheet/player/death_spritesheet_8x6.png";
const SHADOW_SPRITE_PATH: &str = "spritesheet/player/shadow_sprite.png";

#[derive(Resource)]
pub struct PlayerAnimations {
    map: HashMap<(Action, Direction), Handle<SpriteAnimation>>,
    pub shadow_texture: Handle<Image>,
}

impl PlayerAnimations {
    pub fn get(&self, action: Action, dir: Direction) -> Option<Handle<SpriteAnimation>> {
        self.map.get(&(action, dir)).cloned()
    }

    pub fn get_image_handle(assets: &AssetServer, action: Action) -> Handle<Image> {
        match action {
            Action::IDLE => assets.load(IDLE_SPRITESHEET_PATH),
            Action::WALKING => assets.load(WALK_SPRITESHEET_PATH),
            Action::DASHING => assets.load(DASH_SPRITESHEET_PATH),
            Action::DYING => assets.load(DEATH_SPRITESHEET_PATH),
        }
    }
    pub fn get_layout(
        texture_atlas_layout: &mut Assets<TextureAtlasLayout>,
        action: Action,
    ) -> Handle<TextureAtlasLayout> {
        match action {
            Action::IDLE => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::new(48, 64),
                8,
                6,
                None,
                None,
            )),
            Action::WALKING => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::new(48, 64),
                8,
                6,
                None,
                None,
            )),
            Action::DASHING => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::new(48, 64),
                8,
                6,
                None,
                None,
            )),
            Action::DYING => texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                UVec2::new(48, 64),
                8,
                6,
                None,
                None,
            )),
        }
    }
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let layout = {
            Self::get_layout(
                world.resource_mut::<Assets<TextureAtlasLayout>>().as_mut(),
                IDLE,
            )
        };

        let configs: &[(Action, u64, bool)] = &[
            (Action::IDLE, 120, true),
            (Action::WALKING, 120, true),
            (Action::DASHING, 120, false),
            (Action::DYING, 300, false),
        ];

        let direction_rows: &[(Direction, usize)] = &[
            (Direction::SOUTH, 0),
            (Direction::SOUTHWEST, 1),
            (Direction::WEST, 1),
            (Direction::NORTHWEST, 2),
            (Direction::NORTH, 3),
            (Direction::NORTHEAST, 4),
            (Direction::EAST, 5),
            (Direction::SOUTHEAST, 5),
        ];

        let mut map = HashMap::new();
        for (action, frame_ms, looping) in configs {
            for (dir, row) in direction_rows {
                let image = { Self::get_image_handle(world.resource::<AssetServer>(), *action) };
                let nb_frames =
                    count_frames(world.resource::<Assets<TextureAtlasLayout>>(), &layout);
                let anim =
                    SpriteAnimation::from_row(image.clone(), layout.clone(), *row, nb_frames)
                        .with_duration(AnimationDuration::PerFrame(Duration::from_millis(
                            *frame_ms,
                        )))
                        .looping(*looping);
                map.insert(
                    (*action, *dir),
                    world.resource_mut::<Assets<SpriteAnimation>>().add(anim),
                );
            }
        }

        PlayerAnimations {
            map,
            shadow_texture: world.resource::<AssetServer>().load(SHADOW_SPRITE_PATH),
        }
    }
}
