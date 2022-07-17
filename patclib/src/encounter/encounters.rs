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
        // test_encounter(wolves.clone()),
        wolf_fight(wolves),
        // electric_sheep(),
        // merchant(),
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
    let bullet_trade = EncounterPhase::Trade(
        "You get some ammo.",
        "Unfortunately your math rocks failed you today, the marchant gets huffy.",
        PlayerResources {
            money: "10".into(),
            ..default()
        },
        PlayerResources {
            bullets: 6,
            ..default()
        },
    );

    let food_trade = EncounterPhase::Trade(
        "You manage to gain some energy.",
        "You don't have enough money.",
        PlayerResources {
            money: "8".into(),
            ..default()
        },
        PlayerResources {
            stamina: 4,
            ..default()
        },
    );

    let mystery_box_trade = EncounterPhase::Loop(vec![
        EncounterPhase::Trade(
            "You gain a mystery box!",
            "You don't have enough money.",
            PlayerResources {
                money: "2d6".into(),
                ..default()
            },
            PlayerResources::default(),
        ),
        EncounterPhase::Line(
            "The box contains absolutely nothing, as you look back up, the merchant grins at you",
        ),
        EncounterPhase::Break,
    ]);

    let trade_options = vec![
        ("Bullets, 6 for 10 dice value!", Box::new(bullet_trade)),
        (
            "Food, 4 points of stamina for 8 dice value",
            Box::new(food_trade),
        ),
        ("Mystery box", Box::new(mystery_box_trade)),
    ];

    let trade_loop = EncounterPhase::Loop(vec![
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Do you wish to engage in trade?",
            options: vec![
                (
                    "Sure, why not.",
                    Box::new(EncounterPhase::Line(
                        "The merchant opens his coat to reveal trinkets and baubles of all sorts",
                    )),
                ),
                ("Maybe some other time.", Box::new(EncounterPhase::Break)),
            ],
        }),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Anything in particular?",
            options: trade_options,
        }),
        EncounterPhase::Line("Still interested?"),
    ]);

    Encounter::from_phases(vec![
        EncounterPhase::Line("You see a merchant travelling down the road"),
        EncounterPhase::Line("Interested in trade, are we?"),
        trade_loop,
        EncounterPhase::Line("The merchant continues his journey."),
    ])
}

pub fn game_start() -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("Welcome to the frontier. You are a cowboy in charge of a pupper"),
        EncounterPhase::Line("Use the arrow keys to navigate and space or enter to select"),
        EncounterPhase::Line("Adventure awaits"),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Start your adventure?",
            options: vec![
                (
                    "Simple yes",
                    Box::new(EncounterPhase::Line("That's the sprit!")),
                ),
                (
                    "Sarcastic yes",
                    Box::new(EncounterPhase::Line(
                        "Aren't you a rascal! Too bad you have no agency",
                    )),
                ),
                (
                    "No but actually yes",
                    Box::new(EncounterPhase::Line(
                        "Denying the quest, how heroic. Too bad there is a game to be played.",
                    )),
                ),
                (
                    "I got confused",
                    Box::new(EncounterPhase::Line(
                        "Yeah sometimes it be like that. Hopefully you'll figure it out",
                    )),
                ),
            ],
        }),
    ])
}

pub fn game_over() -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("Well, sometimes it that's how the dice fall. Can't win them all."),
        EncounterPhase::Line("Some people can't win them any."),
        EncounterPhase::Line(
            "Hopefully you found some enjoyment out of this. But I have to go now.",
        ),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Wait what?",
            options: vec![
                (
                    "Who are you?",
                    Box::new(EncounterPhase::Line("Does it matter?")),
                ),
                (
                    "Cheers",
                    Box::new(EncounterPhase::Line("It was fun while it lasted")),
                ),
                (
                    "*Nod and tip your hat",
                    Box::new(EncounterPhase::Line("Pardner, *tips back")),
                ),
                (
                    "I'm still confused, even moreso than before",
                    Box::new(EncounterPhase::Line("I sincerely hope you figure it out")),
                ),
            ],
        }),
    ])
}
