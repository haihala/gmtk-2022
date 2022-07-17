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
        health: 10,
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

    let stamina_drain = EncounterPhase::Lose(
        "All this effort is exhausting",
        PlayerResources {
            stamina: 1,
            ..default()
        },
    );

    vec![
        test_encounter(wolves.clone(), stamina_drain.clone()),
        wolf_fight(wolves, stamina_drain.clone()),
        electric_sheep(stamina_drain.clone()),
        merchant(),
        chase(),
        town(),
    ]
}

fn town() -> Encounter {
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

    let battery_trade = EncounterPhase::Loop(vec![
        EncounterPhase::Trade(
            "It's a box of batteries. What did you expect?",
            "Dry on cash are we?",
            PlayerResources {
                money: "20".into(),
                ..default()
            },
            PlayerResources {
                batteries: "10d6".into(),
                ..default()
            },
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
        ("Battery box, 10 for 20 dice value", Box::new(battery_trade)),
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

    let general_store = EncounterPhase::Loop(vec![
        EncounterPhase::Line("You see a strangely familiar trader in his shack"),
        EncounterPhase::Line("Interested in trade, are we?"),
        trade_loop,
        EncounterPhase::Line("You leave, somehow feeling emptier than when you came in."),
    ]);

    let watering_hole = EncounterPhase::Loop(vec![
        EncounterPhase::Line("You head to the closest hole in the wall, looking for a drink"),
        EncounterPhase::Loop(vec![
            EncounterPhase::Decision(EncounterDecision {
                prompt: "10 dice points for a bit of relief",
                options: vec![("Yeah that seems fair", Box::new(EncounterPhase::Trade(
                "You feel like you were born yesterday",
                "Unfortunately, capitalism has been deeply rooted here and you need to pay",
                PlayerResources {
                    money: "10".into(),
                    ..default()
                },
                PlayerResources {
                    stamina: 20,
                    ..default()
                },
            ))), ("Eh, that's a bit steep", Box::new(EncounterPhase::Break))],
            }),
            EncounterPhase::Line("You could stay here forever, maybe just one more day"),
        ]),
        EncounterPhase::Break,
    ]);

    let town_loop = EncounterPhase::Loop(vec![EncounterPhase::Decision(EncounterDecision {
        prompt: "Where to?",
        options: vec![
            ("Get a drink", Box::new(watering_hole)),
            ("Stock up", Box::new(general_store)),
            ("head out", Box::new(EncounterPhase::Break)),
        ],
    })]);

    Encounter::from_phases(vec![
        EncounterPhase::Line("You arrive in a sleepy town"),
        town_loop,
        EncounterPhase::Line("This is a place worth visiting."),
    ])
}

fn chase() -> Encounter {
    let yes = EncounterPhase::Loop(vec![
        EncounterPhase::Line("You pick up the pace and pursue"),
        EncounterPhase::Battle(Battle::with(vec![Enemy {
            name: "Getaway car",
            ..default()
        }])),
        EncounterPhase::Gain(
            "You rummage the remains of the cart and find a fistful of dollars",
            PlayerResources {
                money: "10d6".into(),
                ..default()
            },
        ),
        EncounterPhase::Break,
    ]);

    Encounter::from_phases(vec![
        EncounterPhase::Line("A getaway car kicks up dust as it almost runs you over"),
        EncounterPhase::Decision(EncounterDecision {
            prompt: "Pursue",
            options: vec![
                ("Rob the robbers!", Box::new(yes)),
                (
                    "Let them get away",
                    Box::new(EncounterPhase::Line("You watch as the car leaves")),
                ),
            ],
        }),
    ])
}
fn test_encounter(wolves: Battle, stamina_drain: EncounterPhase) -> Encounter {
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
        stamina_drain,
    ])
}

fn wolf_fight(wolves: Battle, stamina_drain: EncounterPhase) -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::Line("Wolves attack!"),
        EncounterPhase::Battle(wolves),
        stamina_drain,
    ])
}

fn electric_sheep(stamina_drain: EncounterPhase) -> Encounter {
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
                            }, Weapon {
                                name: "Philosophical quandries",
                                damage: "1d6".into(),
                                range: 2,
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
                        "You start feeling drowsy, briefly dream of androids and the herd tramples you",
                        PlayerResources {
                            stamina: 20,
                            ..default()
                        },
                    )),
                ),
                (
                    "Spook them with your gun (6 bullets)",
                    Box::new(EncounterPhase::Trade(
                        "You rattle a few shots into the air, they seem unimpressed",
                        "You think of shooting, but don't have enough bullets. They seem to ignore you nonetheless.",
                        PlayerResources {
                            bullets: 6,
                            ..default()
                        },
                        PlayerResources {
                            stamina: -20,   // Negative because this is a ghetto way to drain health
                            ..default()
                        },
                    )),
                ),
            ],
        }),
        EncounterPhase::Line("The semimechanical bovine have been dealt with, but at what cost"),
        stamina_drain,
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
        EncounterPhase::Line("Onwards!"),
    ])
}

pub fn game_over() -> Encounter {
    Encounter::from_phases(vec![
        EncounterPhase::HighlightLine("Death comes to all. And did any of it matter in the end."),
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
                    Box::new(EncounterPhase::Line(
                        "Does it matter? Why? Do you really crave meaning that badly?",
                    )),
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
