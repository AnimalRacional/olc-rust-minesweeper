pub mod minesweeper;
pub mod gamedrawer;
pub mod debugging;

fn main(){
    gamedrawer::MinesweeperDrawer::new(9, 9, 10).start_game().unwrap();
}