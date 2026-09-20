use bevy::{
    math::{
        Dir3, Vec2, Vec3,
        primitives::{Circle, Cuboid, Rectangle, RegularPolygon},
    },
    mesh::{PlaneMeshBuilder, SphereKind, SphereMeshBuilder},
};
use config::ConfigTag;
use config::config_tag::ConfigTag;
use serde::{Deserialize, Serialize};

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum MeshType {
    // 2D
    Circle {
        radius: f32,
    },
    Polygon {
        radius: f32,
        sides: u32,
    },
    Rectangle {
        width: f32,
        height: f32,
    },
    // 3D
    Sphere {
        radius: f32,
        subdivisions: u32,
    },
    Cuboid {
        size: Vec3,
    },
    Plane {
        normal: Vec3,
        size: Vec2,
        subdivisions: u32,
    },
}
impl Into<bevy::mesh::Mesh> for MeshType {
    fn into(self) -> bevy::mesh::Mesh {
        match self {
            MeshType::Circle { radius } => Circle::new(radius).into(),
            MeshType::Polygon { radius, sides } => RegularPolygon::new(radius, sides).into(),
            MeshType::Rectangle { width, height } => Rectangle::new(width, height).into(),
            MeshType::Sphere {
                radius,
                subdivisions,
            } => SphereMeshBuilder::new(radius, SphereKind::Ico { subdivisions }).into(),
            MeshType::Cuboid { size } => Cuboid::from_size(size).into(),
            MeshType::Plane {
                normal,
                size,
                subdivisions,
            } => {
                let normalised = normal.normalize();
                let Ok(dir) = Dir3::new(normalised) else {
                    panic!(
                        "The length the normal used to create a plane is {} instead of 1.",
                        normalised.length()
                    )
                };

                PlaneMeshBuilder::new(dir, size)
                    .subdivisions(subdivisions)
                    .into()
            }
        }
    }
}
