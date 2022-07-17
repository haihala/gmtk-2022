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
Encounter "Mistake"
- Yo, would you sell the dog for a handful?
  - No
    - I have to insist
    -> Battle
  - Yes
    -> Gain 1-2 gold
    - Shit... Forgot I had to take care of the dog
    -> Battle, but harder


## Battle System

3 linjaa.

Mahdollisia toimintoja 6
  - Move
    - Change lane
  - Shoot
    - 1 ammo
    - 1d6
  - Fan it
    - all ammo in revolver
    - 1d4 per ammo
  - Reload
    - revolver to full
  - Battery
    - 1d4 to 3 lanes
  - Cover

Esimerkkitaistelu:
sudet
3 kpl
6 hp per susi
Liikkuvat yhden per vuoro
Kun tulevat lähimmäiselle ruudulle, siirtyvät 2 ruutua taaemmas ja poistavat staminaa 1
Ottaessaan osuman, siirtyvät yhden ruudun sivuun.
-> Ei pysty fanaamaan kuoliaaksi

Thug
2 kpl
10 hp per thug
Liikkuvat yhden per vuoro
-> ottavat takapakkia osumasta, maks 3 päähän osumasta
