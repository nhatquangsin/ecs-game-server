use bevy_app::Plugin;

pub mod components;
pub mod systems;

#[derive(Default)]
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        todo!()
    }
}
