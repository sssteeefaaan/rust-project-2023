pub mod field;
pub mod state;

use std::{fmt::Display, io::{Error, ErrorKind}, collections::{HashMap, HashSet, VecDeque}, thread, sync::{Mutex, Arc, mpsc}};

use crate::{utilities::read_binary};
use field::Field;

use self::state::State;


const DEFAULT_ROWS: usize = 6;
const DEFAULT_COLUMNS: usize = 9;

const DEFAULT_END: u8 = 0b11;
const DEFAULT_KEY: u8 = 0b1100;

const DEFAULT_DIRECTIONS: &'static [u8; 4] = &[0b1000, 0b100, 0b10, 0b1];

#[derive(Clone)]
pub struct Maze {
    pub fields: Vec<Vec<Field>>,
    pub dimensions: (usize, usize),
    pub start: (usize, usize),
    pub exits: HashSet<(usize, usize)>,
    pub state: Option<State>
}

impl Default for Maze{
    fn default() -> Self {
        Self{
            fields: Vec::new(),
            dimensions: (DEFAULT_ROWS, DEFAULT_COLUMNS),
            start: (0, 0),
            exits: HashSet::new(),
            state: None
        }
    }
}

impl Display for Maze{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("Dimensions: ({}, {})\n", self.dimensions.0, self.dimensions.1);
        output += format!("Exit count: ({})\n", self.exits.len()).as_str();
        output += "Field data:";
        for (row_index, row) in self.fields.iter().enumerate(){
            output += format!("\nRow[{row_index}]: ").as_str();
            for (_, element) in row.iter().enumerate(){
                output += format!("{}", element).as_str();
            }
            output += "\n";
        }
        f.write_str(&output)
    }
}

impl Maze{
    pub fn parse_from_file(bin_file_path: &String) -> Result<Self, Error>{
        match read_binary(bin_file_path) {
            Ok(data)=> Ok(Maze::parse_from_vector(data)?),
            Err(er)=> Err(er)
        }
    }

    pub fn parse_from_vector(data: Vec<u8>) -> Result<Self, Error>{
        let mut maze = Maze::default();
        if (data.len() as f32) < maze.dimensions.0 as f32 * maze.dimensions.1 as f32 * 1.5 {
            return Err(Error::new(ErrorKind::InvalidData, "Maze data is incomplete!"))
        }

        let mut new_row = true;
        let mut index:usize = 0;
        let mut doors: u8;
        let mut walls: u8;
        let mut key: u8;
        let mut end: u8;

        for i in 0..maze.dimensions.0{
            maze.fields.push(Vec::<Field>::new());
            for j in 0..maze.dimensions.1{
                let mut field = Field::default();
                field.position = (i, j);
                if new_row{
                    walls = data[index] >> 4;
                    doors = data[index] & 0b1111;
                    key = data[index + 1] >> 4;
                    end = data[index + 1] >> 4;
                    index += 1;
                    new_row = false;
                }else{
                    walls = data[index] & 0b1111;
                    doors = data[index + 1] >> 4;
                    key = data[index + 1];
                    end = data[index + 1];
                    index += 2;
                    new_row = true;
                }
                for j in 0..4{
                    field.walls[j] = (walls & DEFAULT_DIRECTIONS[j]) != DEFAULT_DIRECTIONS[j];
                    field.doors[j] = (doors & DEFAULT_DIRECTIONS[j]) == DEFAULT_DIRECTIONS[j];
                }
                if (key & DEFAULT_KEY) == DEFAULT_KEY{
                    field.key = true;
                }
                if (end & DEFAULT_END) == DEFAULT_END{
                    field.exit = true;
                    maze.exits.insert((i, j));
                }
                maze.fields[i].push(field);
            }
        }

        if maze.exits.len() < 1 {
            Err(Error::new(ErrorKind::InvalidInput, "Maze doesn't have an exit!"))
        }else{
            maze.state = Some(State::create_from_maze(&maze));
            Ok(maze)
        }
    }

    fn get_doors_graph(&self)->HashMap<(usize,usize), HashSet<(usize,usize)>>{
        let mut ret = HashMap::new();

        for (row_index, row) in self.fields.iter().enumerate(){
            for (col_index, field) in row.iter().enumerate(){
                ret.insert((row_index, col_index), HashSet::new());
                for (direction, door) in field.doors.iter().enumerate(){
                    if *door{
                        ret.get_mut(&(row_index, col_index)).unwrap().insert(
                            match direction{
                                0 => (row_index, col_index - 1),
                                1 => (row_index, col_index + 1),
                                2 => (row_index - 1, col_index),
                                _ => (row_index + 1, col_index)
                            }
                        );
                    }
                }
            }
        }

        return ret;
    }

