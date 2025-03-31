# Spacedogs

2D spaceshooter in Rust+Bevy. The dogs are still missing though

## Todo

- [x] Define plugins
  - [x] Player plugin
  - [x] Enemy Plugin
    - [ ] Spawn different enemy types
  - [ ] Projectile/Weapon Plugin
    - [ ] Change current weapon
    - [ ] Spawn weapon upgrades
  - [ ] Menu plugins
- [ ] Game state
  - [ ] Running
  - [ ] Paused -> Pause Menu
  - [ ] Start Menu
- [ ] Show Player health (maybe HUD Plugin?)
- [ ] Generate some assets

## Architecture & Systems

### Physics

physics.rs basically only contains collision detection.
On each collision a respective event is sent and consumed in world.rs and lower systems

### UI

ui.rs contains HUD and other things. It reacts to events send in world.rs

### World

All the action for the player happens here, basically all game logic.
Communication with UI/Physics through events sent by world.
