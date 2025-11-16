# Bevy Autoshooter Clone - Development Roadmap

## Project Context

- **Engine**: Bevy 0.17 with Rust nightly
- **Genre**: Auto-shooter roguelite
- **Core Mechanic**: Player controls character movement, weapons auto-aim and shoot at enemies
- **Progression**: Wave-based gameplay with increasing difficulty

## Current Implementation Status

### ✅ Completed Features

- Player movement (WASD/Arrow keys)
- Camera follow system with smooth lerp
- Wave system (timer-based with pause between waves)
- Enemy spawning system with warning indicators
- Three weapon types (MachineGun, Pistol, Shotgun)
- Weapons orbit player and auto-aim within sectors
- Collision detection (bullets vs enemies, player vs enemies)
- Basic HUD showing wave, XP, level, HP, enemy count
- Experience system with level-ups (health increase on level)
- Tile-based background with sprite atlas
- Component-based architecture using Bevy ECS

### 🚧 Currently Implementing

- **Upgrade Selection UI System** - Between-wave upgrade choices

## Feature Roadmap (Priority Order)

### 1. Upgrade System ⭐ HIGH PRIORITY

**Status**: In Development

- [ ] Upgrade card UI component
- [ ] 3 random upgrade choices between waves
- [ ] Upgrade types:
    - Weapon stat improvements (damage, fire rate, range)
    - Player stat improvements (speed, max HP, HP regen)
    - New weapon unlocks
    - Special modifiers (piercing, explosive, multishot)
- [ ] Persistent upgrade tracking during run
- [ ] Visual feedback for applied upgrades

### 2. Enemy Variety & AI ⭐ HIGH PRIORITY

- [ ] Fast enemy type (150% speed, 50% HP)
- [ ] Tank enemy type (200% HP, 50% speed, 2x damage)
- [ ] Splitter enemy (splits into 2-3 smaller enemies on death)
- [ ] Ranged enemy (shoots projectiles from distance)
- [ ] Boss enemy every 5 waves (unique mechanics)
- [ ] Enemy visual differentiation (different colors/shapes)
- [ ] Mini-boss spawns at wave 3, 6, 9, etc.

### 3. Visual Polish 🎨

- [ ] Particle system integration
    - Bullet impact particles
    - Enemy death explosions
    - Muzzle flashes
    - Blood splatter effects
- [ ] Screen shake on player damage
- [ ] Damage numbers (floating text on hit)
- [ ] Health bars above enemies
- [ ] Enemy spawn animations (fade-in or pop)
- [ ] Sprite animations for player/enemies
- [ ] Better weapon visuals (actual gun sprites)

### 4. Power-ups & Pickups

- [ ] XP orb drops from enemies (auto-collect in radius)
- [ ] Health pack drops (chance on enemy death)
- [ ] Temporary power-up system:
    - Shield bubble (absorb X damage)
    - Speed boost (30s duration)
    - Double damage (15s duration)
    - Magnet (increases XP collection radius)
- [ ] Pickup visual effects
- [ ] Power-up cooldown/duration UI

### 5. Advanced Difficulty Scaling

Current: Only spawn rate increases

- [ ] Scale enemy HP per wave: `ENEMY_HEALTH * (1.0 + wave * 0.15)`
- [ ] Scale enemy speed per wave: `ENEMY_SPEED * (1.0 + wave * 0.05)`
- [ ] Scale enemy damage per wave: `damage * (1.0 + wave * 0.10)`
- [ ] Increase max concurrent enemies
- [ ] Introduce new enemy types at specific waves
- [ ] Boss waves with special mechanics

### 6. Combat Enhancements

- [ ] Weapon special abilities/alt-fire
- [ ] Ultimate ability with charge meter
- [ ] Dash/dodge ability (cooldown-based)
- [ ] Combo system (consecutive kills = XP multiplier)
- [ ] Critical hit system (% chance for 2x damage)
- [ ] Status effects (slow, burn, poison)

