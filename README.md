# Pachinko and the cub

Game where you go through various encounters to escort a kid home. Written in rust with bevy. Submission for gmtk 2022 game jam.

Run with `cargo run`. Prebuilt executables will be available eventually.

## Premise
- Dice as Resources
  - When buying something, dice rolled to be currency
  - When playing anything, you need to use dice
    - Dice bullets
    - Dice batteries for devices

## Theme
  - Semi Scifi?
  - Western
    - Revolveri
      - Venäläinen ruletti'ish juttua
    - The Oregon Trail1

## Gameplay Loop
  - Encounter
    - Tee jotain (?)
    - Noppa pyörii
    - ???
    - Profit (?)

## Progression
### Chapters:

1. Departure
  - Merchants
  - Gamblers
  - A thug
2. Travel
  - Battles
  - Other adventurers asking for stuff
  - Wild non-combat encounters
  - Treasure
3. Arrival
  - Swindlers
    - Combat
  - A gang
  - Reward for dog

### Encounters
Encounter "Chase"
- "A young adventurer stops you during your journey and challenges you to a race"
  - Accept Challenge
    - 50% / 50% Win/Lose
    - Lose random amount of stamina
    - On win, the adventurer helps you on your next fight
  - Decline Challenge
    - Pushes you around.
    - Lose a round in your revolver
Encounter "Mistake"
- Yo, would you sell the dog for a handful?
  - No
    - I have to insist
    -> Battle
  - Yes
    -> Gain 1-2 gold
    - Shit... Forgot I had to take care of the dog
    -> Battle, but harder

Encounter "Trading"
- Would you like to browse my wares?
  - Yes please
    - Buy ammo
    - Buy batteries
    - Buy a mystery box
      - Is nothing
  - No thank you

## TODO:
- Visuals
- Battle system
- End state
- Beginning state
- Events
  - Structure
  - Content

## Priority
- Init
- Main loop
  - Non-combat encounter
  - Combat encounter
  - Travel between encounters
- End
