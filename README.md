# Anthill - A Modern Terminal Rogue-Like
_Welcome to the Anthill, an endless underground maze bustling with danger and treasure alike._

This is a student project, created for the module "Practical Course on Software Development" in Rust. Our inspiration were games like _Rogue_ or _NetHack_.

![Title Screen](/extra/anthill_main.jpg)

## Features
Explore an endless amount of rooms full of monsters and loot.
- Procedurally generated dungeon levels
- Handcrafted dungeon levels for a bit of extra challenge
- Exploration mechanics like Fog of War and Line of Sight
- Turn-based action and combat system
- Modernized UI layout in the Terminal (ratatui)
- Many different items and enemies to find

## Setup
> [!NOTE]
> - Requires a terminal to be played
> - Requires Rust 1.85

```bash
git clone https://github.com/ValeWeber2/anthill
cd anthill
cargo run
```

## How to Play (Basics)
The game revolves around combat and exploration. Move through the dungeon, collect powerful items, and try to survive.

| Key | Control |
| :---: | :--- |
| <kbd>WASD</kbd> | Move |
| <kbd>Shift + h</kbd> | Show Help |
| <kbd>i</kbd> | Open Inventory |
| <kbd>.</kbd> | Wait for 1 turn |
| <kbd>Shift + q</kbd>  | Quit the Game |

### Bumping into Things
The player interacts with the environment by "bumping" into it. This is done by using <kbd>WASD</kbd> to move into your target.
- Bump into an _item_ to pick it up
- Bump into an _enemy_ to attack them
- Bump into a _door_ to open it
- Bump into _stairs_ to walk to another level

### Inventory
Press <kbd>i</kbd> to open the inventory in use mode. You can then select an item using the alphabetical index displayed next to your item slots.

Press <kbd>Shift + d</kbd> to open the inventory in drop mode. This way you can drop items you no longer need.

![In-Game Screenshot](/extra/anthill_game.jpg)