use crate::logic::snake_brain::state_handler::State;
use crate::logic::snake_brain::SIZE;
use std::io;
use std::io::Write;

use std::collections::VecDeque;

pub fn floodfill(state : &State, worst_case : bool, buffer : u8) -> u8{
    //returns longest path that can be found


    let board_struct : [[u8; SIZE]; SIZE] = state.board_struct;
    let lengths : [u8; 2] = state.snake_lengths;


    let start_struct = get_start_struct(board_struct, lengths, buffer);
    //print_board_struct(start_struct);

    let heads_of_snakes = state.snake_heads;

    let mut enemy_head_struct : [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut smaller_enemy_head_struct : [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];

    for i in 1..2{
        if lengths[i] >= 3{
            if lengths[i] >= lengths[0]{
                enemy_head_struct[heads_of_snakes[i].1 as usize][heads_of_snakes[i].0 as usize] = true;
            }
            else{
                smaller_enemy_head_struct[heads_of_snakes[i].1 as usize][heads_of_snakes[i].0 as usize] = true;
            }
        }
    }

    //print_head_struct(enemy_head_struct);

    let mut lifo_queue: VecDeque<([[u8; SIZE]; SIZE], [u8; 2], u8, u8, u8, [[bool; SIZE]; SIZE], [[bool; SIZE]; SIZE])> = VecDeque::new();
    lifo_queue.push_back((start_struct, lengths, heads_of_snakes[0].0, heads_of_snakes[0].1, 0, enemy_head_struct, smaller_enemy_head_struct));

    let mut max_depth = 0;
    let mut iteration = 0;
    while let Some(cur) = lifo_queue.pop_back() {
        iteration += 1;

        if iteration >= 100{
            break;
        }

        let cur_board_struct = cur.0;
        let mut snake_lengths = cur.1;
        let cur_x = cur.2 as i8;
        let cur_y = cur.3 as i8;
        let depth = cur.4;
        let enemy_head_struct = cur.5;
        let smaller_enemy_head_struct = cur.6;


        if depth > snake_lengths[0] + 10{
            return depth;
        }


        let new_enemy_head_struct = update_head_struct(enemy_head_struct, cur_board_struct);
        //print_head_struct(new_enemy_head_struct);

        let directions = [(0,1), (0,-1), (-1,0), (1,0)];

        for (dx, dy) in directions{
            if cur_y + dy >= SIZE as i8 || cur_y + dy < 0 || cur_x + dx >= SIZE as i8 || cur_x + dx < 0{
                continue;
            }


            if worst_case && (enemy_head_struct[(cur_y+dy) as usize][(cur_x+dx) as usize] || smaller_enemy_head_struct[(cur_y+dy) as usize][(cur_x+dx) as usize]){
                continue;
            }

            if cur_board_struct[(cur_y + dy) as usize][(cur_x + dx) as usize] == 0b11111111 || cur_board_struct[(cur_y + dy) as usize][(cur_x + dx) as usize]&0b00111111 <= 1{

                let new_smaller_enemy_head_struct = update_head_struct(smaller_enemy_head_struct, cur_board_struct);
                let new_board_struct = move_head(cur_board_struct, &mut snake_lengths, cur_x as u8, cur_y as u8, (cur_x + dx) as u8, (cur_y + dy) as u8, buffer);
                
                lifo_queue.push_back((new_board_struct, lengths, (cur_x + dx) as u8, (cur_y + dy) as u8, depth + 1, new_enemy_head_struct, new_smaller_enemy_head_struct));
            }
        }
    }
    return max_depth;
}
fn get_start_struct(board_struct : [[u8; SIZE]; SIZE], snake_lengths : [u8; 2], buffer : u8) -> [[u8; SIZE]; SIZE]{

    let mut start_struct : [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for y in 0..SIZE as usize{
        for x in 0..SIZE as usize{
            let cell = board_struct[y][x];

            if cell == 0b11111111{
                start_struct[y][x] = cell;
                continue;
            }


            start_struct[y][x] = cell + buffer;

        }
    }

    return start_struct;
}
fn move_head(board_struct : [[u8; SIZE]; SIZE], snake_lengths : &mut [u8; 2], old_x : u8, old_y : u8, new_x : u8, new_y : u8, buffer : u8) -> [[u8; 11]; 11]{
    let ate_food = board_struct[new_y as usize][new_x as usize] == 0b11111111;
    snake_lengths[0] += ate_food as u8;

    let mut new_board_struct : [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for y in 0..SIZE as usize{
        for x in 0..SIZE as usize{
            let cell = board_struct[y][x];
            new_board_struct[y][x] = cell;

            if cell == 0b11111111{
                continue;
            }

            else if cell > 0{
                //belongs to a snake

                let index = ((cell&0b10000000) >> 7) as usize;

                if index == 0{
                    //not head so subtract
                    new_board_struct[y][x] -= (!ate_food) as u8;
                    continue;
                }
                else{
                    new_board_struct[y][x] -= 1;
                    if new_board_struct[y][x]&0b01111111 == 0{
                        new_board_struct[y][x] = 0;
                    }
                    continue;
                }
            }
        }
    }

    new_board_struct[old_y as usize][old_x as usize] = snake_lengths[0] - 1 + buffer;

    return new_board_struct;
}
fn update_head_struct(head_struct : [[bool; SIZE]; SIZE], board_struct : [[u8; SIZE]; SIZE]) -> [[bool; SIZE]; SIZE]{
    let mut new_head_struct = head_struct.clone();

    for y in 0..SIZE as usize{
        for x in 0..SIZE as usize{
            if y < SIZE-1{
                if board_struct[y][x]&0b01111111 <= 1{
                    new_head_struct[y][x] |= head_struct[y + 1][x];
                }
            }
            if y > 0{
                if board_struct[y][x]&0b01111111 <= 1{
                    new_head_struct[y][x] |= head_struct[y - 1][x];
                }
            }
            if x > 0{
                if board_struct[y][x]&0b01111111 <= 1{
                    new_head_struct[y][x] |= head_struct[y][x - 1];
                }
            }
            if x < SIZE-1{
                if board_struct[y][x]&0b01111111 <= 1{
                    new_head_struct[y][x] |= head_struct[y][x + 1];
                }
            }
        }
    }

    return new_head_struct;
}



fn print_board_struct(board_struct: [[u8; 11]; 11]){
    for y in 0..11{
        for x in 0..11{
            let mut cell_value = board_struct[11 - y - 1][x];

            for _ in 0..8{
                let add = (cell_value & 0b10000000) >> 7;
                print!("{:?}", add);
                cell_value = cell_value << 1
            }
            print!("  ");
            io::stdout().flush().unwrap();
        }
        println!("");
    }
    println!("");
}
