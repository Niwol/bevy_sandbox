use bevy::prelude::*;

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
