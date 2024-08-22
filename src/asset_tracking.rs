use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash)
            .continue_to_state(Screen::Title)
            .load_collection::<AudioAssets>()
            .load_collection::<FontAssets>()
            .load_collection::<TextureAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio/end_level.ogg")]
    // pub end_level: Handle<AudioSource>,
    // #[asset(path = "audio/falling.ogg")]
    // pub falling: Handle<AudioSource>,
    // #[asset(path = "audio/jump.ogg")]
    // pub jump: Handle<AudioSource>,
    // #[asset(path = "audio/sheep_jump.ogg")]
    // pub sheep_jump: Handle<AudioSource>,
    // #[asset(path = "audio/sheep_falling.ogg")]
    // pub sheep_falling: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    // #[asset(path = "fonts/PixelifySans-Medium.ttf")]
    // pub pixelify_sans_medium: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/player.png")]
    pub player: Handle<Image>,
}
