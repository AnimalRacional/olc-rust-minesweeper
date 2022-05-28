use crate::minesweeper::{MinesweeperGame, Squares, State};
use crate::debugging;
use olc_pixel_game_engine as olc;

pub struct MinesweeperDrawer {
    game: MinesweeperGame
}


// Graphics
impl MinesweeperDrawer {
    pub fn new(xsize: i32, ysize: i32, bomb_amount: i32) -> MinesweeperDrawer{
        MinesweeperDrawer{
            game: MinesweeperGame::new(xsize, ysize, bomb_amount)
        }
    }

    fn calculate_screen_width(&self) -> i32 {
        self.game.board_size.x
    }

    fn calculate_screen_height(&self) -> i32 {
        self.game.board_size.y
    }

    const SQUARE_SIZE: i32 = 10;
    pub fn start_game(&mut self) -> Result<(), olc::Error> {
        let sw = self.calculate_screen_width();
        let sh = self.calculate_screen_height();
        
        let win_title = &format!("Minesweeper: {}x{}, {} bombs", self.game.board_size.x, self.game.board_size.y, self.game.ori_bomb_amount)[..];
        debugging::print_debug(win_title);
        olc::start(
            win_title,
            self, // The app
            sw * MinesweeperDrawer::SQUARE_SIZE,sh * MinesweeperDrawer::SQUARE_SIZE, // The screen width and height
            10,10) // The pixel size
    }

    fn draw_board(&self){
        let len = self.game.board.len();
        for i in 0..len {
            let mut coords = self.game.calculate_coords_by_index(i as i32);
            coords.x *= MinesweeperDrawer::SQUARE_SIZE;
            coords.y *= MinesweeperDrawer::SQUARE_SIZE;
            olc::fill_rect(coords.x, coords.y, coords.x + MinesweeperDrawer::SQUARE_SIZE, coords.y + MinesweeperDrawer::SQUARE_SIZE, match self.game.board[i]{
                Squares::ClosedSafe => { olc::GREY },
                Squares::ClosedBomb => { if !debugging::debug_on() { olc::GREY } else { olc::GREEN } },
                Squares::OpenSafe => { olc::WHITE },
                _ => { olc::RED }
            });
            olc::draw_rect(coords.x, coords.y, coords.x + MinesweeperDrawer::SQUARE_SIZE, coords.y + MinesweeperDrawer::SQUARE_SIZE, olc::BLACK);
            if let Squares::OpenSafe = self.game.board[i] {
                self.draw_number(coords.x / MinesweeperDrawer::SQUARE_SIZE, coords.y / MinesweeperDrawer::SQUARE_SIZE);
            }
        }
    }

    fn draw_number(&self, x: i32, y: i32){
        let n = self.game.calculate_neighbours(x,y);
        if n > 0 {
            //println!("Drawing number at {} {}", x,y);
            olc::draw_string(x*MinesweeperDrawer::SQUARE_SIZE+1, y*MinesweeperDrawer::SQUARE_SIZE+1, &(n.to_string())[..], olc::BLACK).unwrap();
        }
    }

    fn screen_to_game(&self, x: i32, y: i32) -> olc::Vi2d{
        olc::Vi2d::new(x,y)
    }
}

// Implementing the application
impl olc::Application for MinesweeperDrawer {
    fn on_user_create(&mut self) -> Result<(), olc::Error>{
        self.draw_board();
        Ok(())
    }

    fn on_user_update(&mut self, _: f32) -> Result<(), olc::Error>{
        if self.game.game_state == State::Lost {
            if olc::get_key(olc::Key::SPACE).pressed {
                self.game.restart_game();
                self.draw_board();
            }
            else{
                olc::clear(olc::RED);
                olc::draw_string(0, 0, "You lose!", olc::BLACK).unwrap();
            }
        }
        else if self.game.game_state == State::Won {
            if olc::get_key(olc::Key::SPACE).pressed {
                self.game.restart_game();
                self.draw_board();
            }
            else{
                olc::clear(olc::YELLOW);
                olc::draw_string(0, 0, "You win!", olc::BLACK).unwrap();
            }
        }
        else{
            let gamepos = self.screen_to_game(olc::get_mouse_x(), olc::get_mouse_y());
            if olc::get_mouse(0).pressed {
                self.game.reveal(gamepos.x / MinesweeperDrawer::SQUARE_SIZE, gamepos.y / MinesweeperDrawer::SQUARE_SIZE);
                self.draw_board();
            }
            else if olc::get_mouse(1).pressed {
                self.game.flag(gamepos.x / MinesweeperDrawer::SQUARE_SIZE, gamepos.y / MinesweeperDrawer::SQUARE_SIZE);
                self.draw_board();
            }
        }



        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error>{
        Ok(())
    }
}
