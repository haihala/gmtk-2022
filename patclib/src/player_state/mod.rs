use bevy::prelude::*;

enum Currency {
    Copper,
    Silver,
    Gold,
}

enum Ammo {
    BadAmmo,
    NormalAmmo,
    GoodAmmo,
}

pub struct PlayerState {
    stamina: u32,
    wallet: Wallet,
    ammo: Ammo,
    revolver: [Ammo; 6],
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, _: &mut App) {}
}
