use bevy::app::Plugin;
use config::ConfigTag;
use config::config_tag::ConfigTag;

#[derive(ConfigTag)]
pub struct HandlesPlugin {}
impl Plugin for HandlesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}
