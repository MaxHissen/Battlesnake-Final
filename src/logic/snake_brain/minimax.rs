/*
search strat:

Opponent Pruning Paranoid Search:


my move is played

the player with the best counter-move to me is searched for any move he could make (l1 = max moves)

the other snakes only have their highest ranked moves evaluated (l2 = max moves)




alpha-beta pruning can still be applied
*/
use crate::logic::snake_brain::state_handler::area_control_score;
use crate::logic::snake_brain::state_handler;
use crate::logic::snake_brain::state_handler::State;
use crate::logic::snake_brain::SIZE;
use std::time::{Instant};

pub fn minimax(state : State) -> u8{
    //returns best move
    let start_time = Instant::now();
    
    let mut depth = 0;
    let mut best_move = (0x7FFFFFFF, 0);

    loop{
        depth += 1;
        best_move = opps(state.clone(), 0, depth, 0, 0, 0xFFFFFFFF, true, start_time);
        //break; //break if depth = 1?
        if (Instant::now() - start_time).as_millis() > 30 || depth > 50{
            break;
        }
    }
    println!("depth: {:?}", depth);

    //state_handler::print_state(&state);
    return best_move.1;
}

fn opps(mut state: State, mut depth: u8, max_depth: u8, mut player: u8, mut alpha: u32, mut beta: u32, first_time: bool, start_time: std::time::Instant) -> (u32, u8) {
    //state_handler::print_state(&state);
    
    // If back to beginning
    if player > 1 {
        player = 0;
        depth += 1;
        if !first_time {
            state_handler::end_turn(&mut state);
            //println!("{:?}", state.original_board_struct);

            if state.are_snakes_alive[0] && !state.are_snakes_alive[1]{
                return (0xFFFFFFFF, 4);
            }
            if !state.are_snakes_alive[0] && state.are_snakes_alive[1]{
                return (0 + (state.snake_heads[0] == state.snake_heads[1]) as u32, 4);
            }
            if !state.are_snakes_alive[0] && !state.are_snakes_alive[1]{
                return (0x7FFFFFFF, 4);
            }
        }
    }

    // If max depth
    if depth >= max_depth {
        let mut value = state_handler::state_value(&state, &0);
        //println!("value {:?}", value);
        return (value, 4);
    }


    // Traverse tree with alpha-beta pruning
    let mut best_move_score = 0;
    if player != 0{
        best_move_score = 0xFFFFFFFF;
    }
    let mut best_move_direction = 4;

    let mut move_order = vec![0, 1, 2, 3];
    if depth > 1{
        move_order = get_move_order(&state, &player);
    }
    //println!("{:?}", move_order);

    for move_direction in move_order {

        let mut state_copy = state.clone();
        let move_struct = move_direction + (player << 2);

        state_handler::make_move(&mut state_copy, &move_struct);
        //don't copy state if unnecessary
        let mut score = opps(state_copy.clone(), depth, max_depth, player + 1, alpha, beta, false, start_time).0;

        //println!("{:?} {:?} {:?}", move_direction, player, score);
        if score < 0x10000000{
            score += depth as u32
        }
        if player == 0 {
            if score > best_move_score {
                best_move_score = score;
                best_move_direction = move_direction;
            }
            alpha = alpha.max(best_move_score);
        } else {
            if score < best_move_score {
                best_move_score = score;
                best_move_direction = move_direction;
            }
            beta = beta.min(best_move_score);
        }

        // Alpha-beta pruning
        if beta <= alpha {
            break;
        }
    }

    (best_move_score, best_move_direction)
}

fn get_move_order(state : &State, player : &u8) -> Vec<u8>{
    //returns the 2 most promising moves for the player

    let mut array : Vec<(u8, u32)> = vec![(0,0), (1,0), (2,0), (3,0)];
    
    if *player == 0{
        for my_move in 0..4{
            if is_move_a_bad_one(state, &0, my_move){
                array[my_move as usize].1 = 0;
                continue;
            }

            let mut state_copy = state.clone();
            state_handler::make_move(&mut state_copy, &(my_move as u8));

            let mut best_enemy_score = 0xFFFFFFFF;
            for enemy_move in 0..4{
                if is_move_a_bad_one(&state_copy, &1, enemy_move){
                    continue;
                }
                let mut state_copy_copy = state_copy.clone();

                state_handler::make_move(&mut state_copy_copy, &(4 + enemy_move as u8));
                state_handler::end_turn(&mut state_copy_copy);

                let positions_score = state_handler::state_value(&state_copy_copy, &0);
                if positions_score < best_enemy_score{
                    best_enemy_score = positions_score;
                }
            }
            
            array[my_move as usize].1 = best_enemy_score;
        }

        array.sort_by_key(|&value| std::cmp::Reverse(value.1.clone()));
        array.truncate(2);

        return array.iter().map(|&(first, _)| first).collect();
    }




    else if *player == 1{
        //state_handler::print_state(state);
        for my_move in 0..4{
            if is_move_a_bad_one(state, &1, my_move){
                array[my_move as usize].1 = 0xFFFFFFFF;
                continue;
            }

            let mut state_copy = state.clone();
            state_handler::make_move(&mut state_copy, &(4 + my_move as u8));
            state_handler::end_turn(&mut state_copy);

            array[my_move as usize].1 = state_handler::state_value(&state_copy, &0);
        }
        
        array.sort_by_key(|&value| value.1.clone());


        array.truncate(2);


        let array: Vec<_> = array.iter().map(|&(first, _)| first).collect();
        
        return array;
    }

    println!("for player {:?} {:?}", player, array);

    return vec![0];


    fn is_move_a_bad_one(state : &State, player : &u8, move_int : u8) -> bool{
        //returns whether move is an instant loss for player

        let head_x = state.snake_heads[*player as usize].0;
        let head_y = state.snake_heads[*player as usize].1;

        let mut new_x = head_x;
        let mut new_y = head_y;

        if move_int == 0{
            if head_y >= SIZE as u8 - 1{
                return true;
            }
            new_y += 1;
        }
        if move_int == 1{
            if head_y <= 0{
                return true;
            }
            new_y -= 1;
        }
        if move_int == 2{
            if head_x <= 0{
                return true;
            }
            new_x -= 1;
        }
        if move_int == 3{
            if head_x >= SIZE as u8 - 1{
                return true;
            }
            new_x += 1;
        }

        let cell = state.board_struct[new_y as usize][new_x as usize];
        if cell != 0b11111111{
            if cell&0b01111111 > 1{
                return true;
            }
            if state.snake_healths[*player as usize] <= 1{
                return true;
            }
        }

        return false;
    }
}
