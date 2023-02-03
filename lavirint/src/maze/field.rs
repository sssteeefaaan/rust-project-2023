use std::fmt::Display;

#[derive(Clone)]
pub struct Field{
    pub position: (usize, usize),
    pub exit: bool,
    pub key: bool,
    pub walls: Vec<bool>,
    pub doors: Vec<bool>
}

impl Default for Field{
    fn default() -> Self {
        Self { position: (0, 0), exit: false, key: false, doors: vec![false; 4], walls: vec![true; 4] }
    }
}

impl Display for Field{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("Position({}, {}) - ", self.position.0, self.position.1);
        output += format!("Exit: {} - ", self.exit).as_str();
        output += format!("Key: {} - ", self.key).as_str();
        output += format!("Walls: ({}, {}, {}, {}) - ", self.walls[0], self.walls[1], self.walls[2], self.walls[3]).as_str();
        output += format!("Doors: ({}, {}, {}, {})", self.doors[0], self.doors[1], self.doors[2], self.doors[3]).as_str();
        f.write_str(&output)
    }
}