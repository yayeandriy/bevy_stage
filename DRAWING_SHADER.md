# Drawing Shader Plugin

This plugin creates an interactive drawing shader that responds to mouse input. It's similar to the animated shader but allows users to interact with it using the mouse.

## Features

- **Mouse interaction**: Click and drag to draw on the shader
- **Visual feedback**: Shows a cursor ring when not drawing
- **Real-time effects**: Creates ripple effects and color trails based on mouse position
- **Background patterns**: Includes animated noise and grid patterns

## How it works

The drawing shader plugin:

1. **Tracks mouse position**: Converts screen coordinates to normalized UV coordinates (0.0 to 1.0)
2. **Detects mouse clicks**: Monitors left mouse button state
3. **Updates shader uniforms**: Passes time, mouse position, and click state to the GPU
4. **Renders effects**: The WGSL shader creates visual effects based on these inputs

## Usage

The plugin is automatically added to the game when you run it. You'll see:

- An interactive quad on the left side of the screen
- A cursor indicator when hovering over it
- Colorful drawing effects when clicking and dragging

## Shader uniforms

The shader receives these uniforms:
- `time`: Current elapsed time for animations
- `mouse_pos`: Normalized mouse position (0.0 to 1.0)
- `mouse_pressed`: Whether left mouse button is pressed (0.0 or 1.0)
- `resolution`: Screen resolution for aspect ratio correction

## Comparison with AnimatedShaderPlugin

| Feature | AnimatedShaderPlugin | DrawingShaderPlugin |
|---------|---------------------|---------------------|
| Input | Time only | Time + Mouse position + Mouse clicks |
| Interaction | None | Full mouse interaction |
| Position | Right side (200, 200) | Left side (-200, 200) |
| Effects | Animated waves and colors | Interactive drawing with ripples |
| Use case | Background decoration | Interactive art/drawing |

## Technical details

- Built using Bevy's Material2d system
- Uses WGSL shaders for GPU-accelerated rendering
- Efficiently updates only when mouse state changes
- Works on both native and web platforms
