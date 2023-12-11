use crate::logic::snake_brain::state_handler::State;
use crate::logic::snake_brain::SIZE;

use std::collections::VecDeque;

pub fn get_area_control_score(state : &State, player : &u8) -> (u8, [Vec<u8>; 2]) {


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

    let mut iteration = 0;

    let mut gone_to : [[u8; SIZE]; SIZE] = [[4; SIZE]; SIZE];
    //print_board_struct(gone_to);
    
    //println!("{:?}", frontier);
    while let Some(cur) = frontier.pop_front() {
        //println!();
        iteration += 1;

        let cur_x = cur.0;
        let cur_y = cur.1;
        let cur_index = cur.2;
        let cur_distance = cur.3;
        //println!("index: {:?} {:?} {:?}", cur_index, cur_x, cur_y);

        if state.board_struct[cur_y as usize][cur_x as usize]&0b01111111 > cur_distance && state.board_struct[cur_y as usize][cur_x as usize] != 0b11111111{
            continue;
        }
        if gone_to[cur_y as usize][cur_x as usize] != 4{
            continue;
        }
        //println!("index: {:?}", cur_index);

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

    return (area_i_control, food_distances);
}

pub fn body_in_center_score(state : &State) -> u8{
    let mut in_center_score : u16 = 0;

    for y in 0..SIZE{
        for x in 0..SIZE{
            let cell = state.board_struct[y][x];

            let near_middle_x = -(x as i16 - SIZE as i16/2).abs() + SIZE as i16/2;
            let near_middle_y = -(y as i16 - SIZE as i16/2).abs() + SIZE as i16/2;
            if (cell&0b11000000) >> 6 == 0 && cell > 0{
                in_center_score += ((cell/10) as i16 * (near_middle_x + near_middle_y)) as u16;
            }
        }
    }

    return in_center_score as u8;
}