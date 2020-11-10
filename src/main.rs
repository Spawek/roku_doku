use roku_doku::brick::{print_brick, random_brick, xy, Brick, XY};
use roku_doku::resolve::resolve_board;
use roku_doku::Board;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
struct GameState {
    board: Board,
    available_bricks: Vec<Brick>,
    points: i32,
    last_move_was_match: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            board: Board::default(),
            points: 0,
            last_move_was_match: false,
            available_bricks: vec![],
        }
    }
}

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
        joined_brick_offsets.extend(brick.offsets.iter().map(|v| xy(v.x + x_offset, v.y)));
        x_offset = x_offset + brick.offsets.iter().max_by_key(|v| v.x).unwrap().x + 3;
    }

    print_brick(&Brick {
        offsets: joined_brick_offsets,
    });
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
    // assert!(can_put_brick(&board, &brick, &pos));
    // DEBUG
    if !can_put_brick(&board, &brick, &pos){
        println!("FATAL - PUTTING BRICK IN INVALID PLACE: PRINTING STATE BEFORE LEAVING");
        print_board(&board);
        print_brick(&brick);
        dbg!(pos);
        panic!();
    }

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
            let pos = xy(x, y);
            if can_put_brick(board, brick, &pos) {
                ret.push(pos);
            }
        }
    }

    ret
}

#[derive(Debug)]
struct Move {
    // 0-based index
    brick_index: i32,
    // 0-based axes
    pos: XY,
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
                } else {
                    print!(
                        "only bricks {:?} are available",
                        0..game_state.available_bricks.len()
                    );
                    continue;
                }
            }
            Err(..) => {
                println!("first value should be an integer: {}", first_val);
                continue;
            }
        };

        let second_val = split.get(1).unwrap().trim();
        if second_val.len() != 2 {
            println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
            continue;
        }

        let x_char = second_val.chars().nth(0).unwrap();
        let x_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let x_index = x_letters.iter().position(|c| c == &x_char);
        let x = match x_index {
            Some(x) => x,
            None => {
                println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                continue;
            }
        };

        let y_char = second_val.chars().nth(1).unwrap();
        let y = match y_char.to_string().parse::<i32>() {
            Ok(i) => {
                if i > 0 && i <= 9 {
                    i - 1
                } else {
                    println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                    continue;
                }
            }
            Err(..) => {
                println!("second part of the input should contains a letter a-i followed by a digit 1-9 - e.g. `d3`");
                continue;
            }
        };

        let brick = game_state
            .available_bricks
            .get(brick_index as usize)
            .unwrap();
        let pos = xy(x as i32, y);
        if !can_put_brick(&game_state.board, &brick, &pos) {
            println!(
                "the brick ({}) can't be put in the position you selected ({})",
                &first_val, &second_val
            );
            continue;
        }

        return Move { brick_index, pos };
    }
}

fn newly_filled_cells_count(new_board: &Board, old_board: &Board) -> i32 {
    let mut ret = 0;
    for x in 0..9 {
        for y in 0..9 {
            ret = ret
                + if new_board[y][x] && !old_board[y][x] {
                    1
                } else {
                    0
                }
        }
    }

    ret
}

// Points:
// 18 points per line
// 1 point for each cell that is filled after current move, that wasn't filled before current move
// if previous move was a line +9 points extra (streak)

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
    let new_cells_points = newly_filled_cells_count(&board, &game_state.board);
    let bonus_points = resolve_result.blocks_removed * 18 + streak_points + new_cells_points;

    GameState {
        board,
        points: game_state.points + bonus_points,
        last_move_was_match: resolve_result.blocks_removed > 0,
        available_bricks,
    }
}

#[derive(Clone, Debug)]
struct PossibleMove {
    brick: Brick,
    brick_index: i32,
    pos: XY,
}

impl From<PossibleMove> for Move {
    fn from(m: PossibleMove) -> Self {
        Move{pos: m.pos, brick_index: m.brick_index}
    }
}

fn get_possible_moves(game_state: &GameState) -> Vec<PossibleMove> {
    let mut moves = vec![];
    let mut brick_index = 0;
    for brick in &game_state.available_bricks {
        moves.extend(
            possible_moves(&game_state.board, &brick)
                .iter()
                .map(|pos| PossibleMove {
                    brick: brick.clone(),
                    brick_index,
                    pos: pos.clone(),
                })
                .collect::<Vec<_>>(),
        );
        brick_index = brick_index + 1;
    }

    moves
}

#[derive(Clone, Debug)]
struct PossibleMoveScore {
    possible_move: PossibleMove,
    score: i32,
}

impl From<PossibleMoveScore> for Move {
    fn from(m: PossibleMoveScore) -> Self {
        m.possible_move.into()
    }
}

fn count_filled_cells(board: &Board) -> i32 {
    board.iter()
        .map(|row| row.iter()
            .map(|cell| if *cell { 1 } else { 0 }).sum::<i32>())
        .sum()
}

// tests avg score on 100 games:
// random move: 74
// min brick_index, then min x, then min y: 182
// do move which gives the most points: 458
// do 3 moves which gives the most points: 1858
// do 3 moves which gives the most points + subtract 2 points for each filled cell:
fn ai_move(game_state: &GameState) -> PossibleMoveScore {
    let mut possible_moves = vec![];
    for m in get_possible_moves(game_state) {
        let s = perform_move(&game_state, &m.clone().into());
        if s.available_bricks.is_empty(){
            possible_moves.push(PossibleMoveScore{
                possible_move: m.clone(),
                score: s.points - count_filled_cells(&s.board) * 2});
            continue;
        }

        let moves2 = get_possible_moves(&s);
        if moves2.is_empty() {
            possible_moves.push(PossibleMoveScore{possible_move: m.clone(), score: s.points - 1000});
            continue;
        }

        let best_sub_move = ai_move(&s);
        possible_moves.push(PossibleMoveScore{possible_move: m.clone(), score: best_sub_move.score});
    }

    possible_moves.into_iter().max_by_key(|x| x.score).unwrap()
}

fn main() {
    // print_all_bricks();

    let mut scores = vec![];
    for _ in 0..100 {
        let mut move_counter = 0;
        let mut game_state = GameState::new();
        loop {
            if game_state.available_bricks.is_empty() {
                game_state.available_bricks = vec![random_brick(), random_brick(), random_brick()];
            }

            // print_game_state(&game_state);

            let possible_move_count = game_state
                .available_bricks
                .iter()
                .map(|brick| possible_moves(&&game_state.board, brick).len())
                .sum::<usize>();
            if possible_move_count == 0 {
                println!("game over!\n your score: {} (in {} moves)", game_state.points, &move_counter);
                scores.push(game_state.points);
                break;
            } else {
                // println!("\nnumber of possible moves: {}", &possible_move_count);
            }

            // let user_move = read_user_move(&game_state);
            let user_move = ai_move(&game_state).into();
            // println!("executing move: {:?}", &user_move);

            game_state = perform_move(&game_state, &user_move);
            move_counter = move_counter + 1;

            if move_counter % 100 == 0 && move_counter != 0 {
                println!("{} moves done", move_counter);
            }
        }
    }

    println!("final scores: {:#?}", scores);
    println!("min score: {:#?}", scores.iter().min().unwrap());
    println!("max score: {:#?}", scores.iter().max().unwrap());
    println!("avg score: {:#?}", scores.iter().sum::<i32>() / scores.len() as i32);
}

// TODO: write some macro to disable printing (as it takes a lot of time)
// TODO: do profiling
// TODO: multithreading
// TODO: penalty for "holes"