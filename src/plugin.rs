use crate::valid_key::ValidKey;
use crate::{handles::Handles, mesh_type::MeshType};
use bevy::pbr::StandardMaterial;
use bevy::{
    app::{Plugin, PreStartup},
    ecs::system::ResMut,
};
use bevy::{asset::Assets, color::Color, mesh::Mesh};
use config::ConfigTag;
use config::config_tag::ConfigTag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(ConfigTag, Serialize, Deserialize, Clone)]
#[serde(bound = "ColorKey: ValidKey, MeshKey: ValidKey, StandardMaterialKey: ValidKey")]
pub struct HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>
where
    ColorKey: ValidKey,
    MeshKey: ValidKey,
    StandardMaterialKey: ValidKey,
{
    pub(crate) colors: HashMap<ColorKey, Color>,
    pub(crate) meshes: HashMap<MeshKey, MeshType>,
    pub(crate) standard_materials: HashMap<StandardMaterialKey, ColorKey>,
}
impl<ColorKey, MeshKey, StandardMaterialKey> HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>
where
    ColorKey: ValidKey,
    MeshKey: ValidKey,
    StandardMaterialKey: ValidKey,
{
    pub fn new(
        colors: HashMap<ColorKey, Color>,
        meshes: HashMap<MeshKey, MeshType>,
        standard_materials: HashMap<StandardMaterialKey, ColorKey>,
    ) -> Self {
        Self {
            colors,
            meshes,
            standard_materials,
        }
    }

    fn init_handles(
        config: Self,
        mut handle: ResMut<Handles<StandardMaterialKey, MeshKey>>,
        mut meshes_asset: ResMut<Assets<Mesh>>,
        mut standard_materials_asset: ResMut<Assets<StandardMaterial>>,
    ) {
        handle.with_meshes(&mut meshes_asset, config.meshes);
        handle.with_standard_materials(
            &mut standard_materials_asset,
            config.colors,
            config.standard_materials,
        );
    }
}
impl<ColorKey, MeshKey, StandardMaterialKey> Plugin
    for HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>
where
    ColorKey: ValidKey,
    MeshKey: ValidKey,
    StandardMaterialKey: ValidKey,
{
    fn build(&self, app: &mut bevy::app::App) {
        let me = self.clone();

        app.init_resource::<Handles<StandardMaterialKey, MeshKey>>()
            .add_systems(
                PreStartup,
                move |handle: ResMut<Handles<StandardMaterialKey, MeshKey>>,
                      meshes_asset: ResMut<Assets<Mesh>>,
                      standard_materials_asset: ResMut<Assets<StandardMaterial>>| {
                    Self::init_handles(me.clone(), handle, meshes_asset, standard_materials_asset)
                },
            );
    }
}
