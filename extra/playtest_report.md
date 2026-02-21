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

### Qualitative Results

| Survey | Questions | Result |
| :----- | :-------: | :----- |
| Clarity of user experience | 4, 5 | _4.33/5_ |
| Intuitiveness of controls | 7 | _4/5_ |
| Legibility of screen layout | 9 | _4.3/5_ |
| Clarity of world presentation | 10 | _3.83/5_ |
| Perception of difficulty | 14 | _2.8/5_ |
| Understandable reasons for game over | 15 | _4.1/5_ |
| Rewarding exploration | 16 | _4/5_ |
| Understanding of game's tools | 17 | _4.2/5_ |

Overall, the clarity of the game's presentation seems to be adequate with only slight misunderstandings scattered throughout.

The clarity of the world presentation was rated extremely high, with one vote, however, giving the lowest rating. The terminal-style of presentation might not be suited for all tastes, but among the ones who like it, it is clear and understandeable.

On the topic of difficulty, the game seems to be rated slightly below average difficulty. While steeply increasing difficulty is a feature of Rogue-likes, we knew that some numbers need to be tweaked.

### Qualitative Results

## Bugs Identified

## Discussion

### Solutions Implemented

### Plans for the Future