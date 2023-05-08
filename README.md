# CL Dungeon game

## About
A terminal based RPG game, where you play as a hero exploring dungeons.
### Locations
 - dungeon
 - shop
 - camp

### How to play
On startup game will ask if you want to resume saved game or create new. Resuming will try to find *save.dungeons* file, or asks you to provide path to savefile.
Starting in camp, game will ask for your action (`enter shop`, `explore dungeon`, `cook food`, `inspect inventory`, `save`)

#### Shop
Shopkeeper will offer you one for each (`head`, `body`, `legs`, `weapon`, `food`). Each with **stats** randomly distributed based on your **level**. You can also sell your gear for **buying cost** lowered by lost **durability**. `Food` has limited supply and you can choose how much you want to buy.

**Gear** in shop will reset each time you hit new **level**. **Food** resets after returning from dungeon.

#### Cooking
Based on cooked amount it will cost **money** and take **time**. Food that is being cooked will be temporarily lost. You will recieve 3x the **food** you cooked after time elapses.

note: You can **NOT** add any food if you are already cooking.

#### Exploration
You choose your path mid-rooms. Each room costs stamina to enter. Possible rooms:
- Small room (`small monster` or `small treasure` or both or nothing): 1 stamina
- Big room (`small monster` or `big monster` + `big treasure` or `big monster`): 2 stamina
- Treasure room (`big treasure` or `small treasure`): 5 stamina
- Escape passage (lets you return to camp without spending stamina): 0 stamina
- Final room (`big monster` + `big treasure` + return to camp for 0 stamina): 1 stamina

Before entering a room, you can:
 - eat (converts food to 3x stamina): 1 stamina
 - flee (costs stamina for each room you entered, leaves dungeon)

You **lose** if you run out of **stamina** even if you have **food** left.

#### Combat
Combat mode is entered upon encountering a monster during exploration. Killing the monster will revard you with `small treasure` and you will be able to collect remaining **trasure** in the room. You always start with 100 **HP** + **HP** from your items.

Monster always attacks **first**. The game will end if you die. If you survive, you get to choose your action:
 - Attack (attacks monster): 1 stamina
 - Eat (converts food to 3x stamina): 1 stamina
 - Flee (costs stamina for each room you entered, leaves dungeon): 5 stamina
 - Hide (small chance for monster to leave): 3 stamina

Process will loop until you **flee**, **win** or **die** or if the monster **leaves**.

#### Gear stats
Stats that you get from different gear parts:
 - `head` (`armor`, `damage`, heavy `luck`)
 - `body` (heavy `armor`, `damage`, `HP`, negative `speed`)
 - `legs` (`armor`, heavy `speed`, `HP`, `luck`)
 - `weapon` (heavy `damage`, `luck`)

#### Stats
Functionality of different stats:
 - `armor` decreases damage taken upon getting hit
 - `HP` increases your combat HP
 - `damage` increases your damage in combat
 - `luck` chance to increase your damage while attacking, to have monster leave or not appear at all
 - `speed` decreases stamina lost while fleeing from dungeon

#### Power budget
Is total power item has. Power budget is determined by level + 10 + random(-5, 5).
Once Power budget is calculated item is being given values for its stats by getting a random number from budget and decreasing budget by this number. **Cost** is calculated the same way as budget. **durability** is level + 10 + cost - budget
example:
> calculating budget for head piece, player level: 32
> budget: 32 + 10 + random(-5, 5) = 47
> cost: 32 + 10 + random(-5, 5) = 40
> durability: 32 + 10 + 40 - 47 = 35
> luck: 23 (is the first because it is marked as heavy)
> armor: 17
> damage: 7 (last gets whats left)

Monster power budget is also calculated based on player level. It is calculated as follows level + `type` + random(-5, 5).
`type` refers to it being **big** or **small** monster:
 - big: 50
 - small: 10

Budget is then distributed in the same way as item stats (monster has `HP`, `damage`, `armor`, order is important). Reward is budget / 5:
> small monster, player level: 50
> budget: 50 + type + random(-5, 5) = 61
> hp: 17
> damage: 30
> armor: 14
> reward: 61 / 5 = 12