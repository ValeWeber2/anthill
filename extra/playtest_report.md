# Project Anthill Playtest Report
As we are nearing the end of the development of _Anthill_, we decided to conduct a final test. As in game development, system tests tend to not fully cover all cases, a play test with a survey was conducted.

The goal of the playtest is to evaluate what we have produced so far and where we would go from here.

## Methodology
We wrapped up any lose ends in our project and conducted one final merge of a complete game. This `1.0.0-beta` version was distributed to friends and colleagues to play around with.

In addition, we created a survey form (simply through Google Forms) to be filled out by playtesters. The playtesters had 1 week to fill out the survey before we would lock it and evaluate the results.

A separate bug submissino form was provided for reporting technical problems.

### Questions in the Playtester Form
1. How often do you play video games (Likert scale)
2. Have you played a Rogue-like game before? (Likert scale)
3. How long did you play our game? (Selection)
    - \<10 Minutes
    - 10-30 Minutes
    - 30-60 Minutes
    - \>1 Hour
4. Was it clear what you were supposed to do at the start? (Likert scale)
5. How easy was it to start playing? (Likert scale)
6. What confused you at first? (Text response)
7. Were the controls intuitive? (Likert scale)
8. Did you ever press a wrong key or struggle to find the right key for the right action? If so, what control did you have problems with? (Text response)
9. Was the Screen Layout readable? (Likert scale)
10. Was the world map easy to read visually? (Likert scale)
11. Did you read the message log? (Likert scale: 1-3)
12. Do you have any suggestions/wishes for the User Interface?
13. Out of interest: What is the deepest level you have reached in Anthill?
    - 0 (Tutorial)
    - 1
    - 2 (The Gauntlet)
    - 3-5
    - 6-9
    - 10 (The Second Gauntlet)
    - 11+
14. How did the game's difficulty feel? (Likert scale)
15. If you reached Game Over by dying, did you understand why your character died? (Likert scale)
16. Did exploration of the world feel rewarding?
17. Did you understand all of the tools that were available to your character (attacking, interacting, inventory, looking)?
18. What did you like most about the game?
19. What did you like least about the game?
20. Do you have any other suggestions? Is there anything else you would like to tell us?

Theses questions were aimed at certain concepts which we focused on. A clear user interface was one of our goals, since it's one of the weak points of NetHack (our inspiration for this project). Another goal was, of course, an engaging and rewarding game play experience. The rest of the questions are for profiling the playtester and general matters.

## Results
`7` people filled out our playtesting form. Some gave their thoughts in an unstructured follow-up interview.

### Participants
Of the number of participants, all stated to be regularly playing video games, although not all were familiar with the concept of Rogue-like games.

`57%` of the playtesters played the game for longer than 1 hour, which is great for our data collected.

### Quantitative Results

| Survey | Questions | Result |
| :----- | :-------: | :----- |
| Clarity of user experience | 4, 5 | _4.29/5_ |
| Intuitiveness of controls | 7 | _4/5_ |
| Legibility of screen layout | 9 | _4.29/5_ |
| Clarity of world presentation | 10 | _3.86/5_ |
| Perception of difficulty | 14 | _2.86/5_ |
| Understandable reasons for game over | 15 | _4.14/5_ |
| Rewarding exploration | 16 | _4/5_ |
| Understanding of game's tools | 17 | _4.14/5_ |

Overall, the clarity of the game's presentation seems to be adequate with only slight misunderstandings scattered throughout.

The clarity of the world presentation was rated extremely high, with one vote, however, giving the lowest rating. The terminal-style of presentation might not be suited for all tastes, but among the ones who like it, it is clear and understandeable.

On the topic of difficulty, the game seems to be rated slightly below average difficulty. While steeply increasing difficulty is a feature of Rogue-likes, we knew that some numbers need to be tweaked.

### Qualitative Results

The free-text responses provided deeper insight into specific points and player wishes.

#### Logging & Feedback

Many Players wished for more detailed log messages for events such as:
    - Critical hits
    - Picking up items
    - Level ups 
    - Potion effects 
    - Clear reason for death on Game Over
    
Several participants expressed that more explicit feedback would increase clarity ans improve the feeling of impact.

#### UI & Visual Communication

Recurring suggestions included:
    - Red HP bat when below 20% health
    - Color-coding items by rarity (common, rare, legendary)
    - Highlighting **STR** and **DEX** values more pominently 
    - Showing XP progress numerically (e.g., '50 / 100')
    - Displaying potion effects diectly 
    - Showing the currently equipped "empty hand" weapon (e.g., 'Fist(1d4)')
    - Tooltip for window size sttings (e.g., "make text smaller")
    
Players also requested that the inventory remain open after selecting items to reduce repetitive reopening.

#### Controls and Interactions

