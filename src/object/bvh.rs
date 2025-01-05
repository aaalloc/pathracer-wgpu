use super::{Mesh, Object, ObjectType, Sphere, AABB};

fn build_aabbs(objects: &Vec<Object>, spheres: &Vec<Sphere>, meshes: &Vec<Mesh>) -> Vec<AABB> {
    let mut aabbs = Vec::new();
    for object in objects {
        let aabb = match ObjectType::from_u32(object.obj_type) {
            ObjectType::Sphere => spheres[object.id as usize].get_aabb(),
            ObjectType::Mesh => meshes[object.id as usize].get_aabb(),
        };
        aabbs.push(aabb);
    }
    aabbs
}

pub fn bvh_print(bvh: &Vec<AABB>, index: u32, depth: u32) {
    let node = &bvh[index as usize];
    for _ in 0..depth {
        print!("    ");
    }
    if depth > 0 {
        print!("└── ");
    }
    // vec3 to string
    println!("Min: {} {} {}", node.min.x, node.min.y, node.min.z);
    if node.left_child != u32::MAX {
        bvh_print(bvh, node.left_child, depth + 1);
    }
    if node.right_child != u32::MAX {
        bvh_print(bvh, node.right_child, depth + 1);
    }
}

pub fn get_bvh(objects: &Vec<Object>, spheres: &Vec<Sphere>, meshes: &Vec<Mesh>) -> Vec<AABB> {
    fn build_bvh_recursive(
        aabbs: &mut [AABB],
        indices: &[usize],
        axis: usize,
        bvh: &mut Vec<AABB>,
    ) -> u32 {
        if indices.is_empty() {
            return u32::MAX;
        }

        if indices.len() == 1 {
            let leaf_index = indices[0];
            let node_index = bvh.len() as u32;
            bvh.push(AABB {
                min: aabbs[leaf_index].min,
                max: aabbs[leaf_index].max,
                left_child: u32::MAX,
                right_child: u32::MAX,
            });
            return node_index;
        }

        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_by(|&i1, &i2| {
            aabbs[i1].min[axis]
                .partial_cmp(&aabbs[i2].min[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    aabbs[i1].max[axis]
                        .partial_cmp(&aabbs[i2].max[axis])
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        let mid = sorted_indices.len() / 2;
        let (left_indices, right_indices) = sorted_indices.split_at(mid);

        let mut combined_aabb = AABB {
            min: glm::vec3(f32::MAX, f32::MAX, f32::MAX),
            max: glm::vec3(f32::MIN, f32::MIN, f32::MIN),
            left_child: u32::MAX,
            right_child: u32::MAX,
        };
        for &index in indices {
            combined_aabb.min = glm::min2(&combined_aabb.min, &aabbs[index].min);
            combined_aabb.max = glm::max2(&combined_aabb.max, &aabbs[index].max);
        }

        let node_index = bvh.len() as u32;
        bvh.push(combined_aabb);

        let next_axis = (axis + 1) % 3;
        bvh[node_index as usize].left_child =
            build_bvh_recursive(aabbs, left_indices, next_axis, bvh);
        bvh[node_index as usize].right_child =
            build_bvh_recursive(aabbs, right_indices, next_axis, bvh);

        node_index
    }

    let mut aabbs = build_aabbs(objects, spheres, meshes);

    let mut bvh = Vec::new();
    let indices: Vec<usize> = (0..aabbs.len()).collect();
    build_bvh_recursive(&mut aabbs, &indices, 0, &mut bvh);

    // print bvh as a tree for debugging
    bvh_print(&bvh, 0, 0);
    println!("BVH: {:?}", bvh);
    bvh
}
