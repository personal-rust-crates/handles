use crate::mesh_type::MeshType;
use crate::valid_key::ValidKey;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::ecs::resource::Resource;
use bevy::material::AlphaMode;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::utils::default;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Resource)]
pub struct Handles<StandardMaterialKey, MeshKey>
where
    StandardMaterialKey: ValidKey,
    MeshKey: ValidKey,
{
    pub(crate) standard_materials: HashMap<StandardMaterialKey, Handle<StandardMaterial>>,
    pub(crate) meshes: HashMap<MeshKey, Handle<Mesh>>,
}
impl<StandardMaterialKey, MeshKey> Handles<StandardMaterialKey, MeshKey>
where
    StandardMaterialKey: ValidKey,
    MeshKey: ValidKey,
{
    pub fn mesh(&self, key: &MeshKey) -> Handle<Mesh> {
        self.meshes[key].clone()
    }

    pub fn standard_material(&self, key: &StandardMaterialKey) -> Handle<StandardMaterial> {
        self.standard_materials[key].clone()
    }

    pub fn with_standard_materials<ColorKey>(
        &mut self,
        materials_asset: &mut Assets<StandardMaterial>,
        colors: HashMap<ColorKey, Color>,
        materials: HashMap<StandardMaterialKey, ColorKey>,
    ) where
        ColorKey: Eq + Hash + Sync + Send + 'static,
    {
        for (standard_material_key, color_key) in materials {
            let Some(color) = colors.get(&color_key) else {
                continue;
            };

            self.standard_materials.insert(
                standard_material_key,
                materials_asset.add(
                    // StandardMaterial::from_color(*color)
                    StandardMaterial {
                        base_color: *color,
                        alpha_mode: AlphaMode::Blend,
                        perceptual_roughness: 0.85,
                        reflectance: 0.04,
                        metallic: 0.0,
                        ..default()
                    },
                ),
            );
        }
    }

    pub fn with_meshes(
        &mut self,
        meshes_asset: &mut Assets<Mesh>,
        meshes: HashMap<MeshKey, MeshType>,
    ) {
        for (mesh_key, mesh_type) in meshes {
            self.meshes.insert(mesh_key, meshes_asset.add(mesh_type));
        }
    }
}
impl<StandardMaterialKey, MeshKey> Default for Handles<StandardMaterialKey, MeshKey>
where
    StandardMaterialKey: ValidKey,
    MeshKey: ValidKey,
{
    fn default() -> Self {
        Self {
            standard_materials: HashMap::new(),
            meshes: HashMap::new(),
        }
    }
}
