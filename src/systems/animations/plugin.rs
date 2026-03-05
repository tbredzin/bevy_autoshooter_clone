use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::SpriteAnimator;
use crate::systems::animations::messages::AnimationEnded;
use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteAnimation>()
            .add_message::<AnimationEnded>()
            .add_systems(Update, tick_sprite_animators);
    }
}

fn tick_sprite_animators(
    time: Res<Time>,
    animations: Res<Assets<SpriteAnimation>>,
    mut query: Query<(Entity, &mut SpriteAnimator, &mut Sprite)>,
    mut ended: MessageWriter<AnimationEnded>,
) {
    for (entity, mut animator, mut sprite) in &mut query {
        // ── Apply pending switch ─────────────────────────────────────────────
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
