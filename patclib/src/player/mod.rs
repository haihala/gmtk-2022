use bevy::prelude::*;

// Implement functionality for different ammo types/currencies if there is time.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Player {
    pub resources: PlayerResources,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            resources: PlayerResources {
                stamina: 100,
                money: 5,
                bullets: 15,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct PlayerResources {
    pub stamina: u32,
    pub money: u32,
    pub bullets: u32,
}
impl PlayerResources {
    pub fn add(&mut self, other: PlayerResources) {
        self.stamina += other.stamina;
        self.money += other.money;
        self.bullets += other.bullets;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(Player::default());
}
