use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    battle::{Battle, Enemy, Weapon},
    player::PlayerResources,
};

use super::{Encounter, EncounterDecision, EncounterPhase};

pub fn get_random_encounter() -> Encounter {
    let mut rng = rand::thread_rng();
    encounters().choose(&mut rng).unwrap().to_owned()
}

fn encounters() -> Vec<Encounter> {
    let wolf = Enemy {
        name: "",
        health: 20,
        weapons: vec![
            Weapon {
                name: "The Jaws that Bite",
                damage: "2d6".into(),
                ..default()
            },
            Weapon {
                name: "The Claws that Catch",
                damage: "2d6".into(),
                ..default()
            },
        ],
        ..default()
    };

    let wolves = Battle::with(vec![
        Enemy {
            name: "Gamma wolf",
            ..wolf.clone()
        },
        Enemy {
            name: "Beta wolf",
            ..wolf.clone()
        },
        Enemy {
            name: "Scientifically accurate alpha wolf",
            ..wolf.clone()
        },
    ]);

    vec![
        test_encounter(wolves.clone()),
        wolf_fight(wolves),
        electric_sheep(),
        merchant(),
    ]
}

fn test_encounter(wolves: Battle) -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("This is a test encounter"),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Would you like to fight?",
            options: vec![
                ("Hell yeah!", Box::new(EncounterPhase::Battle(wolves))),
                (
                    "Would rather not",
                    Box::new(EncounterPhase::Gain(
                        "You avoid the pointless fight, have a money",
                        PlayerResources {
                            money: "1d6".into(),
                            ..default()
                        },
                    )),
                ),
            ],
        }),
        EncounterPhase::Line("This encounter is over"),
    ])
}

fn wolf_fight(wolves: Battle) -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("Wolves attack!"),
        EncounterPhase::Battle(wolves),
    ])
}

fn electric_sheep() -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("A herd of electric sheep are barreling at you"),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "What to do in time of crisis?",
            options: vec![
                (
                    "Stand your ground",
                    Box::new(EncounterPhase::Battle(Battle::with(vec![
                        Enemy {
                            name: "Half sheep half machine",
                            health: 10,
                            weapons: vec![Weapon {
                                name: "Hoofs of steel",
                                damage: "1".into(),
                                ..default()
                            }],
                            ..default()
                        };
                        5
                    ]))),
                ),
                (
                    "Attempt to count them",
                    Box::new(EncounterPhase::Lose(
                        "You start feeling drowsy and the herd tramples you",
                        PlayerResources {
                            stamina: 20,
                            ..default()
                        },
                    )),
                ),
            ],
        }),
        EncounterPhase::Line("The semimechanical bovine have been dealt with, but at what cost"),
    ])
}

fn merchant() -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("You see a merchant travelling down the road"),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Would you like to see my wares?",
            options: vec![
                (
                    "Sure, why not.",
                    Box::new(EncounterPhase::Decision(EncounterDecision {
                        prompt: "What do you choose?",
                        options: vec![
                            (
                                "Ammo!",
                                Box::new(EncounterPhase::Trade(
                                    "You get some ammo.",
                                    "You don't have enough money.",
                                    PlayerResources {
                                        money: "3d6".into(),
                                        ..default()
                                    },
                                    PlayerResources {
                                        bullets: 6,
                                        ..default()
                                    },
                                )),
                            ),
                            (
                                "Food",
                                Box::new(EncounterPhase::Trade(
                                    "You manage to gain some energy.",
                                    "You don't have enough money.",
                                    PlayerResources {
                                        money: "2d6".into(),
                                        ..default()
                                    },
                                    PlayerResources {
                                        stamina: 4,
                                        ..default()
                                    },
                                )),
                            ),
                            (
                                "Mystery box",
                                Box::new(EncounterPhase::Trade(
                                    "You gain a mystery box!",
                                    "You don't have enough money.",
                                    PlayerResources {
                                        money: "2d6".into(),
                                        ..default()
                                    },
                                    PlayerResources { ..default() },
                                )),
                            ),
                        ],
                    })),
                ),
                (
                    "No thank you.",
                    Box::new(EncounterPhase::Line("The merchant continues his journey.")),
                ),
            ],
        }),
    ])
}
