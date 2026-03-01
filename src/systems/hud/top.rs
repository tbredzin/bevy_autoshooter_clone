use crate::systems::hud::components::{HUDLevelUp, HUDLevelUps, HUDText, HUDTop};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::states::waves::player::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::*;

const NB_LEVEL_UPS_PER_ROW: f32 = 6.0;
pub fn spawn_hud(mut commands: Commands) {
    commands.spawn(HUDTop::new(
        "Wave: 1 | XP: 0 | Level: 1 | HP: 100/100".to_string(),
    ));
    // HUD Level ups locations
    commands.spawn((
        HUDLevelUps {},
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::RowReverse,
            column_gap: Val::Px(12.0),
            row_gap: Val::Px(10.0),
            width: Val::Px(NB_LEVEL_UPS_PER_ROW * (32.0 + 12.0)), // 6x 32+10
            height: Val::Px(32.0),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
    ));
}

pub fn despawn_hud(
    mut commands: Commands,
    top: Single<Entity, With<HUDText>>,
    levelups: Single<Entity, With<HUDLevelUps>>,
) {
    commands.entity(top.entity()).despawn();
    commands.entity(levelups.entity()).despawn();
}

pub fn update(
    mut ui_query: Query<&mut Text, With<HUDText>>,
    wave_manager: Res<WaveManager>,
    player_query: Query<(&PlayerStats, &Health), With<Player>>,
) {
    let Ok((stats, player_health)) = player_query.single() else {
        return;
    };

    for mut text in &mut ui_query {
        **text = format!(
            "Wave: {} | HP: {:.0}/{:.0} | {}",
            wave_manager.wave,
            player_health.value,
            stats.max_health,
            format!("Time: {:.1}s", wave_manager.wave_timer.remaining_secs())
        );
    }
}

pub fn update_level_up_indicator(
    mut commands: Commands,
    xp_query: Query<&PlayerExperience, With<Player>>,
    level_ups_query: Query<(Entity, Option<&Children>), With<HUDLevelUps>>,
    sprites: Res<HUDTextureAtlas>,
) {
    let Ok(player_xp) = xp_query.single() else {
        return;
    };
    let Ok((hud_level_ups, children)) = level_ups_query.single() else {
        return;
    };
    match children {
        None => {
            if player_xp.new_levels >= 1 {
                commands
                    .entity(hud_level_ups)
                    .with_child(HUDLevelUp::render(sprites));
            }
        }
        Some(icons) => {
            if icons.len() > player_xp.new_levels as usize {
                commands.entity(*icons.last().unwrap()).despawn();
            }
            if icons.len() < player_xp.new_levels as usize {
                commands
                    .entity(hud_level_ups)
                    .with_child(HUDLevelUp::render(sprites));
            }
        }
    };
}
