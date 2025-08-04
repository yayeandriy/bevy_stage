# Interactive Animated Shader - Clear Visual Feedback

## What You'll See

The animated shader now has **very clear visual feedback** for mouse interaction:

### 🎯 **Visual Indicators**

1. **White Ring (Mouse Hover)**
   - When you move your mouse over the animated shader quad
   - A white circular ring appears at your mouse position
   - Indicates the shader is tracking your mouse movement

2. **Black Circle (Mouse Click)**
   - When you **click and hold** the left mouse button
   - A **solid black circle** appears exactly where your mouse is
   - Makes it impossible to miss the interaction!

3. **Animated Circles**
   - Concentric circles radiate from your mouse position
   - Colors shift and animate based on distance from mouse
   - The entire animation pattern follows your mouse movement

### 🎮 **How to Test**

1. **Run the application**: `cargo run`
2. **Navigate to Drawing state**: Click "Play" in the menu
3. **Look at the RIGHT side** of the screen (the animated shader quad)
4. **Move your mouse** over the animated shader → See white ring
5. **Click and hold** left mouse button → See black circle appear
6. **Drag while holding** → Watch the black circle follow your mouse

### 🔧 **Technical Details**

- **Mouse Position**: Converted to normalized UV coordinates (0.0 to 1.0)
- **Visual Feedback**: 
  - Hover: `mouse_influence = 1.5` → White ring
  - Click: `mouse_influence = 3.0` → Black circle
- **Circle Sizes**:
  - Hover ring: 0.05 radius
  - Click circle: 0.08 radius with soft edges
- **Animation Center**: All concentric circles now originate from mouse position

### 📍 **Where to Look**

- **Animated Shader**: RIGHT side of screen at position (200, 200)
- **Drawing Shader**: LEFT side of screen at position (-200, 200)

Both shaders now respond to mouse input, but the animated shader has the most obvious visual feedback with the black circle on click!
