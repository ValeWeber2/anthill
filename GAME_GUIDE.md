# Anthill – Game Guide

## Table of Contents
1. [Introduction](#1-introduction)
2. [Quick Start](#2-quick-start)  
3. [Getting Started](#3-getting-started)  
   - [Basic Controls](#31-basic-controls)  
   - [Look Mode](#32-look-mode)  
   - [Ranged Attack Mode](#33-ranged-attack-mode)  
4. [User Interface](#4-user-interface)  
   - [Worldspace](#41-worldspace-main-game-view)  
   - [Menu Panel](#42-menu-panel)  
   - [Character Info Panel](#43-character-info-panel)  
5. [Player Stats](#5-player-stats)  
6. [Exploration](#6-exploration)  
7. [Combat](#7-combat)  
8. [Inventory & Equipment](#8-inventory--equipment)  
9. [Items](#9-items)  
10. [Enemies](#10-enemies)  
11. [Death](#11-death)
12. [Controls & Commands Overview](#12-controls--commands-overview)    

---

# 1. Introduction

Anthill is a turn-based roguelike played in a modern terminal UI.  
Every action you take — moving, attacking, using an item — advances the game by one turn.  
Enemies only act when you act, giving you time to think and plan.

Your goal is simple: explore deeper into the Anthill, survive its dangers, and gather whatever treasures you can find.

---

# 2. Quick Start

If you want to jump straight into the game, here’s everything you need:

```
Movement:               W A S D
Wait:                   .
Look Mode:              l
Ranged Combat Mode:     r
Inventory:              i (use mode), D (drop mode)
Unequip:                W (weapon), A (armor)
Descend:                Walk onto >
Attack:                 Walk into an enemy
Pick up:                Walk over an item
```

If you forget a command, press **H** to open the in‑game help window.

---

# 3. Getting Started

## 3.1 Basic Controls
You move your character using **WASD**.

Movement also interacts with the world:
- Walk into an item to pick it up  
- Walk into an enemy to attack  
- Walk into a door to open it  
- Walk onto `>` to descend  

Press `.` to wait one turn.

## 3.2 Look Mode
Press `l` to enter Look Mode.
- A cursor appears on your character  
- Move the cursor with **WASD**  
- Press **ENTER** to inspect the tile  

Look Mode does not consume turns.

## 3.3 Ranged Attack Mode
Press `r` to enter Ranged Attack Mode.
- Move the cursor with **WASD**  
- Press **ENTER** to fire at the selected tile  

If the target is valid and visible, your ranged weapon fires.

---

# 4. User Interface

## 4.1 Worldspace (Main Game View)
The worldspace is where the game takes place. It displays:
- The dungeon layout
- Your character
- Enemies and items
All movement, combat, and exploration happen here.

## 4.2 Menu Panel
The menu panel changes depending on the current mode. It has three states:

### Log Mode
Shows recent messages such as:
- Combat results
- Environmental messages
- Invalid actions
This is the default mode when no other menu is open.

### Inventory (Use Mode)
Opened with `i`.  
Displays all items in your inventory, each assigned a letter from a–z.
Press the corresponding letter to **use** the item.

### Inventory (Drop Mode)
Opened with `D` (Shift + d).  
Same layout as Use Mode, but selecting an item **drops** it on the ground.

## 4.3 Character Info Panel
This panel is always visible and shows your character’s current status. 
It includes:
- HP — current and maximum health
- Weapon / Armor — currently equipped gear
- EXP — experience points
- Round — number of turns taken
- Coordinates — your position in the dungeon
- Stats — STR, DEX, VIT, PER
- Dungeon Floor — current level of the Anthill

---

# 5. Player Stats
Anthill uses a simple but effective stat system. Your character has four core attributes:

### Strength (STR)
Represents physical power.
Influences melee combat effectiveness — stronger characters deal more damage with close‑range attacks.

### Dexterity (DEX)
Represents agility and precision.
Affects accuracy and finesse actions.

### Vitality (VIT)
Represents toughness and resilience.
Improves survivability.

### Persistence (PER)
Represents awareness and perception.
? Useful for mechanics like spotting enemies, improving vision, or interacting with the environment more effectively.

### Health (HP)
Your life total.
When HP reaches **0**, the run ends.

---

# 6. Exploration

### Vision & Fog of War
You cannot see through walls. Your field of view updates as you move, revealing new parts of the dungeon.

---

# 7. Combat

## 7.1 Melee Combat
Move into an enemy to attack with your equipped weapon (or bare hands).

## 7.2 Ranged Combat
If you have a ranged weapon equipped, you can attack from a distance by using Ranged Attack Mode (`r`) to aim and shoot.

## 7.3 Damage & Mitigation
Stats, armor, and weapon types influence how effective your attacks are.

---

# 8. Inventory & Equipment

## 8.1 Inventory
- Press `i` to open your inventory in use mode.
- Press `D` (Shift + d) to open it in drop mode.
- Each item is assigned a letter from **a–z**, and you select items by pressing their letter.
- Inventory capacity is limited to 26 items — choose wisely.

## 8.2 Equipment
You can unequip gear at any time:
- `W` → unequip weapon  
- `A` → unequip armor  

---

# 9. Items
Items are essential for survival in the Anthill. You will find weapons, armor, food, and other useful objects scattered throughout the dungeon.

## 9.1 Weapons
Weapons increase your damage output and determine whether you can attack at range.

### Rusty Sword
A simple melee weapon.
- Damage: 5  
- Crit Chance: 5%  
- Melee  
- Glyph: `/` (gray)

### Shortbow
A basic ranged weapon.
- Damage: 3  
- Crit Chance: 5%  
- Ranged  
- Glyph: `D` (gray)
Ranged weapons can be used in Ranged Attack Mode `r`, allowing you to strike enemies from a distance.

## 9.2 Armor
Armor reduces incoming damage and increases your survivability.

### Leather Armor
Light protection made from hardened leather.
- Mitigation: 2  
- Glyph: `A` (yellow)

## 9.3 Food
Food restores nutrition and helps keep your character alive.

### Cake
A small but tasty treat.
- Nutrition: 1  
- Glyph: `%` (red)

---

# 10. Enemies
The Anthill is home to a variety of creatures, each with its own strengths and weaknesses. Enemies act only when you take a turn, but they will pursue and attack you once you are in their range of sight.

## Goblin
- Glyph: `g` (green)  
Goblins are weak but nimble creatures that roam the Anthill.
- HP: 10  
- Damage: 2  
- Dodge: 10  
- Mitigation: 0  
They are not very dangerous alone, but can overwhelm careless players in groups.

## Funny Frog
- Glyph: `f` (light green)  
A strange, harmless creature that hops around the dungeon.
- HP: 5  
- Damage: 0  
- Dodge: 20  
- Mitigation: 0  
Funny Frogs pose no real threat, but their high dodge makes them surprisingly hard to hit.

## Orc
- Glyph: `O` (gray)  
Orcs are tough, aggressive fighters that appear deeper in the Anthill.
- HP: 20  
- Damage: 5  
- Dodge: 0  
- Mitigation: 2  
They hit hard and can withstand several blows, making them one of the more dangerous early enemies.

---

# 11. Death

Anthill follows classic roguelike tradition:

**When you die, the run ends.**  
There are no second chances — but every run teaches you something new.

---

# 12. Controls & Commands Overview

Below is a concise summary of the most important controls and commands.

## Hotkeys

### Movement
- **W A S D** — move  
- **.** — wait one turn  

### Interaction
- Walk into items to pick them up  
- Walk into enemies to attack  
- Walk into doors to open them  
- Walk onto `>` to descend  

### Inventory & Equipment
- **i** — open inventory (use mode)  
- **D** — open inventory (drop mode)  
- **W** — unequip weapon  
- **A** — unequip armor  

### Modes
- **l** — Look Mode (cursor inspect)  
- **r** — Ranged Attack Mode (cursor aim & shoot)  

### Game Control
- **ENTER** — start game / confirm  
- **Q** — quit game  
- **ESC** — close menus  

### Command Prompt
- **:** — open command prompt  
- **ENTER** — run command  
- **ESC** — cancel  

---

## Player Commands
- `quit` — quit the game  
- `help` — list available commands  
- `playerinfo` — print player info to log  

---

## Developer Commands (optional)
These are intended for debugging and testing:

- `maxstats` — give player max stats  
- `maxequip` — give best equipment  
- `rngtest` — test RNG engine  
- `suicide` — set HP to zero  
- `teleport <x> <y>` — teleport player  
- `give <item> <amount>` — spawn items  
- `revealall` — reveal entire map for 1 round  
- `noclip` — walk through walls  
- `legend` — show map symbol list  

---