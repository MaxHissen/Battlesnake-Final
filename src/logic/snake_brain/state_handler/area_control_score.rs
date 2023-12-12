use crate::logic::snake_brain::state_handler::State;
use crate::logic::snake_brain::SIZE;

use std::collections::VecDeque;

pub fn get_area_control_score(state : &State, player : &u8, is_body : bool) -> (u8, [Vec<u8>; 2], bool) {
    //is body means that the area control starts from every body part

    let mut me_tail_spot : (u8, u8) = (11, 11);
    let mut enemy_tail_spot : (u8, u8) = (11, 11);
    let mut can_reach_me_tail = false;
    get_tail_positions(state, &mut me_tail_spot, &mut enemy_tail_spot, player);


    let mut area_i_control = 0;

    // (auto) sorted list of food distances for each snake
    let mut food_distances : [Vec<u8>; 2] = [Vec::new(), Vec::new()];


    let mut indices: Vec<usize> = (0..state.snake_lengths.len()).collect();
    indices.sort_by(|&a, &b| state.snake_lengths[b].cmp(&state.snake_lengths[a]));
    

    let mut frontier: VecDeque<(u8, u8, u8, u8)> = VecDeque::new();
    for i in indices.iter(){
        if state.snake_lengths[*i] >= 3{
            frontier.push_back((state.snake_heads[*i].0, state.snake_heads[*i].1, *i as u8, 0));
        }
    }

    if is_body{
        for y in 0..SIZE{
            for x in 0..SIZE{
                let cell = state.board_struct[y][x];
                if cell != 0b11111111 && cell != 0{
                    let index = (cell&0b10000000) >> 7;
                    frontier.push_back((x as u8, y as u8, index, 0));
                }
            }
        }
    }

    let mut iteration = 0;

    let mut gone_to : [[u8; SIZE]; SIZE] = [[4; SIZE]; SIZE];

    while let Some(cur) = frontier.pop_front() {
        iteration += 1;

        let cur_x = cur.0;
        let cur_y = cur.1;
        let cur_index = cur.2;
        let cur_distance = cur.3;

        if state.board_struct[cur_y as usize][cur_x as usize]&0b01111111 > cur_distance && state.board_struct[cur_y as usize][cur_x as usize] != 0b11111111{
            continue;
        }
        if gone_to[cur_y as usize][cur_x as usize] != 4{
            continue;
        }
        //println!("index: {:?}", cur_index);

        if (cur_x, cur_y) == me_tail_spot{
            if cur_index == *player{
                can_reach_me_tail = true;
            }
        }

        if state.board_struct[cur_y as usize][cur_x as usize] == 0b11111111{
            food_distances[cur_index as usize].push(cur_distance);
        }

        gone_to[cur_y as usize][cur_x as usize] = cur_index;
        if cur_index == *player{
            area_i_control += 1;
        }

        if cur_y < 10{
            frontier.push_back((cur_x, cur_y + 1, cur_index, cur_distance + 1));
        }
        if cur_y > 0{
            frontier.push_back((cur_x, cur_y - 1, cur_index, cur_distance + 1));
        }
        if cur_x > 0{
            frontier.push_back((cur_x - 1, cur_y, cur_index, cur_distance + 1));
        }
        if cur_x < 10{
            frontier.push_back((cur_x + 1, cur_y, cur_index, cur_distance + 1));
        }
    }

    return (area_i_control, food_distances, can_reach_me_tail);
}


pub fn get_tail_positions(state : &State, me_tail_spot : &mut (u8, u8), enemy_tail_spot : &mut (u8, u8), player : &u8){
    let mut smallest_me_index = (SIZE*SIZE) as u8;
    let mut smallest_enemy_index = (SIZE*SIZE) as u8;

    for y in 0..SIZE{
        for x in 0..SIZE{
            let cell = state.board_struct[y][x];
            if cell != 0b11111111{
                let index = (cell & 0b10000000) >> 7;
                let size = cell&0b01111111;
                if size >= 1{
                    if index == *player{
                        if size < smallest_me_index{
                            smallest_me_index = size;
                            me_tail_spot.0 = x as u8;
                            me_tail_spot.1 = y as u8;
                        }
                    }
                    if index == (1 - *player){
                        if size < smallest_enemy_index{
                            smallest_enemy_index = size;
                            enemy_tail_spot.0 = x as u8;
                            enemy_tail_spot.1 = y as u8;
                        }
                    }
                }
            }
        }
    }
}
