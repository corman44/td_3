# 3D Tower Defense

## Design Patterns

### Event Usage vs. States Usage
Given the case of transitioning the camera from GameView to EditorView, it's possible to send an event that triggers the movement and it's possible to use a state transition to trigger the movement

What are the Pros and Cons of each approach?
- Events allow for dynamic trigger of systems (player attacking)
- State Transitions allow for setting up and altering the game from one state to the next, less flexible than events.

I focus on utilizing State Transitions everywhere I can (without adding a million states) and otherwise will utilze Events.