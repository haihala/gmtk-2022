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
                        Box::new(EncounterPhase::Gain((
                            "You avoid the pointless fight, have a money",
                            PlayerResources {
                                money: 1,
                                ..default()
                            },
                        ))),
                    ),
                ],
            }),
            EncounterPhase::Line("This is a test encounter"),
        ]),
        Encounter::from_phases(vec![
            EncounterPhase::Line("Wolves attack!"),
            EncounterPhase::Battle(wolves.clone()),
        ]),
    ]
}
