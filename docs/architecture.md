# Architecture & Systems

This document provides detailed technical information on the game's architecture, systems, and implementation details.

## 📊 Release Details

**Current version:** v0.1.0  
A polished single-player release with complete combat, audio, UI, and game flow systems.

### ✅ Core Combat Features
- **Combat System**: Complete with hitboxes, hurtboxes, health management, and damage calculations
- **Jumping Mechanics**: Ground-based jumping with physics and collision detection
- **Blocking System**: Defensive blocking with invulnerability and reduced knockback
- **AI Boss System**: Fully implemented with unique behaviors for each programming bug type

### ✅ Visual & Audio Polish
- **Menu Background**: Custom animated background with smooth floating effects
- **UI/UX Polish**: All text in crisp white with increased font sizes for readability
- **Background Music**: Atmospheric looping music in menu with auto-play/stop
- **Victory/Defeat Music**: Unique victory and defeat stings with proper cleanup
- **Visual Effects**: Boss sprites, particle systems, color coding, and blocking feedback
- **Arena**: Contained fighting environment with boundary walls and ceiling
- **Victory/Defeat Screen**: Full-screen overlay with "BUG FIXED"/"SEGFAULT" text

### ✅ Game Systems
- **Game Flow**: Complete menu system, pause, and restart functionality
- **Audio Framework**: Full sound effects and background music integration
- **Pause System**: Multi-modal pause with UI button and keyboard shortcuts
- **Difficulty Scaling**: Easy/Normal/Hard modes with balanced progression
- **Character Variety**: All 5 character types used with unique boss mappings
- **Animation System**: Complete fighting game animations (blocking, victory poses, etc.)

## 🎯 Game Features Details

### Combat Mechanics
- **Jumping System**: Ground-based physics jumping with 600-unit upward impulse
- **Blocking Defense**: 0.5-second invulnerability window with reduced knockback (2-second cooldown)
- **Ground Detection**: Collision-based system prevents air jumping
- **Visual Blocking Feedback**: Blue tint appears when blocking successfully

### Boss AI Behaviors
- **Easy Difficulty**: 30% slower movement, 50% less frequent attacks
- **Normal Difficulty**: Balanced gameplay
- **Hard Difficulty**: 30% faster movement, 30% more frequent attacks
- **AI Jumping**: Bosses occasionally jump for unpredictable movement (2% chance when grounded)

### Visual System
- **Boss Sprites**: Each boss has unique colors and shapes representing their bug nature
- **Blocking Effects**: Players turn blue when blocking, "Attack blocked!" messages appear
- **Particle Effects**: 5 red particles spawn on hits with physics simulation
- **UI Labels**: Health bars show specific boss names instead of generic "BOSS"

### Pause System
- **Central Pause Button**: Gray button positioned 25% from top, centered
- **P Key Shortcut**: Keyboard pause during gameplay
- **Pause Menu**: Shows "PAUSED", "Escape to Resume", Resume and Exit buttons
- **State Preservation**: Game state frozen during pause

### Audio Framework
- **Background Music**: Atmospheric looping music in main menu with auto-play/stop
- **Victory/Defeat Music**: Unique 10-second victory sting and defeat music with auto-cleanup
- **Sound Effects**: Attack sounds, hit effects, and block feedback
- **Event System**: Combat events structured for sound playback
- **Audio Management**: State-aware music system with proper cleanup

### Character & Animation System
- **Full Character Variety**: All 5 character types (Player, Zombie, Adventurer, Female, Soldier) fully utilized
- **Unique Boss Characters**: Each boss type mapped to distinct character with unique animations
- **Fighting Game Animations**: Blocking, victory poses, falling, special attacks, and hurt states
- **Animation Priority System**: Blocking > Victory > Attacking > Jumping > Hurt > Walking > Idle
- **Asset Utilization**: Increased from 9.2% to 100% of available character animations

## 📦 Building for Distribution

### macOS
```bash
./bundle.sh
# Creates: Juice-Zero-Bugs-Given.zip
```

### Windows (PowerShell)
```powershell
./bundle_windows.ps1
# Creates: dist/Juice-Zero-Bugs-Given-Windows.zip
```

### Linux
```bash
./bundle_linux.sh
# Creates: dist/Juice-Zero-Bugs-Given-Linux.zip
