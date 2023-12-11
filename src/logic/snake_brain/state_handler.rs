
use std::io;
use std::io::Write;
use crate::logic::snake_brain::SIZE;
pub mod area_control_score;
pub mod floodfill;
use crate::{Battlesnake, Board};

#[derive(Clone)]
pub struct State {
    pub board_struct: [[u8; SIZE]; SIZE],
    pub snake_lengths : [u8; 2],
    pub snake_healths : [u8; 2],
    pub snake_heads : [(u8, u8); 2],
    pub are_snakes_alive : [bool; 2],
    pub turn : u16,
}



pub fn initialize_board_struct_for_position(board_struct : &mut [[u8; SIZE]; SIZE], snake_lengths : &mut [u8; 2], snake_healths : &mut [u8; 2], snake_heads : &mut [(u8, u8); 2], are_snakes_alive : &mut [bool; 2], board: &Board, you: &Battlesnake){
    
    
    //get important values from game
    let me_id = &you.id;

    let amount_of_alive_snakes : u8 = board.snakes.len() as u8;

    //add in enemy snakes
    //add in enemy bodies
    let mut snake_index = 1;
    for s in 0..amount_of_alive_snakes as usize{
        if &board.snakes[s].id != me_id{
            //set alive value
            are_snakes_alive[snake_index] = true;
            
            //set health values
            snake_healths[snake_index] = board.snakes[s].health as u8;
            
            //set length values
            snake_lengths[snake_index] = board.snakes[s].length as u8;


            let body_length = board.snakes[s].length;
            for i in (1..body_length).rev(){
                let x_pos = board.snakes[s].body[i as usize].x as usize;
                let y_pos = board.snakes[s].body[i as usize].y as usize;

                let mut to_add = (body_length - i) as u8;
                to_add += (snake_index<<7) as u8;

                board_struct[y_pos][x_pos] = to_add;
            }


            //add in enemy snake head
            let head = &board.snakes[s].head;
            let head_x = head.x as u8;
            let head_y = head.y as u8;

            snake_heads[snake_index] = (head_x, head_y);

            snake_index += 1;
        }
    }



    //add in me
    //add in me body
    let body_length = you.body.len();

    for i in (1..body_length).rev(){
        let x_pos = you.body[i].x as usize;
        let y_pos = you.body[i].y as usize;

        let to_add = (body_length - i) as u8;

        board_struct[y_pos][x_pos] = to_add;
    }


    //add in me head
    let head = &you.head;
    let head_x = head.x as u8;
    let head_y = head.y as u8;
    snake_heads[0] = (head_x, head_y);


    //add in food
    for s in 0..board.food.len(){
        let x_pos = board.food[s].x as usize;
        let y_pos = board.food[s].y as usize;
        board_struct[y_pos][x_pos] = 0b11111111;
    }
}
pub fn make_move(state: &mut State, move_struct : &u8){
    //change State by moving snake as defined by move_struct
    //move_struct: 00000111 move snake 1 (1)  right (3)

    //add in neck part
    let snake_to_move = ((move_struct&0b00001100) >> 2) as usize;
    if state.are_snakes_alive[snake_to_move] != true{
        return;
    }
    state.board_struct[state.snake_heads[snake_to_move].1 as usize][state.snake_heads[snake_to_move].0 as usize] = state.snake_lengths[snake_to_move] + ((snake_to_move as u8) << 7);

    //move head
    let move_direction = move_struct&0b00000011;
    if move_head(&mut state.snake_heads, &move_direction, &snake_to_move) {

        //find if ate food
        let ate_food = state.board_struct[state.snake_heads[snake_to_move].1 as usize][state.snake_heads[snake_to_move].0 as usize] == 0b11111111;

        //remove body by 1 if not ate food
        if !ate_food{
            for y in 0..SIZE{
                for x in 0..SIZE{
                    let value = state.board_struct[y][x];
                    if value != 0b11111111{
                        if (value&0b10000000) >> 7 == snake_to_move as u8{
                            if value&0b01111111 > 0{
                                state.board_struct[y][x] -= 1;
                                if state.board_struct[y][x]&0b01111111 == 0{
                                    state.board_struct[y][x] = 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        else{
            //remove trailing 1 for some reason
            for y in 0..SIZE{
                for x in 0..SIZE{
                    let value = state.board_struct[y][x];
                    if (value&0b10000000) >> 7 == snake_to_move as u8{
                        if value&0b01111111 == 1{
                            state.board_struct[y][x] = 0;
                        }
                    }
                }
            }
        }

        //remove health by 1 if not ate food
        if ate_food{
            state.snake_healths[snake_to_move] = 100;
            state.snake_lengths[snake_to_move] += 1;
        }
        else{
            state.snake_healths[snake_to_move] -= 1;
        }
    }

    fn move_head(heads : &mut [(u8, u8); 2], move_direction : &u8, snake_to_move : &usize) -> bool{
        if *move_direction == 0{
            if heads[*snake_to_move].1 >= SIZE as u8 - 1{
                heads[*snake_to_move] = (SIZE as u8, SIZE as u8);
                return false;
            }
            heads[*snake_to_move] = (heads[*snake_to_move].0, heads[*snake_to_move].1 + 1);
        }
        if *move_direction == 1{
            if heads[*snake_to_move].1 <= 0{
                heads[*snake_to_move] = (SIZE as u8, SIZE as u8);
                return false;
            }
            heads[*snake_to_move] = (heads[*snake_to_move].0, heads[*snake_to_move].1 - 1);
        }
        if *move_direction == 2{
            if heads[*snake_to_move].0 <= 0{
                heads[*snake_to_move] = (SIZE as u8, SIZE as u8);
                return false;
            }
            heads[*snake_to_move] = (heads[*snake_to_move].0 - 1, heads[*snake_to_move].1);
        }
        if *move_direction == 3{
            if heads[*snake_to_move].0 >= SIZE as u8 - 1{
                heads[*snake_to_move] = (SIZE as u8, SIZE as u8);
                return false;
            }
            heads[*snake_to_move] = (heads[*snake_to_move].0 + 1, heads[*snake_to_move].1);
        }
        return true;
    }
}
pub fn end_turn(state: &mut State){

    //kill the snakes that need to be killed according to board struct
    //to be used at the end of the turn (ie. before it's snake 0's turn again)
    state.turn += 1;


    let mut to_kill : [bool; 2] = [false; 2];

    for snake_index in 0..2 as usize{
        //health <= 0
        if state.snake_healths[snake_index] <= 0{
            to_kill[snake_index] = true;
            continue;
        }

        //moved out of bounds
        let head_x = state.snake_heads[snake_index].0;
        let head_y = state.snake_heads[snake_index].1;
        if head_x >= SIZE as u8 || head_y >= SIZE as u8{
            to_kill[snake_index] = true;
            continue;
        }


        //collided with snake part
        let cell = state.board_struct[head_y as usize][head_x as usize];
        if cell != 0b11111111{
            if cell&0b01111111 > 0 {
                //if something in cell that's not food, kill snake
                to_kill[snake_index] = true;
                continue;
            }
        }

        //lost head-to-head
        for to_collide in 0..2 as usize{
            if snake_index != to_collide{
                if state.snake_heads[snake_index] == state.snake_heads[to_collide]{
                    if state.snake_lengths[snake_index] <= state.snake_lengths[to_collide]{
                        to_kill[snake_index] = true;
                        continue;
                    }
                }
            }
        }

        //remove food cells with heads on them
        if state.board_struct[head_y as usize][head_x as usize] == 0b11111111{
            state.board_struct[head_y as usize][head_x as usize] = 0;
        }

    }



    for s in 0..2 as usize{
        if to_kill[s]{
            kill_snake(state, &s);
        }
    }

    fn kill_snake(state: &mut State, snake_index : &usize){
        state.snake_heads[*snake_index] = (SIZE as u8, SIZE as u8);
        state.snake_lengths[*snake_index] = 0;
        state.snake_healths[*snake_index] = 0;
        state.are_snakes_alive[*snake_index] = false;
        for y in 0..SIZE{
            for x in 0..SIZE{
                let cell_value = state.board_struct[y][x];

                if cell_value != 0b11111111{
                    if (cell_value&0b10000000) >> 7 == *snake_index as u8{
                        state.board_struct[y][x] = 0;
                    }
                }
            }
        }
    }
}


pub fn state_value(state: &State, player: &u8) -> u32{
    if state.are_snakes_alive[0] && !state.are_snakes_alive[1]{
        return 0xFFFFFFFF;
    }
    if !state.are_snakes_alive[0] && state.are_snakes_alive[1]{
        return 0;
    }
    if !state.are_snakes_alive[0] && !state.are_snakes_alive[1]{
        return 0x7FFFFFFF;
    }



    let mut value : u32 = 0x80000000;



    //area control score
    let me_area = area_control_score::get_area_control_score(state, player).0 as u32;
    value += me_area;


    if state.turn >= 200{
        //enemy chasing tail score
        let tail_position : (u8, u8) = get_tail_position(state, &(1 - player));
        //println!("{:?}", tail_position);
        let dx = (tail_position.0 as i8 - state.snake_heads[1 - *player as usize].0 as i8).abs() as i8;
        let dy = (tail_position.1 as i8 - state.snake_heads[1 - *player as usize].1 as i8).abs() as i8;
        value += (SIZE as i8*2+1 - std::cmp::max((dx + dy)^(2), SIZE as i8*2+1)) as u32;
    }


    if state.turn >= 150{
        //giving no room for enemy score
        value += next_to_enemy_score(state, player);
    }



    let hungry = state.snake_healths[*player as usize] < 30 || state.turn < 200 || state.snake_lengths[*player as usize] < 20;
    if hungry && *player == 0{
        if state.snake_healths[*player as usize] < 30{
            value -= 10;
        }
        value += (size_value(state, player) + near_food_score(state, player));
    }
    
    return value as u32;

    fn get_tail_position(state : &State, player : &u8) -> (u8, u8){
        let mut tail_x = 11;
        let mut tail_y = 11;
        let mut smallest_seen = (SIZE*2+1) as u8;
        
        for y in 0..SIZE{
            for x in 0..SIZE{
                let cell_value = state.board_struct[y][x];
                if cell_value != 0b11111111{
                    if (cell_value&0b10000000) >> 7 == *player{
                        if (cell_value&0b01111111) < smallest_seen{
                            smallest_seen = cell_value&0b01111111;
                            tail_x = x as u8;
                            tail_y = y as u8;
                        }
                    }
                }
            }
        }

        return (tail_x, tail_y);
    }
    fn area_control_value(state : &State, player : &u8) -> u32{
        let (area_control_value, food_distances) = area_control_score::get_area_control_score(state, &0);
        return area_control_value as u32;
    }
    fn close_to_head_value(state : &State, player : &u8) -> u32{
        let mut head_distance_x = state.snake_heads[0].0 as i8 - state.snake_heads[1].0 as i8;
        if head_distance_x < 0{
            head_distance_x *= -1;
        }

        let mut head_distance_y = state.snake_heads[0].1 as i8 - state.snake_heads[1].1 as i8;
        if head_distance_y < 0{
            head_distance_y *= -1;
        }
        let head_distance = head_distance_x + head_distance_y;

        return (SIZE as i32*2 - head_distance as i32) as u32;
    }
    fn size_value(state : &State, player : &u8) -> u32{
        return state.snake_lengths[0] as u32 * SIZE as u32*9;
    }
    fn inside_head_value(state : &State, player : &u8) -> u32{
        return (state.snake_heads[0] == state.snake_heads[1]) as u32
    }
    fn near_food_score(state : &State, player : &u8) -> u32{
        //if hungry
        let mut value : u16 = SIZE as u16*5;
        let distances = area_control_score::get_area_control_score(state, &0).1;
        if distances[0].len() > 0{
            value -= (distances[0][0] as u16) * 2;
        }


        let mut smallest_distance : i8 = SIZE as i8*2;
        for y in 0..SIZE{
            for x in 0..SIZE{
                if state.board_struct[y][x] == 0b11111111{
                    let distance = (state.snake_heads[0].0 as i8 - x as i8).abs() + (state.snake_heads[0].1 as i8 - y as i8).abs() as i8;
                    if distance < smallest_distance{
                        smallest_distance = distance;
                    }
                }
            }
        }
        value -= smallest_distance as u16;
        return value as u32;
    }
    fn head_near_centre_score(state : &State, player : &u8) -> u32{
        let distance = ((SIZE/2+1) as i8 - state.snake_heads[0].0 as i8).abs() + ((SIZE/2+1) as i8 - state.snake_heads[0].1 as i8).abs();
        return ((SIZE/2+1) as i8 - distance) as u32;
    }
    fn head_at_edge_score(state : &State, player : &u8) -> u32{
        let mut score = 0;
        if state.snake_heads[0].0 == 0{
            score -= 1;
        }
        if state.snake_heads[0].1 == 0{
            score -= 1;
        }
        if state.snake_heads[0].0 == SIZE as u8-1{
            score -= 1;
        }
        if state.snake_heads[0].1 == SIZE as u8-1{
            score -= 1;
        }
        return score as u32;
    }
    fn next_to_enemy_score(state : &State, player : &u8) -> u32{
        let head_x = state.snake_heads[1-*player as usize].0 as usize;
        let head_y = state.snake_heads[1-*player as usize].1 as usize;

        let mut score = 0;
        if head_y < SIZE-1{
            if state.board_struct[head_y + 1][head_x]&0b10000000 > 0{
                score += 1;
            }
        }
        if head_y > 0{
            if state.board_struct[head_y - 1][head_x]&0b10000000 > 0{
                score += 1;
            }
        }
        if head_x > 0{
            if state.board_struct[head_y][head_x - 1]&0b10000000 > 0{
                score += 1;
            }
        }
        if head_x < SIZE-1{
            if state.board_struct[head_y][head_x + 1]&0b10000000 > 0{
                score += 1;
            }
        }

        return score as u32;
    }
}

pub fn print_state(state: &State){


    println!("STATE:\nSnakes Alive?: {:?}, {:?}", state.are_snakes_alive[0], state.are_snakes_alive[1]);
    println!("Lengths: {:?}, {:?}", state.snake_lengths[0], state.snake_lengths[1]);
    println!("Healths: {:?}, {:?}", state.snake_healths[0], state.snake_healths[1]);
    println!("Positions: {:?}, {:?}", state.snake_heads[0], state.snake_heads[1]);
    for y in 0..SIZE{
        for x in 0..SIZE{
            let mut cell_value = state.board_struct[SIZE - y - 1][x];

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
    println!("");}