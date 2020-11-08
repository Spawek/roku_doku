use roku_doku::resolve::resolve_board;
use roku_doku::Board;
use roku_doku::brick::{random_brick, print_brick, Brick, xy, XY};

#[derive(Debug, Clone)]
struct GameState {
    board: Board,
    available_bricks: Vec<Brick>,
    points: i32,
    last_move_was_match: bool,
}

impl GameState{
    fn new() -> GameState{
        GameState{
            board: Board::default(),
            points: 0,
            last_move_was_match: false,
            available_bricks: vec![]
        }
    }
}

// Original points:
// 20 points per line (sometimes it's 19 - no idea why).
// if previous move was a line +9 points extra (streak)
// 2 lines / line + block at once = 37
// 3 lines = ???

// Points here (simplified):
// 20 points per line
// if previous move was a line +10 points extra (streak)

fn print_board(board: &Board) {
    let mut row_counter = 0;
    println!("  abc def ghi");
    for row in board {
        if row_counter % 3 == 0 {
            println!(" -------------");
        }

        print!("{}", row_counter + 1);
        let mut col_counter = 0;
        for cell in row {
            if col_counter % 3 == 0 {
                print!("|");
            }
            let c = if *cell { "X" } else { "." };
            print!("{}", c);
            col_counter = col_counter + 1;
        }
        println!("|{}", row_counter + 1);

        row_counter = row_counter + 1;
    }
    println!(" -------------");
    println!("  abc def ghi");
}

fn print_bricks(bricks: &Vec<Brick>) {
    println!();

    // let's display 3 bricks as 1 big brick
    let mut joined_brick_offsets = vec![];
    let mut x_offset = 0;
    for brick in bricks {
        joined_brick_offsets.extend(brick.offsets.iter().map(|v|xy(v.x + x_offset, v.y)));
        x_offset = x_offset + brick.offsets.iter().max_by_key(|v| v.x).unwrap().x + 3;
    }

    print_brick(&Brick{offsets: joined_brick_offsets});
}

fn print_game_state(game_state: &GameState) {
    println!("\ncurrent points: {}\n", game_state.points);
    print_board(&game_state.board);
    print_bricks(&game_state.available_bricks);
}

fn can_put_brick(board: &Board, brick: &Brick, pos: &XY) -> bool {
    assert!(pos.x >= 0);
    assert!(pos.y >= 0);

    let min_x = brick.offsets.iter().min_by_key(|v| v.x).unwrap().x;
    assert!(min_x >= 0);

    let min_y = brick.offsets.iter().min_by_key(|v| v.y).unwrap().y;
    assert!(min_y >= 0);

    let max_x = brick.offsets.iter().max_by_key(|v| v.x).unwrap().x;
    let max_y = brick.offsets.iter().max_by_key(|v| v.y).unwrap().y;
    if pos.x + max_x > 8 || pos.y + max_y > 8 {
        return false;
    }

    for offset in &brick.offsets {
        if board[(pos.y + offset.y) as usize][(pos.x + offset.x) as usize] {
            return false;
        }
    }

    return true;
}

fn put_brick(board: &Board, brick: &Brick, pos: &XY) -> Board {
    assert!(can_put_brick(&board, &brick, &pos));

    let mut new_board = board.clone();
    for offset in &brick.offsets {
        new_board[(pos.y + offset.y) as usize][(pos.x + offset.x) as usize] = true;
    }

    new_board
}

fn possible_moves(board: &Board, brick: &Brick) -> Vec<XY> {
    let mut ret = vec![];
    for x in 0..9 {
        for y in 0..9 {
            let pos = xy(x,y);
            if can_put_brick(board, brick, &pos){
                ret.push(pos);
            }
        }
    }

    ret
}

#[derive(Debug)]
struct Move {
    // 0-base index
    brick_index : i32,
    // 0-base axes
    pos : XY,
}

fn read_user_move(game_state: &GameState) -> Move {
    loop {
        println!("\ntype a move in form `brick_no position` - e.g. `3 d4`");
        let mut input_text = String::new();
        std::io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let split = input_text.split(' ').collect::<Vec<_>>();
        if split.len() != 2 {
            println!("input should have 2 parts delimited with space - e.g. `0 d3`");
            continue;
        }

        let first_val = split.get(0).unwrap().trim();
        let brick_index = match first_val.parse::<i32>() {
            Ok(i) => {
                if i > 0 && i <= game_state.available_bricks.len() as i32 {
                    i - 1 // translation from 1-based to 0-based indexing
                }
                else {
                    print!("only bricks {:?} are available", 0..game_state.available_bricks.len());
                    continue;
                }
            }
            Err(..) => {
                println!("first value should be an integer: {}", first_val);
                continue;
            } ,
        };

        let second_val = split.get(1).unwrap().trim();
        if second_val.len() != 2 {
            println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
            continue;
        }

        let x_char= second_val.chars().nth(0).unwrap();
        let x_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let x_index = x_letters.iter().position(|c| c == &x_char);
        let x = match x_index {
            Some(x) => x,
            None => {
                println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                continue;
            }
        };

        let y_char= second_val.chars().nth(1).unwrap();
        let y = match y_char.to_string().parse::<i32>() {
            Ok(i) => { if i > 0 && i <= 9 { i - 1 } else {
                println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                continue;
            } }
            Err(..) => {
                println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                continue;
            },
        };

        let brick = game_state.available_bricks.get(brick_index as usize).unwrap();
        let pos = xy(x as i32, y);
        if !can_put_brick(&game_state.board, &brick, &pos) {
            println!("the brick ({}) can't be put in the position you selected ({})", &first_val, &second_val);
            continue;
        }

        return Move { brick_index, pos };
    }
}


fn perform_move(game_state: &GameState, m: &Move) -> GameState {
    assert!(m.brick_index >= 0);
    assert!(m.brick_index < game_state.available_bricks.len() as i32);

    let mut available_bricks = game_state.available_bricks.clone();
    let brick = available_bricks.remove(m.brick_index as usize);
    let board = put_brick(&game_state.board, &brick, &m.pos);
    let resolve_result = resolve_board(board);
    let board = resolve_result.board;

    let streak = game_state.last_move_was_match && resolve_result.blocks_removed > 0;
    let streak_points = if streak { 9 } else { 0 };

    GameState {
        board,
        points: game_state.points + resolve_result.blocks_removed * 20 + streak_points,
        last_move_was_match: resolve_result.blocks_removed > 0,
        available_bricks,
    }
}

fn main() {
    // print_all_bricks();

    let mut game_state = GameState::new();

    loop {
        if game_state.available_bricks.is_empty() {
            game_state.available_bricks = vec![random_brick(), random_brick(), random_brick()];
        }

        print_game_state(&game_state);

        let possible_move_count = game_state.available_bricks.iter().map(|brick| possible_moves(&&game_state.board, brick).len()).sum::<usize>();
        if possible_move_count == 0 {
            println!("game over!\n your score: {}", game_state.points);
        }
        else {
            println!("\nnumber of possible moves: {}", &possible_move_count);
        }

        let user_move = read_user_move(&game_state);
        println!("executing move: {:?}", &user_move);
        game_state = perform_move(&game_state, &user_move);
    }
    // println!("possible moves: {:#?}", possible_moves(&game_state.board, &game_state.available_bricks[0]));
}
