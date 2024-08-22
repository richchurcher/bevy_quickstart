#![allow(dead_code)]

pub mod interaction;
pub mod palette;
mod widgets;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        interaction::OnPress,
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