Although controls were generally rated positively, several friction points emerged:
    - Shift-based key combinations were not always obvious
    - Lack of arrow key movement was mentioned
    - Some players pressed incorrect keys when performing ranged attacks

These issues highlight the tension between traditional Rouge-like control schemes ans modern player expectations.

#### Combat & Balance

Several balancing concerns were mentioned repeatedly:
    - Bow was too strong (range and damage)
    - Enemy aggro range felt too large
    - Potion overdosing mechanics were too punishing
    - Diagonal enemy attacks were perceived as unfair

#### Progression & Content
While exploration was decribes as engaging and rewarding, players also noted:
    - Late-game content felt limited 
    - Loot could feel underwhelming 
    - Desire for additional envirmoental features (e.g., doors)

Overall sentiment toward the gameplay loop was positive, particually regarding ranged combat and dungeon progression.

## Bugs Identified

The following issues were reported either through the bug submission form or indirectly through survey feedback:

#### Technical Bugs

- Crash during level transitions
- Enemy spwaning directly on staircases
- Enemies stacking on the same tile
- Missing error handlind in 'drop_item'
- Overly aggressive potion overosing behavior 
- A* corridor generation occasionally failing 
- 0-sized room edge case in procedual generation 
- Right-click cintext menu interation (missing preventDefault / unbound R-click)

#### Gameplay Logic Issues

- Enemies spaawning in unfair positions 
- Overly large aggro range 
- Bow imbalance 
- Diagonal enemy attacks perceived as unfair
- Missing explict reason for death on Game Over screen

  
## Discussion

In this prject, the discussion focuses primarily on the design choices and system architecture rather than on the quantitive on the outcomes. A central aspect of our work was the development of a color system that communicates item properties in a clear and intuitive way. Instead of dinamically adapting color based on item rarity, we deliberatly decided on a fixed color scheme. This decision was motivated by the goal of ensuring consistency and recognizability for the player. A stable color system reduces cognitive load and allows useres to quickly associate specifc colors with certain item tyes or functions, which improves usability and overall player experience.

Another important design consideration concerned enemy behavior and combat mechanisms. In many games, enemies become aggressive when attacked from a distance, for example by using a bow. While this could increase realism and tactical depth, we consciously decided against implementing sucha reactive aggression system. Our goal was to keep the gameplay accessible and predictable, especially for less experienced players. Introducing complex enemy reactions might have increased frustration or made encounters feel inconsistent. Instead, we focused on simple and transperent behavior patterns that allow players to better understand and learn the mechanics over time.

Scability was also an important factor in our design process. The chosen approches allow the system to be extended in the future without fundamentally changing the existing visual language. New items or categories can be interegated while preserving the established structure. This contributes to long-term maintainability and reduces the risk of inconsistencies as the project evolves.

Overall, the discussion highlights that our implementations were guided by usability, calrity, and sustainability rather than purely technical optimazation. The final system reflects a conscious trade-of between flexibility and user-friendliness, with the intention of providing a coherent and intuatuive experience for players.

### Solutions Implemented

Based on the playtest results and internal discussion, the following improvements were implemented:

#### Combat & AI

- Removed NPC's ability to attack diagonally
- Adjusted the A* pathfinding algorithm to position NPCs orthgonally
- Fixed A* panics caused by low iteration limits
- Added fallback pathfinding for worst-case scenarios
- Fixed dexterity penalty bug in status effects

#### Logging & Feedback

- Added detailed logging for:
    - Critical hits
    - Level-ups
    - Item pickups
    - Commands
- Highlight HP in red when below 20%
- Show required XP for next level
- Display empty-hand weapon
- Widen info column for clarity
- Updated Game Over creen with dungeon floor, level and XP

#### UI & Controls

- Improved inventory management (inventory remains open after selection)
- Updated help menu and control descriptions
- Added Look Mode improving ranged targeting
- Enabled raw input mode
- Removed unintended mouse interactons in the TUI

#### Stability & Procedural Generation

- Prevented enemy spwaning on staircases
- Fixed enemy stacking
- Improved error handling in 'drop_item'
- Fixed room shrinking algorithm to enforce minimum size
- Added graceful handling for A* failure cases
- Fixed tutorial exit alignment

#### Content & Onboarding 

- Redesigned tutorial level
- Made tutorial optional
- Added step-by-step mechanic explanations
- Introduced new NPCs and items
- Added in-world tutorial signs


### Plans for the Future

Due to time and scope limitation, several larger improvements were categorized internally as **"Not in Budget"** and remain potential future work:

- Procedural doors and richer dungeon interaction
- Expanded late-game content
- Further balancing improvements
- Alternative and customizable input schemes
- Improved death explanation screen
- More robust procedural generation handling

These features require more extensive design and architectural adjustments and were therefore postponed beyond this scope of the current project.
