pub mod field;
pub mod state;

use std::{fmt::Display, io::{Error, ErrorKind}, collections::{HashMap, HashSet, VecDeque}, thread::spawn, sync::mpsc, time::Instant};

use crate::utilities::read_binary;

use self::{field::Field, state::State};

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
                        let neighbour = 
                        match direction{
                                0 if col_index > 0 => Some((row_index, col_index - 1)),
                                1 if col_index + 1 < self.dimensions.1 => Some((row_index, col_index + 1)),
                                2 if row_index > 0 => Some((row_index - 1, col_index)),
                                3 if row_index + 1 < self.dimensions.0 => Some((row_index + 1, col_index)),
                                _ => None
                        };
                        if neighbour.is_some(){
                            ret.get_mut(&(row_index, col_index)).unwrap().insert(neighbour.unwrap());
                        }
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
                        let neighbour = 
                        match direction{
                            0 if col_index > 0 => Some((row_index, col_index - 1)),
                            1 if col_index + 1 < self.dimensions.1 => Some((row_index, col_index + 1)),
                            2 if row_index > 0 => Some((row_index - 1, col_index)),
                            3 if row_index + 1 < self.dimensions.0 => Some((row_index + 1, col_index)),
                            _ => None
                        };
                        if neighbour.is_some(){
                            ret.get_mut(&(row_index, col_index)).unwrap().insert(neighbour.unwrap());
                        }
                    }
                }
            }
        }

        return ret;
    }

    #[allow(dead_code)]
    pub fn get_direct_neighbours(&self, position: &(usize, usize))->HashSet<(usize, usize)>{
        let mut ret = HashSet::new();
        for (i, w) in self.fields[position.0][position.1].walls.iter().enumerate(){
            if !*w{
                let res = match i {
                    0 if position.1 > 0 => Some((position.0, position.1 - 1)),
                    1 if position.1 + 1 < self.dimensions.1 => Some((position.0, position.1 + 1)),
                    2 if position.0 > 0 => Some((position.0 - 1, position.1 )),
                    3 if position.0 + 1 < self.dimensions.0 => Some((position.0 + 1, position.1)),
                    _ => None
                };
                if res.is_some(){
                    ret.insert(res.unwrap());
                }
            }
        }
        ret
    }

    #[allow(dead_code)]
    pub fn get_door_neighbours(&self, position: &(usize, usize))->HashSet<(usize, usize)>{
        let mut ret = HashSet::new();
        for (i, d) in self.fields[position.0][position.1].doors.iter().enumerate(){
            if *d{
                let res = match i {
                    0 if position.1 > 0 => Some((position.0, position.1 - 1)),
                    1 if position.1 + 1 < self.dimensions.1 => Some((position.0, position.1 + 1)),
                    2 if position.0 > 0 => Some((position.0 - 1, position.1 )),
                    3 if position.0 + 1 < self.dimensions.0 => Some((position.0 + 1, position.1)),
                    _ => None
                };
                if res.is_some(){
                    ret.insert(res.unwrap());
                }
            }
        }
        ret
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

    pub fn search_for_shortest_path(&self, state: State) -> Option<Vec<(usize, usize)>>{
        if self.exits.contains(&state.position){ return Some(vec![state.position]); }

        let walls_graph = self.get_walls_graph();

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

        let mut queue = VecDeque::new();
        queue.push_back(state_history);

        let (solutions_tx, solutions_rx) = mpsc::channel();
        let (states_tx, states_rx) = mpsc::channel();

        while !queue.is_empty(){
            let current_history = queue.pop_front().unwrap();
            let current_state = current_history.last().unwrap();
            let current_position = current_state.position;

            let neighbours = walls_graph.get(&current_position).unwrap();

            let mut threads = vec![];

            for node in neighbours{
                let mut new_history = current_history.clone();
                let node_copy = node.clone();
                let exists = self.exits.clone();
                let solutions = solutions_tx.clone();
                let states = states_tx.clone();
                threads.push(spawn(move ||{
                    let potential_new_state = new_history.last().unwrap().transfer_state(&node_copy);
                    if let Some(new_state) = potential_new_state{
                        if !new_history.contains(&new_state){
                            new_history.push(new_state);
                            if exists.contains(&node_copy){
                                solutions.send(
                                    new_history.iter()
                                    .map(|state| { state.position } )
                                    .collect()
                                ).expect("Couldn't send solution to the channel!");
                            }
                            states.send(new_history)
                                .expect("Couldn't send history to the channel!");
                        }
                    }
                }));
            }

            for t in threads{
                t.join().unwrap();
            }

            for solution in solutions_rx.try_iter(){
                return Some(solution);
            }

            for state in states_rx.try_iter(){
                queue.push_back(state);
            }
        }
        None
    }

    pub fn compare_times_for_path_search(&mut self){
        let mut now;
        let state = self.get_state_mut().clone();

        now = Instant::now();
        self.search_for_shortest_path(state.clone());
        println!("Sequential time taken: {}", now.elapsed().as_secs_f64());

        now = Instant::now();
        self.search_for_shortest_path_parallel(state);
        println!("Parallel time taken: {}", now.elapsed().as_secs_f64());
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