    fn get_walls_graph(&self)->HashMap<(usize,usize), HashSet<(usize,usize)>>{
        let mut ret = HashMap::new();

        for (row_index, row) in self.fields.iter().enumerate(){
            for (col_index, field) in row.iter().enumerate(){
                ret.insert((row_index, col_index), HashSet::new());
                for (direction, wall) in field.walls.iter().enumerate(){
                    if !*wall{
                        ret.get_mut(&(row_index, col_index)).unwrap().insert(
                            match direction{
                                0 => (row_index, col_index - 1),
                                1 => (row_index, col_index + 1),
                                2 => (row_index - 1, col_index),
                                _ => (row_index + 1, col_index)
                            }
                        );
                    }
                }
            }
        }

        return ret;
    }

    pub fn get_keys_set(&self) -> HashSet<(usize, usize)>{
        let mut ret = HashSet::new();

        for (row_index, row) in self.fields.iter().enumerate(){
            for(col_index, field) in row.iter().enumerate(){
                if field.key{
                    ret.insert((row_index, col_index));
                }
            }
        }

        return ret;
    }

    #[allow(dead_code)]
    fn search_for_shortest_path(&self)->Option<Vec<(usize, usize)>>{
        if self.exits.contains(&self.start){ return Some(vec![self.start]); }

        let walls_graph = self.get_walls_graph();

        let state = State::create_from_maze(self);
        let mut state_history = Vec::new();
        state_history.push(state);

        let mut queue = VecDeque::new();
        queue.push_back(state_history);

        while !queue.is_empty(){
            let current_history = queue.pop_front().unwrap();
            let current_state = current_history.last().unwrap();
            let current_position = current_state.position;

            let neighbours = walls_graph.get(&current_position).unwrap();

            for node in neighbours{
                let potential_new_state = current_state.transfer_state(node);
                if let Some(new_state) = potential_new_state{
                    if !current_history.contains(&new_state){
                        let mut new_history = current_history.clone();
                        new_history.push(new_state);
                        if self.exits.contains(node){
                            return Some(
                                new_history.iter()
                                            .map(|state|{ state.position })
                                            .collect()
                                )
                        }
                        queue.push_back(new_history);
                    }
                }
            }
        }
        None
    }

    pub fn search_for_shortest_path_parallel(&self, state: State) -> Option<Vec<(usize, usize)>> {
        let start_position = state.position.clone();

        if self.exits.contains(&start_position){ return Some(vec![start_position]); }

        let walls_graph = self.get_walls_graph();

        let mut state_history = Vec::new();
        state_history.push(state);

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        queue.lock().unwrap().push_back(state_history);

        while !queue.lock().unwrap().is_empty(){
            let current_history = queue.lock().unwrap().pop_front().unwrap();
            let current_state = current_history.last().unwrap();
            let current_position = current_state.position;

            let neighbours = walls_graph.get(&current_position).unwrap();

            let mut threads = vec![];
            let (tx, rx) = mpsc::channel();

            for node in neighbours{
                let mut new_history = current_history.clone();
                let node_copy = node.clone();
                let exists = self.exits.clone();
                let queue_copy = queue.clone();
                let solutions = tx.clone();
                threads.push(thread::spawn(move ||{
                    let potential_new_state = new_history.last().unwrap().transfer_state(&node_copy);
                    if let Some(new_state) = potential_new_state{
                        if !new_history.contains(&new_state){
                            new_history.push(new_state);
                            if exists.contains(&node_copy){
                                let _ = solutions.send(new_history.iter().map(|state| { state.position.clone() } ).collect());
                            }
                            queue_copy.lock().unwrap().push_back(new_history);
                        }
                    }
                }));
            }

            for t in threads{
                t.join().unwrap();
            }

            for s in rx.try_iter(){
                return Some(s);
            }
        }
        None
    }

    pub fn get_state_mut<'a>(&'a mut self) -> &'a mut State{
        if self.state.is_none(){
            self.state = Some(State::create_from_maze(self));
        }
        self.state.as_mut().unwrap()
    }

    pub fn get_shortest_path(&mut self) -> Option<Vec<(usize, usize)>>{
        if self.state.is_none(){
            self.state = Some(State::create_from_maze(self));
        }
        if self.state.as_ref().unwrap().shortest_path.is_none(){
            self.state.as_mut().unwrap().shortest_path = self.search_for_shortest_path_parallel(self.state.as_ref().unwrap().clone())
        }
        self.state.as_ref().unwrap().shortest_path.clone()
    }
}

