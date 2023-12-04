#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub cube_sets: Vec<CubeSet>,
}
impl Game {
    
    pub fn max_red(&self) -> u32 {
        self.cube_sets.iter().map(|s| s.red).max().unwrap()
    }
    pub fn max_green(&self) -> u32 {
        self.cube_sets.iter().map(|s| s.green).max().unwrap()
    }
    pub fn max_blue(&self) -> u32 {
        self.cube_sets.iter().map(|s| s.blue).max().unwrap()
    }
}
#[derive(Debug)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
