# Project Structure

This document outlines the refactored folder structure of the Bevy game project.

## Overview

The project has been organized into logical modules for better maintainability and clarity:

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Main library file with GameState and GamePlugin
├── ui/                  # User Interface systems
│   ├── mod.rs           # UI module exports
│   ├── startup_menu.rs  # Main startup menu UI
│   └── drawing_menu.rs  # Drawing mode menu UI
├── gameplay/            # Game logic and mechanics  
│   ├── mod.rs           # Gameplay module exports
│   ├── player.rs        # Player entity and movement logic
│   ├── flock.rs         # Boid flocking simulation
│   └── actions/         # Input handling systems
│       ├── mod.rs       # Action module exports
│       └── game_control.rs # Input control mappings
├── rendering/           # Graphics and shader systems
│   ├── mod.rs           # Rendering module exports
│   ├── shader.rs        # Basic custom shader implementation
│   ├── animated_shader.rs # Interactive animated shader with mouse input
│   └── drawing_shader.rs  # Drawing/painting shader with mouse interaction
└── systems/             # Core engine systems
    ├── mod.rs           # Systems module exports  
    ├── loading.rs       # Asset loading system
    └── audio.rs         # Audio management system
```

## Module Responsibilities

### UI (`src/ui/`)
- **Purpose**: All user interface components and menus
- **Components**: 
  - Startup menu with navigation buttons
  - Drawing mode menu with controls
  - Shared UI components like ButtonColors

### Gameplay (`src/gameplay/`)
- **Purpose**: Game logic, entities, and mechanics
- **Components**:
  - Player entity management and movement
  - Boid flocking simulation with AI behaviors
  - Input action mapping and handling
  - Game state transitions

### Rendering (`src/rendering/`)
- **Purpose**: Graphics, shaders, and visual effects
- **Components**:
  - Custom shader materials and implementations
  - Interactive shader systems with mouse input
  - Animated visual effects
  - GPU uniform data management

### Systems (`src/systems/`)
- **Purpose**: Core engine functionality and resources
- **Components**:
  - Asset loading and management
  - Audio system with background music
  - Resource initialization

## Key Benefits

1. **Separation of Concerns**: Each module has a clear, distinct responsibility
2. **Easier Navigation**: Related functionality is grouped together
3. **Better Maintainability**: Changes to one system don't affect unrelated systems  
4. **Scalability**: New features can be added to appropriate modules
5. **Team Development**: Different developers can work on different modules

## GameState Flow

```
Loading → Startup → Drawing ↔ Playing
```

- **Loading**: Asset loading phase (systems module)
- **Startup**: Main menu (ui module) 
- **Drawing**: Interactive shader mode (rendering + ui modules)
- **Playing**: Gameplay with boids and player (gameplay module)

## Import Patterns

The new structure uses clear import paths:

```rust
// UI components
use crate::ui::{StartupMenuPlugin, DrawingMenuPlugin};

// Gameplay systems  
use crate::gameplay::{ActionsPlugin, FlockPlugin};

// Rendering systems
use crate::rendering::AnimatedShaderPlugin;

// Core systems
use crate::systems::{LoadingPlugin, InternalAudioPlugin};
```

## Dependencies Between Modules

- **UI** → Systems (for TextureAssets)
- **Gameplay** → Systems (for Actions, TextureAssets)  
- **Rendering** → None (self-contained with GameState)
- **Systems** → None (foundation layer)

This creates a clean dependency hierarchy with systems at the foundation.
