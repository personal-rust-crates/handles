pub mod handles;
mod mesh_type;
pub mod plugin;
pub mod valid_key;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use ::config::wrapper::Wrapper;
    use bevy::{
        app::App,
        asset::Assets,
        color::Color,
        math::{vec2, vec3},
        mesh::Mesh,
        pbr::StandardMaterial,
    };
    use config::config_tag::Config;
    use serde::{Deserialize, Serialize};

    use crate::{handles::Handles, mesh_type::MeshType, plugin::HandlesPlugin};

    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
    enum ColorKey {
        Red,
        Green,
        Blue,
    }
    // impl ValidKey for ColorKey{}
    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
    enum MeshKey {
        Circle,
        Polygon,
        Rectangle,
        Sphere,
        Cuboid,
        Plane,
    }

    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
    enum StandardMaterialKey {
        RedMaterial,
        GreenMaterial,
        BlueMaterial,
    }

    type HandlesPluginType = HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>;
    type HandlesConfig = Wrapper<HandlesPluginType>;
    type HandlesResource = Handles<StandardMaterialKey, MeshKey>;

    fn colors() -> HashMap<ColorKey, Color> {
        HashMap::from([
            (ColorKey::Red, Color::hsla(0.0, 1.0, 1.0, 1.0)),
            (ColorKey::Green, Color::hsla(100.0, 1.0, 1.0, 1.0)),
            (ColorKey::Blue, Color::hsla(235.0, 1.0, 1.0, 1.0)),
        ])
    }
    fn meshes() -> HashMap<MeshKey, MeshType> {
        HashMap::from([
            (MeshKey::Circle, MeshType::Circle { radius: 1.0 }),
            (
                MeshKey::Polygon,
                MeshType::Polygon {
                    radius: 1.0,
                    sides: 6,
                },
            ),
            (
                MeshKey::Rectangle,
                MeshType::Rectangle {
                    width: 1.0,
                    height: 1.0,
                },
            ),
            (
                MeshKey::Sphere,
                MeshType::Sphere {
                    radius: 1.0,
                    subdivisions: 3,
                },
            ),
            (
                MeshKey::Cuboid,
                MeshType::Cuboid {
                    size: vec3(1.0, 1.0, 1.0),
                },
            ),
            (
                MeshKey::Plane,
                MeshType::Plane {
                    normal: vec3(0.0, 1.0, 0.0),
                    size: vec2(1.0, 1.0),
                    subdivisions: 2,
                },
            ),
        ])
    }
    fn standard_materials() -> HashMap<StandardMaterialKey, ColorKey> {
        HashMap::from([
            (StandardMaterialKey::RedMaterial, ColorKey::Red),
            (StandardMaterialKey::GreenMaterial, ColorKey::Green),
            (StandardMaterialKey::BlueMaterial, ColorKey::Blue),
        ])
    }
    fn handles_plugin() -> HandlesPluginType {
        HandlesPluginType::new(colors(), meshes(), standard_materials())
    }

    #[test]
    fn plugin_loads_from_json() {
        let actual = HandlesConfig::load_cfg(Path::new("handles.json")).config;
        let expected = HandlesConfig {
            config: handles_plugin(),
        }
        .config;

        assert_eq!(actual.colors, expected.colors);
        assert_eq!(actual.meshes, expected.meshes);
        assert_eq!(actual.standard_materials, expected.standard_materials);
    }

    #[test]
    fn plugin_populates_handles_resource() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>();
        app.init_resource::<Assets<StandardMaterial>>();

        app.add_plugins(handles_plugin());
        app.update();

        let handles = app.world().resource::<HandlesResource>();

        assert_eq!(handles.meshes.len(), meshes().len());
        assert_eq!(handles.standard_materials.len(), standard_materials().len());
    }
}
