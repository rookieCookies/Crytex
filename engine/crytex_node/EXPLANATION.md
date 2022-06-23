# Crytex Node System

## Basic Functionality
- All nodes will have base methods that the user can override to implement their own functionality
    1. Ready()
        This method get's called when all of the nodes are instanced on the scene. This way you can store pointers to nodes on this method and then use them inside methods like Update()
    2. Update()
        This method get's called every frame
    3. Render()
        This method get's called whenever the node is getting re-rendered. This will probably not be too useful
- All nodes can have child nodes creating a complex architecture based on simple nodes
- Each node is only allowed to have a single script

## Under the hood
- A InsideUpdate()/InsideRender() method which will call the Update()/Render() method of the base node and then iterate through each child of the node and calling their Update()/Render() & InsideUpdate()/InsideRender() method, causing a recursive state until there is no children left
- All nodes are based on a a base node which implements the bare bones functionality to exist in the world
- All nodes have a under-the-hood pointer to the main engine allowing them to access any part of the engine without needing to carry    around a billion variables as arguments for each method
  
## Example
```
World
> Player Node --> Has Script "PlayerMovement" (Go To: line 28)
- > Sprite Renderer Node
- > Collision Node
- - > Rectangle Node
> Camera Node
```
### PlayerMovement Script
``` 
# The from keyword means it extends the functionality of Node2D
from Node2D

# If there is no value and no default value, after the ready function is over, throw an error
let sprite_renderer : SpriteRendererNode

# If no value is provided automatically call the default value of the provided type
let velocity : Vec2

# If the type has a ? at the end, it will allow this value to be null and thus not throw an error even if the value is null after ready
let camera_target : Camera?

function ready():
    sprite_renderer = child(0)
    camera_target = root.look_for("Camera")

function update():
    if Input.is_key_pressed(A):
        velocity.x = -50
        sprite_renderer.set_rotation(0)
    else if Input.is_key_pressed(D):
        velocity.x = 50
        sprite_renderer.set_rotation(1)
    else:
        velocity.x = 0
```
