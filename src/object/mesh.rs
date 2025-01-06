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

    pub fn quad() -> Vec<Mesh> {
        vec![
            Mesh {
                vertices: [
                    glm::vec4(-0.5, -0.5, 0.0, 1.0),
                    glm::vec4(0.5, -0.5, 0.0, 1.0),
                    glm::vec4(-0.5, 0.5, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
            Mesh {
                vertices: [
                    glm::vec4(0.5, 0.5, 0.0, 1.0),
                    glm::vec4(0.5, -0.5, 0.0, 1.0),
                    glm::vec4(-0.5, 0.5, 0.0, 1.0),
                ],
                normals: [
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                    glm::vec4(0.0, 0.0, 0.0, 1.0),
                ],
            },
        ]
    }

    pub fn empty() -> Self {
        Self {
            vertices: [glm::vec4(0.0, 0.0, 0.0, 0.0); 3],
            normals: [glm::vec4(0.0, 0.0, 0.0, 0.0); 3],
        }
    }

    pub fn cube() -> Vec<Mesh> {
        let mut meshes = vec![];
        let mut mesh = Mesh::quad();
        translate(&mut mesh, glm::vec3(0.0, 0.0, 0.5));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        let mut mesh = Mesh::quad();
        rotate(&mut mesh, 180., glm::vec3(1.0, 0.0, 0.0));
        translate(&mut mesh, glm::vec3(0.0, 0.0, -0.5));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        let mut mesh = Mesh::quad();
        rotate(&mut mesh, 90., glm::vec3(0.0, 1.0, 0.0));
        translate(&mut mesh, glm::vec3(0.5, 0.0, 0.0));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        let mut mesh = Mesh::quad();
        rotate(&mut mesh, 270., glm::vec3(0.0, 1.0, 0.0));
        translate(&mut mesh, glm::vec3(-0.5, 0.0, 0.0));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        let mut mesh = Mesh::quad();
        rotate(&mut mesh, 90., glm::vec3(1.0, 0.0, 0.0));
        translate(&mut mesh, glm::vec3(0.0, 0.5, 0.0));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        let mut mesh = Mesh::quad();
        rotate(&mut mesh, 270., glm::vec3(1.0, 0.0, 0.0));
        translate(&mut mesh, glm::vec3(0.0, -0.5, 0.0));
        mesh.iter().for_each(|m| meshes.push(m.clone()));

        meshes
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

pub fn rotate(meshes: &mut Vec<Mesh>, angle: f32, axis: glm::Vec3) {
    // degree to radian
    let angle = angle.to_radians();
    let rotation = glm::quat_angle_axis(angle, &axis);
    for mesh in meshes.iter_mut() {
        for vertex in mesh.vertices.iter_mut() {
            let position = glm::vec3(vertex.x, vertex.y, vertex.z);
            let rotated = glm::quat_rotate_vec3(&rotation, &position);
            vertex.x = rotated.x;
            vertex.y = rotated.y;
            vertex.z = rotated.z;
        }
    }
}

pub fn translate(meshes: &mut Vec<Mesh>, translation: glm::Vec3) {
    for mesh in meshes.iter_mut() {
        for vertex in mesh.vertices.iter_mut() {
            vertex.x += translation.x;
            vertex.y += translation.y;
            vertex.z += translation.z;
        }
    }
}

pub fn scale(meshes: &mut Vec<Mesh>, scale: glm::Vec3) {
    for mesh in meshes.iter_mut() {
        for vertex in mesh.vertices.iter_mut() {
            vertex.x *= scale.x;
            vertex.y *= scale.y;
            vertex.z *= scale.z;
        }
    }
}
