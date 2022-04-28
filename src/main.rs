use std::io;
use rand::prelude::*;
use clap::{Arg, App};

struct Tictactoe {
    board: Vec<char>,
    turn: char,
    x_ai: bool,  // Is this player an AI?
    o_ai: bool,
    x_depth: i8, // How deep can we search?
    o_depth: i8,
    x_rand: u8, // How often should we choose a move as good (0 = never, 1 = 50%, 2 = 33%, 3 = 25%, etc)
    o_rand: u8,
    x_bad: u8, // How often should we just make a random move? (0 = never, 1 - 50%, 2 = 33%, 3 = 25%, etc)
    o_bad: u8
}

impl Tictactoe {

    fn new(x_ai: bool, o_ai: bool, x_depth: i8, o_depth: i8, x_rand: u8, o_rand: u8, x_bad: u8, o_bad: u8) -> Self {
        Self {
            board: [ ' ', ' ', ' ',
                     ' ', ' ', ' ',
                     ' ', ' ', ' ' ].to_vec(),
            turn: 'x',
            x_ai,
            o_ai,
            x_depth,
            o_depth,
            x_rand,
            o_rand,
            x_bad,
            o_bad,
        }
    }

    fn new_from(board: &Self) -> Self {
        Self {
            board: board.board.to_vec(),
            turn: board.turn,
            x_ai: board.x_ai,
            o_ai: board.o_ai,
            x_depth: board.x_depth,
            o_depth: board.o_depth,
            x_rand: board.x_rand,
            o_rand: board.o_rand,
            x_bad: board.x_bad,
            o_bad: board.o_bad,
        }
    }

    fn draw_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("{}", self.board[i*3+j]);
                if j != 2 {
                    print!("|");
                }
            }
            print!("\n");
        }
    }

    fn next_turn(&mut self) {
        if self.turn == 'x' {
            self.turn = 'o';
        }
        else {
            self.turn = 'x';
        }
    }

    fn do_move(&mut self, x: u8, y: u8) -> bool {
        let id: usize = (y*3+x).into();

        if x > 2 || y > 2 || self.board[id] != ' ' {
            return false;
        }
        self.board[id] = self.turn;

        self.next_turn();

        true
    }

    fn move_and_new_state(&mut self, spot: usize) -> Self {
        let mut board = Tictactoe::new_from(self);
        if spot >= board.board.len() {
            panic!("Invalid move, off the board!");
        }
        board.board[spot] = board.turn;
        board.next_turn();
        board
    }

    fn get_available_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();

        for i in 0..self.board.len() {
            if self.board[i] == ' ' {
                moves.push(i);
            }
        }

        moves
    }

    // Move to a random open position
    fn random_move(&mut self) {
        let moves = self.get_available_moves();
        let mut rng = rand::thread_rng();
        let move_id: usize = rng.gen_range(0..moves.len());
        let spot: u8 = moves[move_id].try_into().unwrap();

        self.do_move(spot%3, spot/3);
    }

    fn ask_for_move(&mut self) {
        loop {
            println!("Please enter your move like 'x,y':");
            let mut my_move = String::new();
            io::stdin()
                .read_line(&mut my_move)
                .expect("Failed to read move");
            if my_move.len() == 4 {
                // Compiler says no support for chaining if lets as of April 2022: https://github.com/rust-lang/rust/issues/53667
                if let Ok(x) = my_move[0..1].parse() {
                    if let Ok(y) = my_move[2..3].parse() {
                        if self.do_move(x, y) {
                            break;
                        }
                    }
                }
            }
            println!("You entered an invalid move");
        }
    }

    fn check_for_win(&self) -> bool {
        // Check rows
        for y in 0..3 {
            let player = self.board[y*3];

            if player == ' ' { 
                continue;
            }

            for x in 1..3 {
                if self.board[y*3+x] != player {
                    break;
                }
                if x == 2 {
                    return true;
                }
            }
        }

        // Check columns
        for x in 0..3 {
            let player = self.board[x];

            if player == ' ' {
                continue;
            }

            for y in 1..3 {
                if self.board[y*3+x] != player {
                    break;
                }

                if y == 2 {
                    return true;
                }
            }
        }

        // Check diagonals
        let player = self.board[1*3+1]; // Center

        if player == ' ' {
            return false
        }

        if (self.board[0*3+0] == player && self.board[2*3+2] == player)
        || (self.board[0*3+2] == player && self.board[2*3+0] == player) {
            return true;
        }

        return false;
    }

    fn min_max(&mut self, max_depth: i32, depth: i32, player_id: char, mut board: Tictactoe) -> (i8, Option<u8>) {
        if board.check_for_win() { // End goal
            if board.turn == player_id {
                return (1, None);
            }
            else {
                return (-1, None);
            }
        }

        let moves = board.get_available_moves();

        if moves.len() == 0 { // No moves left, so tie
            return (0, None);
        }

        let randomness = if self.turn == 'x' {
            self.x_rand
        } else {
            self.o_rand
        };

        let mut max_score = -10; // Overwritten
        let mut best_move = 99;

        if depth == max_depth {
            return (max_score, None);
        }

        if depth > 8 {
            panic!("Too deep man!");
        }

        let mut rng = rand::thread_rng();
        for i in 0..moves.len() {

            let board = board.move_and_new_state(moves[i]);
            let next_player = if player_id == 'o' {
                'x'
            } else {
                'o'
            };
            let (score, next_move) = self.min_max(max_depth, depth+1, next_player, board);

            if score > max_score 
            || (score == max_score && randomness != 0 && rng.gen_range(0..randomness+1) == 1) {
                max_score = score;
                best_move = moves[i];
            }
        }

        (-1*max_score, Some(best_move.try_into().unwrap()))
    }

    // max_depth = -1 = infinite/till stack overflows
    fn min_max_move(&mut self, max_depth: i32, player_id: char) {
       let (score, next_move) = self.min_max(max_depth, 0, player_id, Tictactoe::new_from(self));
       if !next_move.is_none() {
            let spot = next_move.unwrap();
            if spot == 99 {
                self.random_move();
                return;
            }
            if !self.do_move(spot%3, spot/3) {
                panic!("Invalid move!");
            }
       }
       else {
            panic!("No moves left!");
       }
    }

    fn game_loop(&mut self, verbose: bool) {
        loop {
            let this_turn = self.turn;

            if verbose {
                println!("It is Player {}'s turn", this_turn);
            }

            if (this_turn == 'x' && self.x_ai) || (this_turn == 'o' && self.o_ai) {

                let depth = if this_turn == 'x' {
                    self.x_depth
                } else {
                    self.o_depth
                }.into();

                let randomness = if self.turn == 'x' {
                    self.x_rand
                } else {
                    self.o_rand
                };

                // How often to choose a random move
                let badness = if this_turn == 'x' {
                    self.x_bad
                } else {
                    self.o_bad
                };

                let mut rng = rand::thread_rng();

                if verbose {
                     println!("AI (Depth: {}, rand: {}, bad: {}) is thinking...", depth, randomness, badness);
                }

                if badness != 0 && rng.gen_range(0..badness+1) == 1 {
                    self.random_move()
                }
                else {
                    self.min_max_move(depth, this_turn);

                }
            }
            else {
                self.ask_for_move();
            }
            if verbose {
                self.draw_board();
            }
            let is_win = self.check_for_win();
            if is_win {
                println!("Player {} has won!", this_turn);
                break;
            }
            if self.get_available_moves().len() == 0 {
                println!("Tie!");
                break;
            }
        }
    }
}

