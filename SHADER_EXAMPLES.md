# Shader Plugin Examples

This document shows the differences between the `AnimatedShaderPlugin` and `DrawingShaderPlugin`.

## AnimatedShaderPlugin

The animated shader creates time-based visual effects without user interaction.

### Features:
- **Time-based animation**: Uses `time.elapsed_secs()` for continuous animation
- **Automatic effects**: Animated gradients, waves, and color cycles
- **No interaction**: Purely visual, no mouse input
- **Position**: Right side of screen (200, 200)

### Use case:
Perfect for background effects, ambient visuals, or decorative elements.

## DrawingShaderPlugin

The drawing shader responds to mouse input for interactive experiences.

### Features:
- **Mouse tracking**: Real-time mouse position and click detection
- **Interactive drawing**: Click and drag to create visual effects
- **Dynamic feedback**: Cursor indicators and ripple effects
- **Position**: Left side of screen (-200, 200)

### Use case:
Ideal for interactive art applications, drawing tools, or user engagement.

## Technical Comparison

| Aspect | AnimatedShaderPlugin | DrawingShaderPlugin |
|--------|---------------------|---------------------|
| **Uniforms** | `Vec4` (time + padding) | `Vec4` (time, mouse_pressed, mouse_pos) + `Vec2` (resolution) |
| **Input sources** | Time only | Time + Mouse position + Mouse clicks + Window resolution |
| **GPU data** | 16 bytes | 24 bytes |
| **Update frequency** | Every frame | Every frame (when mouse state changes) |
| **Shader complexity** | Medium | High |
| **Interaction** | None | Full mouse interaction |

## Usage Examples

### Running the Application
```bash
cargo run
```

### What you'll see:
1. **Menu screen**: Click "Play" to enter the game
2. **Game screen**: Two shader quads side by side
   - **Right quad**: Animated shader with time-based effects
   - **Left quad**: Interactive drawing shader

### Interacting with the Drawing Shader:
1. **Hover**: Move mouse over the left quad to see cursor indicator
2. **Draw**: Click and drag to create colorful drawing effects
3. **Observe**: Watch ripple animations and color changes

## Shader Structure

### AnimatedShaderMaterial (Simple)
```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AnimatedShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y,z,w = padding
}
```

### DrawingShaderMaterial (Interactive)
```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DrawingShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y = mouse_pressed, z,w = mouse_pos
    #[uniform(1)] 
    resolution: Vec2,
}
```

## WGSL Shader Differences

### Animated Shader
- Single uniform binding
- Time-based calculations only
- Static visual patterns

### Drawing Shader
- Two uniform bindings
- Mouse position calculations
- Dynamic brush and ripple effects
- Cursor feedback systems
