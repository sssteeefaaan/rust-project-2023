use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::maze::Maze;

pub enum UnlockDoor{
    NoKey,
    NoDoor,
    Unlocked
}
#[derive(Clone, Eq, Debug)]
pub struct State{
    pub position: (usize, usize),
    pub keys: usize,
    pub doors_graph: HashMap<(usize,usize), HashSet<(usize, usize)>>,
    pub keys_set: HashSet<(usize, usize)>,
    pub shortest_path: Option<Vec<(usize, usize)>>
}

impl Default for State{
    fn default() -> Self {
        Self {
            position: (0, 0),
            keys: 0,
            doors_graph: HashMap::new(),
            keys_set: HashSet::new(),
            shortest_path: None
        }
    }
}

impl State{
    pub fn transfer_state(&self, new_position: &(usize, usize)) -> Option<Self>{
        let mut ret = self.clone();
        match ret.unlock_door(new_position) {
            UnlockDoor::NoKey => None,
            _ => {
                ret.position = new_position.clone();
                ret.collect_key(new_position);
                Some(ret)
            }
        }
    }

    pub fn move_to(&mut self, new_position: &(usize, usize)){
        self.position = new_position.clone();
    }

    pub fn create_from_maze(maze: &Maze) -> Self {
        Self{
            doors_graph: maze.get_doors_graph(),
            position: maze.start,
            keys_set: maze.get_keys_set(),
            keys: 0,
            shortest_path: None
        }
    }

    pub fn unlock_door(&mut self, position: &(usize, usize)) -> UnlockDoor{
        if self.doors_graph.get(&self.position).unwrap().contains(position){
            if self.keys > 0 {
                self.keys -= 1;
                self.doors_graph.get_mut(&self.position).unwrap().remove(position);
                UnlockDoor::Unlocked
            }else{
                UnlockDoor::NoKey
            }
        }else{
            UnlockDoor::NoDoor
        }
    }

    pub fn collect_key(&mut self, position: &(usize, usize)) {
        if self.keys_set.remove(position){
            self.keys += 1;
        }
    }
}

impl Hash for State{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.keys.hash(state);
    }
}

impl PartialEq for State{
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position && self.keys == other.keys;
    }
}