### 7. Arena & Environment

- [ ] Procedural obstacle generation
- [ ] Destructible props
- [ ] Environmental hazards (lava pools, spikes)
- [ ] Multiple arena types/biomes
- [ ] Minimap showing enemy positions
- [ ] Arena boundary visual improvements

### 8. Audio Integration 🔊

- [ ] Background music system
- [ ] Dynamic music intensity based on enemy count
- [ ] Weapon firing sounds (unique per weapon type)
- [ ] Enemy hit/death sounds
- [ ] Player damage sound
- [ ] UI interaction sounds
- [ ] Power-up collection sound
- [ ] Level-up fanfare

### 9. Meta Progression

- [ ] Persistent save system (using bevy_save or serde)
- [ ] Currency system (coins from runs)
- [ ] Unlock shop:
    - Starting weapons
    - Permanent stat bonuses
    - New characters with unique stats
- [ ] Achievement system
- [ ] Stat tracking (total kills, highest wave, etc.)
- [ ] Leaderboard integration

### 10. UI/UX Improvements

- [ ] Main menu screen
- [ ] Pause menu (ESC key)
- [ ] Death/game over screen with stats
- [ ] Settings menu (volume, controls)
- [ ] Better wave start/end transitions
- [ ] Tutorial/help overlay
- [ ] Keybind customization

### 11. Advanced Features

- [ ] Multiple character classes
- [ ] Weapon synergy system
- [ ] Challenge modifiers (harder runs with better rewards)
- [ ] Daily challenge mode
- [ ] Co-op multiplayer (local or online)
- [ ] Weapon crafting/fusion system

## Technical Improvements

### Code Quality

- [ ] Improve error handling (reduce unwraps)
- [ ] Add documentation comments
- [ ] Refactor large systems into smaller functions
- [ ] Implement state machine for game states (Menu, Playing, Paused, GameOver)

### Performance

- [ ] Object pooling for bullets/enemies
- [ ] Spatial hashing for collision detection
- [ ] LOD system for distant entities
- [ ] Profiling and optimization pass

### Build & Distribution

- [ ] WASM build support
- [ ] CI/CD pipeline
- [ ] Itch.io deployment
- [ ] Cross-platform testing

## Design Notes

### Weapon Balance Philosophy

- Each weapon should have distinct feel and use case
- Machine Gun: High DPS, short range, low per-shot damage
- Pistol: Balanced, reliable, medium everything
- Shotgun: High burst, close range, slow fire rate

### Wave Progression Pacing

- Waves 1-5: Tutorial difficulty, introduce mechanics
- Waves 6-10: Ramp up, new enemy types
- Waves 11-15: First difficulty spike
- Wave 15+: Survival mode, maximum chaos

### Upgrade Rarity Tiers

- **Common** (60%): Small stat increases
- **Uncommon** (30%): Larger bonuses, minor modifications
- **Rare** (9%): Major modifications, new mechanics
- **Legendary** (1%): Game-changing unique effects

## Known Issues & Tech Debt

- [ ] Collision detection using simple radius (could use proper AABB)
- [ ] No object pooling (spawning creates new entities constantly)
- [ ] Camera clamping could be smoother at boundaries
- [ ] Wave timer doesn't account for remaining enemies
- [ ] No state management (just WaveState enum)

## Reference Links

- Repository: https://github.com/tbredzin/bevy_autoshooter_clone
- Bevy 0.17 Docs: https://docs.rs/bevy/0.17.0/bevy/
- Similar Games: Brotato, Vampire Survivors, 20 Minutes Till Dawn

## Next Session TODO

2. 🚧 Implement upgrade selection UI
3. Test upgrade system thoroughly
4. Plan enemy variety implementation

---
*Last Updated: 2025-11-16*
*Current Focus: Upgrade Selection UI System*