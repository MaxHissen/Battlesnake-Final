
use crate::{Battlesnake, Board};
use crate::logic::snake_brain::state_handler::State;
mod minimax;
mod state_handler;

pub const SIZE: usize = 11;

pub fn calculate_direction(turn: &u32, board: &Board, you: &Battlesnake) -> String {

    //create current game board_struct
    let mut board_struct: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut snake_lengths: [u8; 2] = [you.length as u8, 0];
    let mut snake_healths: [u8; 2] = [you.health as u8, 0];
    let mut snake_heads : [(u8, u8); 2] = [(SIZE as u8, SIZE as u8); 2];
    let mut are_snakes_alive : [bool; 2] = [true, false];
    let turn = *turn as u16;
    let mut original_board_struct: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    state_handler::initialize_board_struct_for_position(&mut board_struct, &mut snake_lengths, &mut snake_healths, &mut snake_heads, &mut are_snakes_alive, board, you, &mut original_board_struct);
    let mut state = State{board_struct, snake_lengths, snake_healths, snake_heads, are_snakes_alive, turn, original_board_struct};
    
    //state_handler::print_state(&state);

    let to_move = minimax::minimax(state);


    println!("moving direction: {:?}", to_move);
    
    //return move direction
    let move_normally = true;
    if move_normally{
        if to_move == 0{
            return "up".to_string();
        }
        if to_move == 1{
            return "down".to_string();
        }
        if to_move == 2{
            return "left".to_string();
        }
        return "right".to_string();
    }
    if turn%2 == 0{
        return "right".to_string();
    }
    return "left".to_string();
    
    
}
