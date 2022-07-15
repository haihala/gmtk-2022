use bevy::prelude::*:

enum currency {
    copper,
    silver,
    gold,
}

enum ammo {
    badAmmo,
    normalAmmo,
    goodAmmo,
}

pub struct playerState {
    stamina: u32,
    wallet: wallet,
    ammo: ammo,
    revolver: [ammo, 6]
};

pub struct playerStatePlugin;

imp Plugin for playerStatePlugin {
    fn build(&self, _: &mut App){}
}