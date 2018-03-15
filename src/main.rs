extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Position {
    row: u32,
    col: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Leave {
    position: Position,
    point: Point,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    position: Position,
    point: Point,
    neighbours: Vec<Leave>,
}

fn main() {
    let nodes = build_nodes();
    let json = serde_json::to_string(&nodes).unwrap();
    println!("{}", json);
}

fn build_nodes() -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let leaves = build_leaves(5, 5);
    for (id, leave) in &leaves {
        let mut neighbours = get_neighbours(&leaves, &leave);
        let point = leave.point.clone();
        let position = leave.position.clone();
        let node: Node = Node { point, position, neighbours };
        nodes.insert(id.clone(), node);
    }
    nodes
}

fn build_leaves(r: u32, c: u32) -> HashMap<String, Leave> {
    let mut leaves: HashMap<String, Leave> = HashMap::new();
    for x in 0..r {
        for y in 0..c {
            leaves.insert(format!("{}-{}", x, y), build_leave(x, y));
        }
    }
    leaves
}

fn build_leave(x: u32, y: u32) -> Leave {
    let mut rng = thread_rng();
    let point: Point = Point {
        x: rng.gen_range((x as f32) - 0.5, (x as f32) + 0.5),
        y: rng.gen_range((y as f32) - 0.5, (y as f32) + 0.5),
    };
    let position: Position = Position { row: x, col: y };
    let leave: Leave = Leave { point, position };
    leave
}

fn get_neighbours(leaves: &HashMap<String, Leave>, leave: &Leave) -> Vec<Leave> {
    let mut neighbours = Vec::new();
    let indices: [i32; 3] = [-1, 0, 1];
    for x in indices.iter() {
        for y in indices.iter() {
            let pos_x: i32 = leave.position.row as i32 + x;
            let pos_y: i32 = leave.position.col as i32 + y;
            match leaves.get(&format!("{}-{}", pos_x, pos_y)) {
                Some(neighbour) => neighbours.push(neighbour.clone()),
                None => ()
            }
        }
    }
    neighbours
}
