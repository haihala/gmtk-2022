use bevy::prelude::*;

#[derive(Debug)]
pub struct AssetHandles {
    pub font: Handle<Font>,
    pub colors: Colors,
    pub images: Images,
}

#[derive(Debug)]
pub struct Colors {
    pub basic_text: Color,
    pub highlight_text: Color,
    pub battle_tile: Color,
    pub dark_background: UiColor,
    pub gray_background: UiColor,
}

#[derive(Debug)]
pub struct Images {
    pub open_hand: Handle<Image>,
    pub dog: Handle<Image>,
    pub cowboy: Handle<Image>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load);
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles {
        font: asset_server.load("FiraSans-Bold.ttf"),
        images: Images {
            open_hand: asset_server.load("kammenAvoin.png"),
            dog: asset_server.load("dog.png"),
            cowboy: asset_server.load("cowboy.png"),
        },
        colors: Colors {
            basic_text: Color::WHITE,
            highlight_text: Color::RED,
            battle_tile: Color::BEIGE,
            dark_background: Color::rgb(0.13, 0.13, 0.13).into(),
            gray_background: Color::rgb(0.23, 0.23, 0.23).into(),
        },
    });
}
