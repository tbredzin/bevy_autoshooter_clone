use crate::systems::animations::components::*;
use crate::systems::animations::resources::AnimationAssets;
use crate::systems::states::waves::components::{Dying, LevelOverlay};
use crate::systems::states::waves::player::components::{Direction, Player, PlayerAction};
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;

const DYING_OVERLAY_TARGET_ALPHA: f32 = 0.80;
const DYING_OVERLAY_FADE_SPEED_PER_SEC: f32 = 1.4;
pub fn update_player_sprite(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    mut query: Query<
        (Entity, &Direction, &PlayerAction, &mut Sprite),
        (
            With<PlayerSprite>,
            Or<(Changed<Direction>, Changed<PlayerAction>)>,
        ),
    >,
) {
    for (entity, direction, action, mut sprite) in query.iter_mut() {
        let (image, atlas) = animation_assets.get_sprite(action);
        let animation = animation_assets.get_animation(action, direction);
        sprite.image = image;
        sprite.texture_atlas = Some(TextureAtlas::from(atlas).with_index(animation.first));
        commands.entity(entity).insert(animation);
    }
}

pub fn animate_player_sprite(time: Res<Time>, mut query: Query<(&mut Animation, &mut Sprite)>) {
    if let Ok((mut animation, mut sprite)) = query.single_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < animation.last {
                    atlas.index += 1;
                }
                if animation.repeated && atlas.index == animation.last {
                    atlas.index = animation.first
                }
            }
        }
    }
}

pub fn animate_game_over(
    time: Res<Time>,
    player: Single<Entity, (With<Dying>, With<Player>)>,
    mut background: Query<&mut BackgroundColor, With<LevelOverlay>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if player.is_empty() {
        return;
    }
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        camera_transform.translation.y += 1.;
    }

    for mut bg in background.iter_mut() {
        let alpha = bg.0.alpha();
        if alpha < DYING_OVERLAY_TARGET_ALPHA {
            bg.0.set_alpha(
                (alpha + time.delta_secs() * DYING_OVERLAY_FADE_SPEED_PER_SEC)
                    .min(DYING_OVERLAY_TARGET_ALPHA),
            );
        }
        return;
    }
}
