use bevy::prelude::*;

use sword_melee::AppPlugin;

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run();
}

