#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Mesh {
    pub vertices: [glm::Vec3; 3],
    pub normals: [glm::Vec3; 3],
    pub padding: [u32; 6],
}

impl Mesh {
    pub fn empty() -> Self {
        Self {
            vertices: [glm::vec3(0.0, 0.0, 0.0); 3],
            normals: [glm::vec3(0.0, 0.0, 0.0); 3],
            padding: [0; 6],
        }
    }

    #[allow(dead_code)]
    pub fn from_tobj(tobj: tobj::Model) -> Vec<Mesh> {
        let mesh = &tobj.mesh;
        let vertices = mesh
            .positions
            .chunks(3)
            .map(|v| glm::vec3(v[0], v[1], v[2]))
            .collect::<Vec<_>>();
        let normals = mesh
            .normals
            .chunks(3)
            .map(|n| glm::vec3(n[0], n[1], n[2]))
            .collect::<Vec<_>>();

        let indices = mesh.indices.chunks(3).map(|c| Mesh {
            vertices: [
                vertices[c[0] as usize],
                vertices[c[1] as usize],
                vertices[c[2] as usize],
            ],
            normals: if normals.is_empty() {
                [glm::vec3(0.0, 0.0, 0.0); 3]
            } else {
                [
                    normals[c[0] as usize],
                    normals[c[1] as usize],
                    normals[c[2] as usize],
                ]
            },
            padding: [0; 6],
        });
        indices.collect()
    }
}
