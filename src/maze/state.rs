use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::maze::Maze;

#[derive(Clone, Eq)]
pub struct State{
    pub position: (usize, usize),
    pub keys: usize,
    pub doors_graph: HashMap<(usize,usize), HashSet<(usize, usize)>>,
    pub keys_set: HashSet<(usize, usize)>
}

impl Default for State{
    fn default() -> Self {
        Self {
            position: (0, 0),
            keys: 0,
            doors_graph: HashMap::new(),
            keys_set: HashSet::new()
        }
    }
}

impl State{
    pub fn transfer_state(&self, new_position: &(usize, usize)) -> (bool, Self){
        let mut ret = self.clone();
        let door_neighbours = ret.doors_graph.get_mut(&ret.position).unwrap();
        if door_neighbours.contains(new_position){
            if ret.keys == 0 {
                return (false, ret);
            }else{
                ret.keys -= 1;
                door_neighbours.remove(new_position);
            }
        }
        if ret.keys_set.remove(new_position){
            ret.keys += 1;
        }
        ret.position = new_position.clone();
        (true, ret)
    }

    pub fn create_from_maze(maze: &Maze)->Self{
        Self{
            doors_graph: maze.get_doors_graph(),
            position: maze.start,
            keys_set: maze.get_keys_set(),
            keys: 0
        }
    }
}

impl Hash for State{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.keys.hash(state);
        // for (_, v) in &self.doors_graph{
        //     for el in v{
        //         el.hash(state);
        //     }
        // }
        // for k in &self.keys_set{
        //     k.hash(state);
        // }
    }
}


impl PartialEq for State{
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position && self.keys == other.keys;
        // if self.position == other.position && self.keys == other.keys{
        //     for k in &self.keys_set{
        //         if other.keys_set.contains(k){
        //             return false;
        //         }
        //     }
        //     for (k, v) in &self.doors_graph{
        //         for f in v{
        //             if !other.doors_graph.get(k).unwrap().contains(f){
        //                 return false;
        //             }
        //         }
        //     }
        //     return true;
        // }
        // return false;
    }
}