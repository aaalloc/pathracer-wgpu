#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
// TODO: For the moment, vec4 for padding, include manually
pub struct Mesh {
    pub vertices: [glm::Vec4; 3],
    pub normals: [glm::Vec4; 3],
}

impl Mesh {
    #[allow(dead_code)]
    pub fn square() -> Vec<Mesh> {
        vec![
            Mesh {
                vertices: [
                    glm::vec4(-0.5, -0.5, 0.0, 1.0),
                    glm::vec4(0.5, -0.5, 0.0, 1.0),
                    glm::vec4(-0.5, 0.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(0.5, 0.0, 0.0, 1.0),
                    glm::vec4(0.5, -0.5, 0.0, 1.0),
                    glm::vec4(-0.5, 0.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        ]
    }

    pub fn left_wall_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 1.0, 1.0),
                    glm::vec4(0.0, 1.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(0.0, 1.0, 1.0, 1.0),
                    glm::vec4(0.0, 0.0, 1.0, 1.0),
                    glm::vec4(0.0, 1.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn right_wall_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(1.0, 0.0, 0.0, 1.0),
                    glm::vec4(1.0, 1.0, 0.0, 1.0),
                    glm::vec4(1.0, 0.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(1.0, 1.0, 0.0, 1.0),
                    glm::vec4(1.0, 1.0, 1.0, 1.0),
                    glm::vec4(1.0, 0.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn quad(mid: glm::Vec3, size: f32) -> Vec<Mesh> {
        let half_size = size / 2.0;
        vec![
            Mesh {
                vertices: [
                    glm::vec4(mid.x - half_size, mid.y - half_size, mid.z, 1.0),
                    glm::vec4(mid.x + half_size, mid.y - half_size, mid.z, 1.0),
                    glm::vec4(mid.x - half_size, mid.y + half_size, mid.z, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(mid.x + half_size, mid.y + half_size, mid.z, 1.0),
                    glm::vec4(mid.x + half_size, mid.y - half_size, mid.z, 1.0),
                    glm::vec4(mid.x - half_size, mid.y + half_size, mid.z, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        ]
    }

    pub fn ceiling_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(0.0, 1.0, 0.0, 1.0),
                    glm::vec4(0.0, 1.0, 1.0, 1.0),
                    glm::vec4(1.0, 1.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(1.0, 1.0, 0.0, 1.0),
                    glm::vec4(0.0, 1.0, 1.0, 1.0),
                    glm::vec4(1.0, 1.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn floor_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 1.0, 1.0),
                    glm::vec4(1.0, 0.0, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(1.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 1.0, 1.0),
                    glm::vec4(1.0, 0.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn back_wall_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(0.0, 0.0, 1.0, 1.0),
                    glm::vec4(0.0, 1.0, 1.0, 1.0),
                    glm::vec4(1.0, 0.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(1.0, 0.0, 1.0, 1.0),
                    glm::vec4(0.0, 1.0, 1.0, 1.0),
                    glm::vec4(1.0, 1.0, 1.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn ceiling_light_quad() -> (Mesh, Mesh) {
        (
            Mesh {
                vertices: [
                    glm::vec4(0.25, 0.99, 0.25, 1.0),
                    glm::vec4(0.25, 0.99, 0.75, 1.0),
                    glm::vec4(0.75, 0.99, 0.25, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(0.75, 0.99, 0.25, 1.0),
                    glm::vec4(0.25, 0.99, 0.75, 1.0),
                    glm::vec4(0.75, 0.99, 0.75, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        )
    }

    pub fn cube(mid: glm::Vec3, size: f32) -> Vec<Mesh> {
        let half_size = size / 2.0;
        let mut meshes = Vec::new();
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(0.0, 0.0, half_size), size));
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(0.0, 0.0, -half_size), size));
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(0.0, half_size, 0.0), size));
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(0.0, -half_size, 0.0), size));
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(half_size, 0.0, 0.0), size));
        meshes.extend_from_slice(&Mesh::quad(mid + glm::vec3(-half_size, 0.0, 0.0), size));
        meshes
    }

    pub fn empty() -> Self {
        Self {
            vertices: [glm::vec4(0.0, 0.0, 0.0, 0.0); 3],
            normals: [glm::vec4(0.0, 0.0, 0.0, 0.0); 3],
        }
    }

    #[allow(dead_code)]
    pub fn from_tobj(tobj: tobj::Model) -> Vec<Mesh> {
        let mesh = &tobj.mesh;
        println!("Positions: {:?}", mesh.positions.len());
        let vertices = mesh
            .positions
            .chunks(3)
            .map(|c| glm::vec4(c[0], c[1], c[2], 0.0))
            .collect::<Vec<_>>();
        let indices = mesh.indices.chunks(3).map(|c| Mesh {
            vertices: [
                vertices[c[0] as usize],
                vertices[c[1] as usize],
                vertices[c[2] as usize],
            ],
            normals: [glm::vec4(0.0, 0.0, 0.0, 0.0); 3],
        });
        indices.collect()
    }
}