fn main() {
    let args = App::new("Tictactoe min-max AI")
                        .version("0.0.1")
                        .author("James Danielson (GeekyLink) geekylink@github.io")
                        .about("Testing out a min-max algo")
                        .arg(Arg::with_name("no-verbose")
                                    .long("no-verbose")
                                    .short("n")
                                    .help("Only print the final line of a game")
                        )
                        .arg(Arg::with_name("iterations")
                                    .long("it")
                                    .help("How many games to play")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("x-ai")
                                    .short("x")
                                    .long("x-ai")
                                    .help("Is Player X an AI?")
                        )
                        .arg(Arg::with_name("o-ai")
                                    .short("o")
                                    .long("o-ai")
                                    .help("Is Player O an AI?")
                        )
                        .arg(Arg::with_name("x-depth")
                                    .long("x-depth")
                                    .help("How deep will X search? (Default: infinite)")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("o-depth")
                                    .long("o-depth")
                                    .help("How deep will O search? (Default: infinite)")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("x-rand")
                                    .long("x-rand")
                                    .help("How often should X choose a move just as good? (0 = never, 1 = 50%, 2 = 33%, etc)")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("o-rand")
                                    .long("o-rand")
                                    .help("How often should O choose a move just as good? (0 = never, 1 = 50%, 2 = 33%, etc)")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("x-bad")
                                    .long("x-bad")
                                    .help("How often should X choose a random move? (0 = never, 1 = 50%, 2 = 33%, etc)")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("o-bad")
                                    .long("o-bad")
                                    .help("How often should O choose a random move? (0 = never, 1 = 50%, 2 = 33%, etc)")
                                    .takes_value(true)
                        )
                        .get_matches();

    let x_ai = args.is_present("x-ai");
    let o_ai = args.is_present("o-ai");

    // If depth can't be parsed, just use -1
    let x_depth = args.value_of("x-depth").unwrap_or("").parse().unwrap_or(-1);
    let o_depth = args.value_of("o-depth").unwrap_or("").parse().unwrap_or(-1);

    let x_rand = args.value_of("x-rand").unwrap_or("").parse().unwrap_or(0);
    let o_rand = args.value_of("o-rand").unwrap_or("").parse().unwrap_or(0);

    let x_bad = args.value_of("x-bad").unwrap_or("").parse().unwrap_or(0);
    let o_bad = args.value_of("o-bad").unwrap_or("").parse().unwrap_or(0);

    let verbose = !args.is_present("no-verbose");
    let iterations = args.value_of("iterations").unwrap_or("").parse().unwrap_or(1);

    if x_depth == 0 || o_depth == 0 || x_depth < -1 || o_depth < -1 {
        println!("Max depth cannot be 0, or less than -1");
    }
    else if iterations == 0 {
        println!("Must be at least one iteration!");
    }
    else {
        for i in 0..iterations {
            let mut game = Tictactoe::new(x_ai, o_ai, x_depth, o_depth, x_rand, o_rand, x_bad, o_bad);
            game.game_loop(verbose);
        }
    }
}
