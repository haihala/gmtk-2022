use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{battle::Battle, player::PlayerResources};

use super::{Encounter, EncounterDecision, EncounterPhase};

pub fn get_random_encounter() -> Encounter {
    let mut rng = rand::thread_rng();
    encounters().choose(&mut rng).unwrap().to_owned()
}

fn encounters() -> Vec<Encounter> {
    let wolves = Battle;

    vec![
        Encounter::from_phases(vec![
            EncounterPhase::Line("This is a test encounter"),
            EncounterPhase::Decision(EncounterDecision {
                prompt: "Would you like to fight?",
                options: vec![
                    (
                        "Hell yeah!",
                        Box::new(EncounterPhase::Battle(wolves.clone())),
                    ),
                    (
                        "Would rather not",
                        Box::new(EncounterPhase::Gain(
                            "You avoid the pointless fight, have a money",
                            PlayerResources {
                                money: 1,
                                ..default()
                            },
                        )),
                    ),
                ],
            }),
            EncounterPhase::Line("This encounter is over"),
        ]),
        Encounter::from_phases(vec![
            EncounterPhase::Line("Wolves attack!"),
            EncounterPhase::Battle(wolves.clone()),
        ]),
        Encounter::from_phases(vec![
            EncounterPhase::Line("A herd of electric sheep are barreling your way"),
            EncounterPhase::Decision(EncounterDecision {
                prompt: "What do you do?",
                options: vec![
                    (
                        "Attempt to count them",
                        Box::new(EncounterPhase::Lose(
                            "You start feeling drowsy, the sheep trample over you, costing you some time and energy",
                            PlayerResources { stamina: 20,..default()}
                        ))
                    ),
                    (
                        "Stand your ground",
                        Box::new(EncounterPhase::Battle(
                            Battle
                        ))
                    )
                ],

            }),
        ]),
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
                                            money: 3,
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
                                            money: 2,
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
                                            money: 2,
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
        ]),
    ]
}
