# Anthill - A Modern Terminal Rogue-Like
_Welcome to the Anthill, an endless underground maze bustling with danger and treasure alike._

This is a student project, created for the module "Practical Course on Software Development" in Rust. Our inspiration were games like _Rogue_ or _NetHack_.

![Title Screen](/extra/anthill_main.jpg)

> [!IMPORTANT]
> See the provided documents in [extra](/extra/) for our full documentation:
> - [technical_document](/extra/technical_document.md) contains our technical documentation of the project.
> - [playtest_report](/extra/playtest_report.md) contains a written report of our testing method.
> - [declaration_of_academic_integrity](/extra/declaration_of_academic_integrity.md) contains our statement for AI use in accordance with the institute's rules.
> - [game_guide.md](/extra/playtest) contains a written guide with tips and tricks on how to play the game.

## Features
Explore an endless amount of rooms full of monsters and loot
- Procedurally generated dungeon levels with infinite possibilities
- Handcrafted dungeon levels for a bit of extra challenge
- Exploration mechanics like Fog of War and Line of Sight
- Turn-based action and combat system
- A dice-based rng engine
- Modernized UI layout in the Terminal (ratatui)
- Many different items and enemies to find

## Setup
> [!NOTE]
> - Requires a terminal to be played
> - Requires Rust 1.85

```bash
git clone https://github.com/ValeWeber2/anthill
cd anthill
```

```bash
# Development Version (includes dev tools)
cargo run
# Production Version
cargo run --no-default-features
```

The codebase is fully documented. To read up on the documentation, be sure to run:
```bash
cargo doc
```

## How to Play (Basics)
> [!TIP]
> This section only describes the basics. [Find the full guide here.](/extra/GAME_GUIDE.md)

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

### User Interface
The central part of the UI is the **World Display**, where you can see the game's world. The `@` Symbol is your player character, which you'll move through rooms and corridors for exploration.

To the right is the **Menu Display**, which conditionally displays additional information about the game. This is where the game's **Log** is by default. In the log, you'll find descriptions of the events in the game. If you open your inventory, it will be displayed in the Menu Display as well.

On the bottom, you find the **Info Display**, which contains information about your character and the game. Keep your eyes on your character's Hit Points (HP).

> [!NOTE]
> Your terminal window should be big enough to display an area of 150x33 characters. If your terminal window is too small, you will only see a warning.
> 
> There are two ways of adjusting your terminal size:
> - Increase the window size in your operaiting system (e.g. maximizing your terminal)
> - Decrease the font size of your terminal (usually <kbd>Ctrl + -</kbd>)

![In-Game Screenshot](/extra/anthill_game.jpg)

## Known Issues
> [!TIP]
> We were able to fix this bug by increasing the iteration limit we put on the A* algorithm. This bug should now be incredibly rare, but it could technically still happen in exceedingly rare circumstances. Please let us know if you happen to come across it.

The *A\* Algorithm* for drawing corridors sometimes cannot find a path in certain situations, leading to the game panicking. This happens very rarely, about once every 50 seeds.

If the game crashes just as you walk down the stairs to a new level, please create an issue and send the game's log file so we can inspect the broken seeds.

> [!NOTE]
> Where to find the game's log files.
> 
> - **Linux** `$XDG_DATA_HOME/Anthill/` or `$HOME/.local/share/Anthill/`
> - **macOS** `$HOME/Library/Application Support/Anthll`
> - **Windows** `C:\Users\<YOURUSERNAME>\AppData\Local\Anthill`