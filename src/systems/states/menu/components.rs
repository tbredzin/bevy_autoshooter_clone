use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenuUI; // Root node of the entire main menu — despawned on OnExit.

#[derive(Component)]
pub struct TitleWord {
    pub phase: f32,
}
#[derive(Component)]
pub struct DividerSegment {
    pub index: usize,
}
#[derive(Component)]
pub struct AnimatedBorder {
    pub phase: f32,
}

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;
