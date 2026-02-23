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

Your goal is simple: **explore deeper into the Anthill, survive its dangers, and gather whatever treasures you can find**.

---

# 2. Quick Start

If you want to jump straight into the game, here’s everything you need:

| Action               | Keys |
|----------------------|------|
| Movement             | <kbd>w</kbd> <kbd>a</kbd> <kbd>s</kbd> <kbd>d</kbd> |
| Wait                 | <kbd>.</kbd> |
| Look Mode            | <kbd>l</kbd> |
| Ranged Combat Mode   | <kbd>r</kbd> |
| Inventory            | <kbd>i</kbd> (use mode), <kbd>SHIFT</kbd> + <kbd>d</kbd> (drop mode) |
| Unequip              | <kbd>SHIFT</kbd> + <kbd>w</kbd> (weapon), <kbd>SHIFT</kbd> + <kbd>a</kbd> (armor) |
| Descend              | Walk onto `<` or `>` |
| Attack               | Walk into an enemy |
| Pick up              | Walk over an item |


If you forget a command, press <kbd>SHIFT</kbd> + <kbd>h</kbd> to open the in‑game help window.

---

# 3. Getting Started

## 3.1 Basic Controls
You move your character using <kbd>w</kbd> <kbd>a</kbd> <kbd>s</kbd> <kbd>d</kbd>.

Movement also interacts with the world:
- Walk into an item to pick it up  
- Walk into an enemy to attack  
- Walk into a door (`+`) to open it  
- Walk onto stairs (`<` or `>`) to descend  

Press <kbd>.</kbd> to wait one turn.

## 3.2 Look Mode
Press <kbd>l</kbd> to enter Look Mode.
- A cursor appears on your character  
- Move the cursor with <kbd>w</kbd> <kbd>a</kbd> <kbd>s</kbd> <kbd>d</kbd>  
- Press <kbd>ENTER</kbd> to inspect the tile  

Look Mode does not consume turns.

## 3.3 Ranged Attack Mode
Press <kbd>r</kbd> to enter Ranged Attack Mode.
- Move the cursor with <kbd>w</kbd> <kbd>a</kbd> <kbd>s</kbd> <kbd>d</kbd>  
- Press <kbd>ENTER</kbd> to fire at the selected tile  

If you have a ranged weapon equipped and your target is valid and visible, the weapon fires.

---

# 4. User Interface

## 4.1 Worldspace (Main Game View)
The worldspace is where the game takes place. 

It displays:
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
Opened with <kbd>i</kbd>.  
Displays all items in your inventory, each assigned a letter from a–z.
Press the corresponding letter to **use** the item.

### Inventory (Drop Mode)
Opened with <kbd>SHIFT</kbd> + <kbd>d</kbd>.  
Same layout as Use Mode, but selecting an item **drops** it on the ground.

## 4.3 Character Info Panel
This panel is always visible and shows your character’s current status. 

It includes:

| Field            | Description |
|------------------|-------------|
| **HP**               | Current and maximum health |
| **Weapon / Armor**   | Currently equipped gear |
| **EXP**              | Experience points |
| **Round**            | Number of turns taken |
| **Coordinates**      | Your position in the dungeon |
| **Stats**            | Strength, Dexterity, Vitality, Perception |
| **Dungeon Floor**    | Current level of the Anthill |

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

### Perception (PER)
Represents awareness and sensory sharpness.  
Improves detection, vision and environmental awareness.

### Health (HP)
Your life total.
When HP reaches **0**, the run ends.

---

# 6. Exploration

The Anthill is infinite — but certain depth thresholds contain special challenge floors known as **Gauntlets**.

### Vision & Fog of War
You cannot see through walls. Your field of view updates as you move, revealing new parts of the dungeon.

---

# 7. Combat

## 7.1 Melee Combat
Move into an enemy to attack with your equipped weapon (or bare hands).

## 7.2 Ranged Combat
If you have a ranged weapon equipped, you can attack from a distance by using Ranged Attack Mode (<kbd>r</kbd>) to aim and shoot.

## 7.3 Damage & Mitigation
Stats, armor, and weapon types influence how effective your attacks are.

