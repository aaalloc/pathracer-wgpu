#![allow(dead_code)]
use std::rc::Rc;

use glm::{Vec3, Vec4};

use super::{Bounds, Mesh, Object, ObjectType, Sphere, AABB};

#[derive(Debug, Clone)]
struct BVHBuildNode {
    bounds: Bounds,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    split_axis: u32,
    first_obj_offset: u32,
    n_obj: u32,
    obj_type: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinearBVHNode {
    min: Vec4,
    max: Vec3,
    offset: u32,
    n_obj: u32,
    axis: u32,
    obj_type: u32,
    obj_offset: u32,
}

#[derive(Debug, Clone)]
pub struct BVH {
    root: Rc<BVHBuildNode>,
    pub total_nodes: u32,
}

impl BVHBuildNode {
    pub fn default() -> Self {
        BVHBuildNode {
            bounds: Bounds {
                min: Vec3::new(0.0, 0.0, 0.0),
                max: Vec3::new(0.0, 0.0, 0.0),
            },
            left: None,
            right: None,
            split_axis: 0,
            first_obj_offset: 0,
            n_obj: 0,
            obj_type: 0,
        }
    }
    pub fn init_leaf(first: u32, n: u32, bounds: Bounds) -> BVHBuildNode {
        BVHBuildNode {
            bounds,
            first_obj_offset: first,
            n_obj: n,
            left: None,
            right: None,
            split_axis: 0,
            obj_type: 0,
        }
    }

    pub fn init_interior(axis: u32, c0: Rc<BVHBuildNode>, c1: Rc<BVHBuildNode>) -> BVHBuildNode {
        let c0_bounds = c0.bounds;
        let c1_bounds = c1.bounds;

        let bounds = Bounds::union(c0_bounds, c1_bounds);

        BVHBuildNode {
            bounds,
            first_obj_offset: 0,
            n_obj: 0,
            left: Some(c0),
            right: Some(c1),
            split_axis: axis,
            obj_type: 0,
        }
    }
}

fn recursive_build(
    aabb: &Vec<AABB>,
    start: usize,
    end: usize,
    total_nodes: &mut u32,
    ordered_objects: &mut Vec<u32>,
) -> Rc<BVHBuildNode> {
    *total_nodes += 1;
    let mut bounds = Bounds::empty();
    for i in start..end {
        bounds = Bounds::union(bounds, aabb[i].bounds);
    }

    let n_obj = end - start;
    if n_obj == 1 {
        // create leaf node
        let first_obj_offset = ordered_objects.len() as u32;
        for i in start..end {
            ordered_objects.push(aabb[i].type_);
        }
        return Rc::new(BVHBuildNode::init_leaf(
            first_obj_offset,
            n_obj as u32,
            bounds,
        ));
    } else {
        let mut centroid_bounds = Bounds::empty();
        for i in start..end {
            centroid_bounds = Bounds::union(centroid_bounds, aabb[i].bounds);
        }
        let dim = centroid_bounds.maximum_extent();
        let mid = start + (end - start) / 2;
        if centroid_bounds.max[dim] == centroid_bounds.min[dim] {
            // create leaf node
            let first_obj_offset = ordered_objects.len() as u32;
            for i in start..end {
                ordered_objects.push(aabb[i].type_);
            }
            return Rc::new(BVHBuildNode::init_leaf(
                first_obj_offset,
                n_obj as u32,
                bounds,
            ));
        } else {
            // partition objects based on the midpoint
            return Rc::new(BVHBuildNode::init_interior(
                dim as u32,
                recursive_build(aabb, start, mid, total_nodes, ordered_objects),
                recursive_build(aabb, mid, end, total_nodes, ordered_objects),
            ));
        }
    }
}

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

fn print_tree(node: &Rc<BVHBuildNode>, depth: u32) {
    for _ in 0..depth {
        print!("  ");
    }
    println!(
        "Bounds: {:?}, n_obj: {}, split_axis: {}, first_obj_offset: {}, n_obj: {}",
        node.bounds, node.n_obj, node.split_axis, node.first_obj_offset, node.n_obj,
    );
    if node.n_obj == 0 {
        print_tree(&node.left.as_ref().unwrap(), depth + 1);
        print_tree(&node.right.as_ref().unwrap(), depth + 1);
    }
}

fn linearize_bvh(node: &Rc<BVHBuildNode>, linear_nodes: &mut Vec<LinearBVHNode>, offset: &mut u32) {
    let linear_node = LinearBVHNode {
        min: Vec4::new(node.bounds.min.x, node.bounds.min.y, node.bounds.min.z, 0.0),
        max: node.bounds.max,
        offset: *offset,
        n_obj: node.n_obj,
        axis: node.split_axis,
        obj_type: node.obj_type,
        obj_offset: node.first_obj_offset,
    };
    *offset += 1;
    linear_nodes.push(linear_node);
    if node.n_obj == 0 {
        linearize_bvh(node.left.as_ref().unwrap(), linear_nodes, offset);
        linearize_bvh(node.right.as_ref().unwrap(), linear_nodes, offset);
    }
}
// https://pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#LinearBVHNode::secondChildOffset
pub fn get_bvh(
    objects: &Vec<Object>,
    spheres: &Vec<Sphere>,
    meshes: &Vec<Mesh>,
) -> Vec<LinearBVHNode> {
    let aabbs = build_aabbs(objects, spheres, meshes);

    let mut ordered_objects = Vec::new();
    let mut total_nodes = 0;
    let root = recursive_build(
        &aabbs,
        0,
        aabbs.len(),
        &mut total_nodes,
        &mut ordered_objects,
    );

    // print_tree(&root, 0);
    let mut linear_nodes = Vec::new();
    let mut offset = 0;
    linearize_bvh(&root, &mut linear_nodes, &mut offset);

    for node in &linear_nodes {
        println!("{:?}", node);
    }
    linear_nodes
}
