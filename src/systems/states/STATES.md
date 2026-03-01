# Current state flow implementation:

```
OnEnter(InWave)          → reset timers
  Update(InWave)         → gameplay, wave_timer ticks → writes NextState(UpgradeSelection)
OnExit(InWave)           → despawn enemies

OnEnter(UpgradeSelection) → spawn upgrade cards UI
  Update(UpgradeSelection) → handle_update_selection + apply_upgrade
                             → on final upgrade: writes NextState(BetweenWaves)
OnExit(UpgradeSelection)  → despawn upgrade UI

OnEnter(Shopping)    → spawn "Start Next Wave" button
  Update(Shopping)   → start_next_wave watches for input → writes NextState(InWave)
OnExit(Shopping)     → despawn button
```