## 7.4 Randomness & Dice Rolls
Anthill uses a dice‑style RNG system.  
Some actions include a small random component.

This means:
- attacks can deal slightly more or less damage than expected  
- some hits may miss due to chance  

The system is designed to feel fair and consistent, while still adding unpredictability to each run.

---

# 8. Inventory & Equipment

## 8.1 Inventory
- Press <kbd>i</kbd> to open your inventory in use mode.
- Press <kbd>SHIFT</kbd> + <kbd>d</kbd> to open it in drop mode.
- Each item is assigned a letter from **a–z**, and you select items by pressing their letter.
- Inventory capacity is limited to 26 items — choose wisely.

## 8.2 Equipment
To equip an item (armor and weapon), simply **use** it from the inventory (<kbd>i</kbd>). 

You can unequip gear at any time:
- <kbd>SHIFT</kbd> + <kbd>w</kbd> → unequip weapon  
- <kbd>SHIFT</kbd> + <kbd>a</kbd> → unequip armor  

---

# 9. Items
Items are essential for survival in the Anthill.  
They come in several categories:

- **Weapons** — used for melee or ranged combat  
- **Armor** — reduces incoming damage  
- **Food** — restores health  
- **Potions** — temporary effects or healing; drinking too many in a short time can trigger an **overdose**

You can inspect items in Look Mode (<kbd>l</kbd>) or in the inventory (<kbd>i</kbd>).

---

# 10. Enemies
The Anthill is home to a variety of creatures, each with its own strengths and weaknesses. Enemies act only when you take a turn, but they will pursue and attack you once you are in their range of sight.

---

# 11. Death

Anthill follows classic roguelike tradition: 

**When you die, the run ends.**  

There are no second chances — but every run teaches you something new.

---

# 12. Controls & Commands Overview

Below is a concise summary of the most important controls and commands.

## Hotkeys

| Action                | Keys |
|-----------------------|------|
| Movement              | <kbd>w</kbd> <kbd>a</kbd> <kbd>s</kbd> <kbd>d</kbd> |
| Help | <kbd>SHIFT</kbd> + <kbd>h</kbd> |
| Wait                  | <kbd>.</kbd> |
| Look Mode             | <kbd>l</kbd> |
| Ranged Attack Mode    | <kbd>r</kbd> |
| Open inventory (use)  | <kbd>i</kbd> |
| Open inventory (drop) | <kbd>SHIFT</kbd> + <kbd>d</kbd> |
| Unequip weapon        | <kbd>SHIFT</kbd> + <kbd>w</kbd> |
| Unequip armor         | <kbd>SHIFT</kbd> + <kbd>a</kbd> |
| Equip item            | Use it from inventory (<kbd>i</kbd>) |
| Open door             | Walk into <kbd>+</kbd> |
| Use stairs            | Walk onto <kbd>&lt;</kbd> or <kbd>&gt;</kbd> |
| Attack                | Walk into an enemy |
| Pick up item          | Walk over an item |
| Start / Confirm       | <kbd>ENTER</kbd> |
| Quit game             | <kbd>SHIFT</kbd> + <kbd>q</kbd> |
| Close menus           | <kbd>ESC</kbd> |
| Open command prompt   | <kbd>:</kbd> |
| Run command           | <kbd>ENTER</kbd> |
| Cancel                | <kbd>ESC</kbd> |

---

## Player Commands

| Command       | Description |
|---------------|-------------|
| `quit`        | Quit the game |
| `help`        | List available commands |
| `playerinfo`  | Print player info to log |
| `legend` | Show map symbol list |

---

## Developer Commands

| Command | Description |
|---------|-------------|
| `maxstats` | Give player max stats |
| `maxequip` | Give player best equipment |
| `rngtest` | Test RNG engine |
| `suicide` | Set HP to zero |
| `teleport <x> <y>` | Teleport player to given coordinates |
| `give <item> <amount>` | Add item to player's inventory |
| `revealall` | Reveal entire map for 1 round |
| `noclip` | Walk through walls |
| `godmode` | Become immortal |

