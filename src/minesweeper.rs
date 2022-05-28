use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Squares{
    ClosedSafe,
    OpenSafe,
    FlaggedSafe,
    ClosedBomb,
    FlaggedBomb
}
#[derive(Eq, PartialEq)]
pub enum State{
    Ongoing,
    Won,
    Lost
}

pub struct MinesweeperGame{
    pub board_size: Point,
    pub board: Vec<Squares>,
    pub ori_bomb_amount: i32,
    pub game_state: State
}

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point{
    fn new(x: i32, y: i32) -> Point{
        Point{ x: x, y: y }
    }
}

// Setting up
impl MinesweeperGame {
    pub fn new(xsize: i32, ysize: i32, bomb_amount: i32) -> MinesweeperGame{
        let mut game = MinesweeperGame {
            board_size: Point::new(xsize, ysize),
            board: vec![Squares::ClosedSafe],
            ori_bomb_amount: if bomb_amount < xsize * ysize { bomb_amount } else { xsize * ysize - 1 },
            game_state: State::Ongoing
        };
        game.reset_board();
        game
    }

    fn reset_board(&mut self){
        self.board.clear();
        let board_size = self.board_size.x * self.board_size.y;
        for _ in 0..board_size {
            self.board.push(Squares::ClosedSafe);
        }

        for _ in 0..self.ori_bomb_amount {
            let mut id = thread_rng().gen_range(0..self.board.len());
            while let Squares::ClosedBomb = self.board[id] {
                id = thread_rng().gen_range(0..self.board.len());
            }
            self.board[id] = Squares::ClosedBomb;
        }
    }
}

// Util
impl MinesweeperGame {
    pub fn pos_moves() -> Vec<Point> {
        vec![
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1)
        ]
    }

    pub fn calculate_index_by_coords(&self, x: i32, y: i32) -> i32{
        y * self.board_size.x + x
    }

    pub fn calculate_coords_by_index(&self, i: i32) -> Point{
        Point::new(i % self.board_size.x, i / self.board_size.x)
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool{
        x < self.board_size.x && x >= 0 && y < self.board_size.y && y >= 0
    }

    pub fn calculate_neighbours(&self, x: i32, y: i32) -> i32 {        
        let mut res = 0;
        let pos_moves = MinesweeperGame::pos_moves();
        for i in pos_moves {
            if self.is_inside(x + i.x, y + i.y) {
                let id = self.calculate_index_by_coords(x + i.x, y + i.y);
                if let Some(a) = self.board.get(id as usize) {
                    if let Squares::ClosedBomb | Squares::FlaggedBomb = a {
                        res += 1;
                        //println!("Found for {} {} at {} {}", x,y,x + i.x, y + i.y);
                    }
                }
            }
        }
        res
    }
}

// Game
impl MinesweeperGame {
    pub fn restart_game(&mut self){
        self.game_state = State::Ongoing;
        self.reset_board();
    }

    pub fn lose(&mut self){
        self.game_state = State::Lost;
    }

    pub fn win(&mut self){
        self.game_state = State::Won;
    }

    pub fn reveal(&mut self, xpos: i32, ypos: i32){
        let id = self.calculate_index_by_coords(xpos, ypos) as usize;
        self.board[id] = match &self.board[id] {
            Squares::ClosedBomb => { self.lose(); Squares::ClosedBomb }
            Squares::ClosedSafe => { 
                self.reveal_around(xpos, ypos);
                Squares::OpenSafe 
            }
            other => { *other }
        };
        if self.has_won() {
            self.win();
        }
    }

    fn reveal_around(&mut self, x: i32, y: i32){
        let pos_moves = MinesweeperGame::pos_moves();
        if self.is_inside(x, y){
            if self.calculate_neighbours(x, y) <= 0 {
                for i in pos_moves {
                    if self.is_inside(x + i.x, y + i.y) {
                        let pos = self.calculate_index_by_coords(x + i.x, y + i.y);
                        if let Some(_) = self.board.get(pos as usize){
                            if let Squares::ClosedSafe = self.board[pos as usize] {
                                self.board[pos as usize] = Squares::OpenSafe;
                                //println!("Started revealing at ({}, {}) with {} neighbours origin ({}, {})", x + i.x, y + i.y, self.calculate_neighbours(x, y), x, y);
                                self.reveal_around(x + i.x, y + i.y);
                            }
                        }
                    }
                }
            }
            let pos = self.calculate_index_by_coords(x, y);
            self.board[pos as usize] = Squares::OpenSafe;
        }
        if self.has_won() {
            self.win();
        }
    }

    pub fn flag(&mut self, xpos: i32, ypos: i32){
        let id = self.calculate_index_by_coords(xpos, ypos) as usize;
        self.board[id] = match &self.board[id]{
            Squares::ClosedBomb => { Squares::FlaggedBomb },
            Squares::ClosedSafe => { Squares::FlaggedSafe },
            Squares::FlaggedBomb => { Squares::ClosedBomb },
            Squares::FlaggedSafe => { Squares::ClosedSafe },
            other => { *other }
        };
        if self.has_won() {
            self.win();
        }
    }

    pub fn has_won(&self) -> bool {
        let mut found_bomb = false;
        let mut found_nonrevealed = false;
        for i in &self.board{
            if *i == Squares::ClosedBomb {
                found_bomb = true;
            }
            else if *i == Squares::FlaggedSafe || *i == Squares::ClosedSafe {
                found_nonrevealed = true;
            }
            if found_bomb && found_nonrevealed {
                return false;
            }
        }
        true
    }
}