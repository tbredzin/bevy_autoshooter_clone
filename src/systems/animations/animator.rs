use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::messages::AnimationEnded;
use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimator {
    pub current: Handle<SpriteAnimation>,
    pub pending: Option<Handle<SpriteAnimation>>,
    pub frame_timer: Timer,
    pub finished: bool,
}

impl SpriteAnimator {
    /// `pending` is pre-set so the tick system initialises the sprite on the very first frame.
    pub fn new(handle: Handle<SpriteAnimation>) -> Self {
        Self {
            current: handle.clone(),
            pending: Some(handle),
            frame_timer: Timer::from_seconds(1.0 / 8.0, TimerMode::Repeating),
            finished: false,
        }
    }

    /// Stages a clip change. No-op if the handle is already current.
    /// The actual atlas/image update happens inside the tick system.
    pub fn switch(&mut self, handle: Handle<SpriteAnimation>) {
        if handle != self.current {
            self.pending = Some(handle);
        }
    }
}

pub fn count_frames(
    resource: &Assets<TextureAtlasLayout>,
    layout: &Handle<TextureAtlasLayout>,
) -> usize {
    let atlas = resource.get(layout).unwrap();
    (atlas.size.x / atlas.textures[0].width()) as usize
}

pub fn tick_sprite_animators(
    time: Res<Time>,
    animations: Res<Assets<SpriteAnimation>>,
    mut query: Query<(Entity, &mut SpriteAnimator, &mut Sprite)>,
    mut ended: MessageWriter<AnimationEnded>,
) {
    for (entity, mut animator, mut sprite) in &mut query {
        if let Some(next) = animator.pending.take() {
            let Some(anim) = animations.get(&next) else {
                continue;
            };
            animator.current = next;
            animator.finished = false;
            animator.frame_timer = Timer::new(anim.frame_interval, TimerMode::Repeating);

            sprite.image = anim.spritesheet.image.clone();
            sprite.flip_x = anim.spritesheet.flip_x;
            match &mut sprite.texture_atlas {
                Some(atlas) => {
                    atlas.layout = anim.spritesheet.layout.clone();
                    atlas.index = anim.spritesheet.first;
                }
                None => {
                    sprite.texture_atlas = Some(TextureAtlas {
                        layout: anim.spritesheet.layout.clone(),
                        index: anim.spritesheet.first,
                    });
                }
            }
        }

        if animator.finished {
            continue;
        }

        let Some(anim) = animations.get(&animator.current) else {
            continue;
        };

        // ── Tick ─────────────────────────────────────────────────────────────
        animator.frame_timer.tick(time.delta());
        if !animator.frame_timer.just_finished() {
            continue;
        }

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if atlas.index < anim.spritesheet.last {
            atlas.index += 1;
        } else if anim.repeat {
            atlas.index = anim.spritesheet.first;
        } else {
            animator.finished = true;
            ended.write(AnimationEnded { entity });
        }
    }
}
