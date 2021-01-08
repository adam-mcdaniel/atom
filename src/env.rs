use std::{
    thread::sleep,
    time::Duration,
    collections::BTreeMap,
    process::exit,
    path::{PathBuf, Component},
    env::current_exe,
    io::{stdin, stdout, Write},
    fs::{read_to_string, write}
};
use super::{Value, Error, VERSION, PRELUDE_FILENAME};

use rand::{seq::SliceRandom, Rng, thread_rng, distributions::Uniform};
use chrono::{Local, Timelike, Datelike};
use os_info::Type;
use asciicolor::Colorize;

use lazy_static::lazy_static;
lazy_static! {
    static ref SPADES: Vec<Value> = {
        vec![
            Value::string("🂡"),
            Value::string("🂢"),
            Value::string("🂣"),
            Value::string("🂤"),
            Value::string("🂥"),
            Value::string("🂦"),
            Value::string("🂧"),
            Value::string("🂨"),
            Value::string("🂩"),
            Value::string("🂪"),
            Value::string("🂫"),
            Value::string("🂭"),
            Value::string("🂮")
        ]
    };
    
    static ref HEARTS: Vec<Value> = {
        vec![
            Value::string("🂱"),
            Value::string("🂲"),
            Value::string("🂳"),
            Value::string("🂴"),
            Value::string("🂵"),
            Value::string("🂶"),
            Value::string("🂷"),
            Value::string("🂸"),
            Value::string("🂹"),
            Value::string("🂺"),
            Value::string("🂻"),
            Value::string("🂽"),
            Value::string("🂾")
        ]
    };

    static ref DIAMONDS: Vec<Value> = {
        vec![
            Value::string("🃁"),
            Value::string("🃂"),
            Value::string("🃃"),
            Value::string("🃄"),
            Value::string("🃅"),
            Value::string("🃆"),
            Value::string("🃇"),
            Value::string("🃈"),
            Value::string("🃉"),
            Value::string("🃊"),
            Value::string("🃋"),
            Value::string("🃍"),
            Value::string("🃎")
        ]
    };

    static ref CLUBS: Vec<Value> = {
        vec![
            Value::string("🃑"),
            Value::string("🃒"),
            Value::string("🃓"),
            Value::string("🃔"),
            Value::string("🃕"),
            Value::string("🃖"),
            Value::string("🃗"),
            Value::string("🃘"),
            Value::string("🃙"),
            Value::string("🃚"),
            Value::string("🃛"),
            Value::string("🃝"),
            Value::string("🃞")
        ]
    };

    static ref CARDS: Value = {
        let mut cards = BTreeMap::new();
        let mut deck = BTreeMap::new();
        deck.insert("all".to_string(), Value::List(vec![
            Value::string("🂡"),
            Value::string("🂱"),
            Value::string("🃁"),
            Value::string("🃑"),
            
            Value::string("🂢"),
            Value::string("🂲"),
            Value::string("🃂"),
            Value::string("🃒"),
            
            Value::string("🂣"),
            Value::string("🂳"),
            Value::string("🃃"),
            Value::string("🃓"),
            
            Value::string("🂤"),
            Value::string("🂴"),
            Value::string("🃄"),
            Value::string("🃔"),
            
            Value::string("🂥"),
            Value::string("🂵"),
            Value::string("🃅"),
            Value::string("🃕"),
            
            Value::string("🂦"),
            Value::string("🂶"),
            Value::string("🃆"),
            Value::string("🃖"),
            
            Value::string("🂧"),
            Value::string("🂷"),
            Value::string("🃇"),
            Value::string("🃗"),
            
            Value::string("🂨"),
            Value::string("🂸"),
            Value::string("🃈"),
            Value::string("🃘"),
            
            Value::string("🂩"),
            Value::string("🂹"),
            Value::string("🃉"),
            Value::string("🃙"),
            
            Value::string("🂪"),
            Value::string("🂺"),
            Value::string("🃊"),
            Value::string("🃚"),
            
            Value::string("🂫"),
            Value::string("🂻"),
            Value::string("🃋"),
            Value::string("🃛"),
            
            Value::string("🂭"),
            Value::string("🂽"),
            Value::string("🃍"),
            Value::string("🃝"),
            
            Value::string("🂮"),
            Value::string("🂾"),
            Value::string("🃎"),
            Value::string("🃞"),
        ]));


        deck.insert("aces".to_string(), Value::List(vec![
            Value::string("🂡"),
            Value::string("🂱"),
            Value::string("🃁"),
            Value::string("🃑"),
        ]));

        deck.insert("kings".to_string(), Value::List(vec![
            Value::string("🂮"),
            Value::string("🂾"),
            Value::string("🃎"),
            Value::string("🃞"),
        ]));

        deck.insert("queens".to_string(), Value::List(vec![
            Value::string("🂭"),
            Value::string("🂽"),
            Value::string("🃍"),
            Value::string("🃝"),
        ]));

        deck.insert("jacks".to_string(), Value::List(vec![
            Value::string("🂫"),
            Value::string("🂻"),
            Value::string("🃋"),
            Value::string("🃛"),
        ]));

        deck.insert("faces".to_string(), Value::List(vec![
            Value::string("🂫"),
            Value::string("🂻"),
            Value::string("🃋"),
            Value::string("🃛"),
            Value::string("🂭"),
            Value::string("🂽"),
            Value::string("🃍"),
            Value::string("🃝"),
            Value::string("🂮"),
            Value::string("🂾"),
            Value::string("🃎"),
            Value::string("🃞"),
        ]));

        deck.insert("numbers".to_string(), Value::List(vec![
            Value::string("🂢"),
            Value::string("🂲"),
            Value::string("🃂"),
            Value::string("🃒"),
            
            Value::string("🂣"),
            Value::string("🂳"),
            Value::string("🃃"),
            Value::string("🃓"),
            
            Value::string("🂤"),
            Value::string("🂴"),
            Value::string("🃄"),
            Value::string("🃔"),
            
            Value::string("🂥"),
            Value::string("🂵"),
            Value::string("🃅"),
            Value::string("🃕"),
            
            Value::string("🂦"),
            Value::string("🂶"),
            Value::string("🃆"),
            Value::string("🃖"),
            
            Value::string("🂧"),
            Value::string("🂷"),
            Value::string("🃇"),
            Value::string("🃗"),
            
            Value::string("🂨"),
            Value::string("🂸"),
            Value::string("🃈"),
            Value::string("🃘"),
            
            Value::string("🂩"),
            Value::string("🂹"),
            Value::string("🃉"),
            Value::string("🃙"),
            
            Value::string("🂪"),
            Value::string("🂺"),
            Value::string("🃊"),
            Value::string("🃚"),
        ]));

        deck.insert("spades".to_string(), Value::List(SPADES.clone()));
        deck.insert("hearts".to_string(), Value::List(HEARTS.clone()));
        deck.insert("diamonds".to_string(), Value::List(DIAMONDS.clone()));
        deck.insert("clubs".to_string(), Value::List(CLUBS.clone()));
        cards.insert("deck".to_string(), Value::Table(deck));
        
        let mut suites = BTreeMap::new();
        suites.insert("spades".to_string(),   Value::string("♠"));
        suites.insert("clubs".to_string(),    Value::string("♣"));
        suites.insert("hearts".to_string(),   Value::string("♥"));
        suites.insert("diamonds".to_string(), Value::string("♦"));
        cards.insert("suites".to_string(), Value::Table(suites));

        cards.insert("suite".to_string(), Value::builtin("cards@suite", |args, env| {
            check_args_len(Value::symbol("cards@suite"), &args, 1)?;
            let card = args[0].eval(env)?;
            Ok(if SPADES.contains(&card) {
                Value::string("♠")
            } else if HEARTS.contains(&card) {
                Value::string("♥")
            } else if DIAMONDS.contains(&card) {
                Value::string("♦")
            } else if CLUBS.contains(&card) {
                Value::string("♣")
            } else {
                return Err(Error::CustomError(format!("{} does not belong to any suite", card)))
            })
        }));

        fn value(card: &Value) -> Result<i32, Error> {
            if let Some(i) = SPADES.iter().position(|x| x.clone() == card.clone()) {
                Ok((i + 1) as i32)
            } else if let Some(i) = HEARTS.iter().position(|x| x.clone() == card.clone()) {
                Ok((i + 1) as i32)
            } else if let Some(i) = DIAMONDS.iter().position(|x| x.clone() == card.clone()) {
                Ok((i + 1) as i32)
            } else if let Some(i) = CLUBS.iter().position(|x| x.clone() == card.clone()) {
                Ok((i + 1) as i32)
            } else {
                return Err(Error::CustomError(format!("{} does not belong to any suite", card)))
            }
        }

        cards.insert("value".to_string(), Value::builtin("cards@value", |args, env| {
            check_args_len(Value::symbol("cards@value"), &args, 1)?;
            let card = args[0].eval(env)?;
            Ok(Value::Integer(value(&card)?))
        }));

        cards.insert("name".to_string(), Value::builtin("cards@name", |args, env| {
            check_args_len(Value::symbol("cards@name"), &args, 1)?;
            let card = args[0].eval(env)?;
            Ok(Value::string(
                format!("{} of {}",
                    match value(&card)? {
                        1 => "ace",
                        2 => "two",
                        3 => "three",
                        4 => "four",
                        5 => "five",
                        6 => "six",
                        7 => "seven",
                        8 => "eight",
                        9 => "nine",
                        10 => "ten",
                        11 => "jack",
                        12 => "queen",
                        13 => "king",
                        _ => return Err(Error::CustomError(format!("invalid card value for {}", card)))
                    },
                    if SPADES.contains(&card) {
                        "spades"
                    } else if HEARTS.contains(&card) {
                        "hearts"
                    } else if DIAMONDS.contains(&card) {
                        "diamonds"
                    } else if CLUBS.contains(&card) {
                        "clubs"
                    } else {
                        return Err(Error::CustomError(format!("{} does not belong to any suite", card)))
                    }
                )
            ))
        }));

        cards.insert("from-name".to_string(), Value::builtin("cards@from-name", |args, env| {
            check_args_len(Value::symbol("cards@from-name"), &args, 1)?;
            let name = args[0].eval(env)?.to_string();
            let words = name.trim().split_whitespace().map(ToString::to_string).collect::<Vec<String>>();

            if words.len() != 3 || words[1] != "of" {
                return Err(Error::CustomError(format!("\"{}\" is not a valid card name", name)))
            }

            let val = match words[0].as_str() {
                "ace" => 1,
                "two" | "2" => 2,
                "three" | "3" => 3,
                "four" | "4" => 4,
                "five" | "5" => 5,
                "six" | "6" => 6,
                "seven" | "7" => 7,
                "eight" | "8" => 8,
                "nine" | "9" => 9,
                "ten" | "10" => 10,
                "jack" => 11,
                "queen" => 12,
                "king" => 13,
                e => return Err(Error::CustomError(format!("invalid card value \"{}\"", e)))
            };

            let suite = match words[2].as_str() {
                "spades"   => SPADES.clone(),
                "hearts"   => HEARTS.clone(),
                "diamonds" => DIAMONDS.clone(),
                "clubs"    => CLUBS.clone(),
                e => return Err(Error::CustomError(format!("invalid card suite \"{}\"", e)))
            };
            
            Ok(Value::string(suite[val-1].clone()))
        }));

        cards.insert("back".to_string(), Value::string("🂠"));
        
        Value::Table(cards)
    };

    static ref CHESS: Value = {
        let mut chess = BTreeMap::new();
        let mut white = BTreeMap::new();
        let mut black = BTreeMap::new();

        white.insert("king".to_string(),   Value::string("♔"));
        white.insert("queen".to_string(),  Value::string("♕"));
        white.insert("rook".to_string(),   Value::string("♖"));
        white.insert("bishop".to_string(), Value::string("♗"));
        white.insert("knight".to_string(), Value::string("♘"));
        white.insert("pawn".to_string(),   Value::string("♙"));
        chess.insert("white".to_string(),  Value::Table(white));
        
        black.insert("king".to_string(),   Value::string("♚"));
        black.insert("queen".to_string(),  Value::string("♛"));
        black.insert("rook".to_string(),   Value::string("♜"));
        black.insert("bishop".to_string(), Value::string("♝"));
        black.insert("knight".to_string(), Value::string("♞"));
        black.insert("pawn".to_string(),   Value::string("♟"));
        chess.insert("black".to_string(),  Value::Table(black));

        chess.insert("space".to_string(),  Value::string("."));

        fn is_piece(piece: &String) -> bool {
            ["♚", "♛", "♜", "♝", "♞", "♟", "♔", "♕", "♖", "♗", "♘", "♙"].contains(&piece.as_str())
        }

        fn is_white(piece: &String) -> bool {
            ["♔", "♕", "♖", "♗", "♘", "♙"].contains(&piece.as_str())
        }

        fn is_black(piece: &String) -> bool {
            ["♚", "♛", "♜", "♝", "♞", "♟"].contains(&piece.as_str())
        }

        fn is_space(piece: &String) -> bool {
            !is_piece(piece)
        }

        chess.insert("is-piece".to_string(), Value::builtin("chess@is-piece", |args, env| {
            check_args_len(Value::Symbol("chess@is-piece".to_string()), &args, 1)?;
            if let Value::String(piece) = args[0].eval(env)? {
                Ok(Value::Boolean(is_piece(&piece)))
            } else {
                Err(Error::InvalidArguments(Value::Symbol("chess@is-piece".to_string()), args.clone()))
            }
        }));

        chess.insert("is-space".to_string(), Value::builtin("chess@is-space", |args, env| {
            check_args_len(Value::Symbol("chess@is-space".to_string()), &args, 1)?;
            if let Value::String(piece) = args[0].eval(env)? {
                Ok(Value::Boolean(is_space(&piece)))
            } else {
                Err(Error::InvalidArguments(Value::Symbol("chess@is-space".to_string()), args.clone()))
            }
        }));

        chess.insert("is-white".to_string(), Value::builtin("chess@is-white", |args, env| {
            check_args_len(Value::Symbol("chess@is-white".to_string()), &args, 1)?;
            if let Value::String(piece) = args[0].eval(env)? {
                Ok(Value::Boolean(is_white(&piece)))
            } else {
                Err(Error::InvalidArguments(Value::Symbol("chess@is-white".to_string()), args.clone()))
            }
        }));

        chess.insert("is-black".to_string(), Value::builtin("chess@is-black", |args, env| {
            check_args_len(Value::Symbol("chess@is-black".to_string()), &args, 1)?;
            if let Value::String(piece) = args[0].eval(env)? {
                Ok(Value::Boolean(is_black(&piece)))
            } else {
                Err(Error::InvalidArguments(Value::Symbol("chess@is-black".to_string()), args.clone()))
            }
        }));

        chess.insert("create".to_string(), Value::builtin("chess@create", |args, _| {
            check_args_len(Value::symbol("chess@create"), &args, 0)?;
            Ok(Value::List(vec![
                Value::List(["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"].iter().map(Value::string).collect()),
                Value::List(vec!["♟"].repeat(8).iter().map(Value::string).collect()),
                Value::List(vec!["."].repeat(8).iter().map(Value::string).collect()),
                Value::List(vec!["."].repeat(8).iter().map(Value::string).collect()),
                Value::List(vec!["."].repeat(8).iter().map(Value::string).collect()),
                Value::List(vec!["."].repeat(8).iter().map(Value::string).collect()),
                Value::List(vec!["♙"].repeat(8).iter().map(Value::string).collect()),
                Value::List(["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"].iter().map(Value::string).collect()),
            ]))
        }));

        chess.insert("flip".to_string(), Value::builtin("chess@flip", |args, env| {
            check_args_len(Value::symbol("chess@flip"), &args, 1)?;

            if let Value::List(mut board) = args[0].eval(env)? {
                board.reverse();
                Ok(Value::List(board))
            } else {
                Err(Error::InvalidArguments(Value::symbol("chess@flip"), args.clone()))
            }
        }));

        fn get(board: Vec<Vec<Value>>, col: usize, row: usize) -> Value {
            board[row][col].clone()
        }

        fn to_coords(pos: &String) -> Result<(usize, usize), Error> {
            let tmp = pos.trim().to_lowercase();
            let mut chars = tmp.chars();
            if let Some(col) = chars.next() {
                if let Some(row) = chars.next() {
                    if chars.next() == None {
                        Ok((match col {
                            'a' => 0,
                            'b' => 1,
                            'c' => 2,
                            'd' => 3,
                            'e' => 4,
                            'f' => 5,
                            'g' => 6,
                            'h' => 7,
                            _ => return Err(Error::CustomError(format!("invalid notation for piece position `{}`", pos)))
                        }, match row.to_string().parse::<usize>() {
                            Ok(n) if 1 <= 1 && n <= 8 => 8 - n,
                            _ => return Err(Error::CustomError(format!("invalid notation for piece position `{}`", pos)))
                        }))
                    } else {
                        Err(Error::CustomError(format!("invalid notation for piece position `{}`", pos)))
                    }
                } else {
                    Err(Error::CustomError(format!("invalid notation for piece position `{}`", pos)))
                }
            } else {
                Err(Error::CustomError(format!("invalid notation for piece position `{}`", pos)))
            }
        }

        chess.insert("get".to_string(), Value::builtin("chess@get", |args, env| {
            check_args_len(Value::symbol("chess@get"), &args, 2)?;

            match (args[0].eval(env)?, args[1].eval(env)?) {
                (Value::List(val_board), Value::String(pos)) => {
                    let mut board = vec![];
                    for val_row in val_board {
                        if let Value::List(row) = val_row {
                            board.push(row);
                        } else {
                            return Err(Error::InvalidArguments(Value::Symbol("chess@get".to_string()), args.clone()))
                        }
                    }
                    let (col, row) = to_coords(&pos)?;
                    Ok(get(board, col, row))
                }
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@get".to_string()), args.clone()))
            }
        }));

        fn to_board(val_board: Vec<Value>) -> Result<Vec<Vec<Value>>, Error> {
            let mut board = vec![];
            for val_row in val_board {
                if let Value::List(row) = val_row {
                    board.push(row);
                } else {
                    return Err(Error::CustomError("malformed board".to_string()))
                }
            }

            Ok(board)
        }

        fn format_board(val_board: Vec<Value>) -> Result<Value, Error> {
            let mut result = String::new();
            let board = to_board(val_board)?;

            let mut is_black = false;
            for (i, row) in board.iter().enumerate() {
                for col in row {
                    result += &if is_black && is_space(&col.to_string()) {
                        "░".to_string()
                    } else if !is_black && is_space(&col.to_string()) {
                        " ".to_string()
                    } else {
                        format!("{}", col)
                    };

                    is_black = !is_black
                }
                is_black = !is_black;
                if i+1 < 8 { result.push('\n') }
            }

            Ok(Value::String(result))
        }

        chess.insert("fmt".to_string(), Value::builtin("chess@fmt", |args, env| {
            check_args_len(Value::symbol("chess@fmt"), &args, 1)?;

            match args[0].eval(env)? {
                Value::List(val_board) => format_board(val_board),
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@fmt".to_string()), args.clone()))
            }
        }));

        chess.insert("print".to_string(), Value::builtin("chess@print", |args, env| {
            check_args_len(Value::symbol("chess@print"), &args, 1)?;

            match args[0].eval(env)? {
                Value::List(val_board) => {
                    println!("{}", format_board(val_board)?);
                    Ok(Value::Nil)
                }
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@print".to_string()), args.clone()))
            }
        }));

        chess.insert("mv".to_string(), Value::builtin("chess@mv", |args, env| {
            check_args_len(Value::symbol("chess@mv"), &args, 3)?;

            match (args[0].eval(env)?, args[1].eval(env)?, args[2].eval(env)?) {
                (Value::List(val_board), Value::String(from), Value::String(to)) => {
                    let mut board = to_board(val_board)?;
                    let (src_col, src_row) = to_coords(&from)?;
                    let (dst_col, dst_row) = to_coords(&to)?;
                    
                    let src_piece = &board[src_row][src_col].to_string();
                    let dst_piece = &board[dst_row][dst_col].to_string();
                    
                    if is_piece(&src_piece) {
                        if (is_white(&src_piece) != is_white(&dst_piece)) || is_space(&dst_piece) {
                            board[dst_row][dst_col] = board[src_row][src_col].clone();
                            board[src_row][src_col] = Value::string(".");

                            Ok(Value::List(board.iter().map(|x| Value::List(x.clone())).collect()))
                        } else {
                            Err(Error::CustomError("cannot capture same color piece".to_string()))
                        }
                    } else {
                        Err(Error::CustomError("cannot move non-piece".to_string()))
                    }
                }
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@mv".to_string()), args.clone()))
            }
        }));

        chess.insert("add".to_string(), Value::builtin("chess@add", |args, env| {
            check_args_len(Value::symbol("chess@add"), &args, 3)?;

            match (args[0].eval(env)?, args[1].eval(env)?, args[2].eval(env)?) {
                (Value::List(val_board), Value::String(dst), Value::String(piece)) => {
                    let mut board = to_board(val_board)?;
                    let (dst_col, dst_row) = to_coords(&dst)?;
                    
                    board[dst_row][dst_col] = Value::String(piece);
                    Ok(Value::List(board.iter().map(|x| Value::List(x.clone())).collect()))
                }
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@add".to_string()), args.clone()))
            }
        }));

        chess.insert("rm".to_string(), Value::builtin("chess@rm", |args, env| {
            check_args_len(Value::symbol("chess@rm"), &args, 2)?;

            match (args[0].eval(env)?, args[1].eval(env)?) {
                (Value::List(val_board), Value::String(rm)) => {
                    let mut board = to_board(val_board)?;
                    let (rm_col, rm_row) = to_coords(&rm)?;
                    
                    board[rm_row][rm_col] = Value::string(".");
                    Ok(Value::List(board.iter().map(|x| Value::List(x.clone())).collect()))
                }
                _ => Err(Error::InvalidArguments(Value::Symbol("chess@rm".to_string()), args.clone()))
            }
        }));

        Value::Table(chess)
    };

    static ref FMT: Value = {
        let mut colorize = BTreeMap::new();
        let mut dark = BTreeMap::new();
        macro_rules! make_color {
            ($color:expr) => {|args, env| {
                let mut result = String::new();
                for (i, arg) in args.iter().enumerate() {
                    result += &format!("{}", arg.eval(env)?);
                    if i < args.len()-1 {
                        result += " ";
                    }
                }
                
                Ok(Value::String($color(result)))
            }};
        }

        dark.insert(String::from("red"), Value::builtin("fmt@dark@red", make_color!(Colorize::red)));
        dark.insert(String::from("green"), Value::builtin("fmt@dark@green", make_color!(Colorize::green)));
        dark.insert(String::from("blue"), Value::builtin("fmt@dark@blue", make_color!(Colorize::blue)));
        dark.insert(String::from("cyan"), Value::builtin("fmt@dark@cyan", make_color!(Colorize::cyan)));
        dark.insert(String::from("yellow"), Value::builtin("fmt@dark@yellow", make_color!(Colorize::yellow)));
        dark.insert(String::from("magenta"), Value::builtin("fmt@dark@magenta", make_color!(Colorize::magenta)));
        colorize.insert(String::from("dark"), Value::Table(dark));
        
        colorize.insert(String::from("red"), Value::builtin("fmt@red", make_color!(Colorize::bright_red)));
        colorize.insert(String::from("green"), Value::builtin("fmt@green", make_color!(Colorize::bright_green)));
        colorize.insert(String::from("blue"), Value::builtin("fmt@blue", make_color!(Colorize::bright_blue)));
        colorize.insert(String::from("yellow"), Value::builtin("fmt@yellow", make_color!(Colorize::bright_yellow)));
        colorize.insert(String::from("magenta"), Value::builtin("fmt@magenta", make_color!(Colorize::bright_magenta)));
        colorize.insert(String::from("cyan"), Value::builtin("fmt@cyan", make_color!(Colorize::bright_cyan)));
        colorize.insert(String::from("black"), Value::builtin("fmt@black", make_color!(Colorize::black)));
        colorize.insert(String::from("gray"), Value::builtin("fmt@gray", make_color!(Colorize::bright_black)));
        colorize.insert(String::from("grey"), Value::builtin("fmt@grey", make_color!(Colorize::bright_black)));
        colorize.insert(String::from("white"), Value::builtin("fmt@white", make_color!(Colorize::bright_white)));

        colorize.insert(String::from("bold"), Value::builtin("fmt@bold", make_color!(Colorize::bold)));
        colorize.insert(String::from("invert"), Value::builtin("fmt@invert", make_color!(Colorize::invert)));
        colorize.insert(String::from("underline"), Value::builtin("fmt@underline", make_color!(Colorize::underline)));
        
        Value::Table(colorize)
    };

    static ref MATH: Value = {
        let mut math = BTreeMap::new();
        math.insert("E".to_string(), Value::Float(std::f64::consts::E));
        math.insert("PI".to_string(), Value::Float(std::f64::consts::PI));
        math.insert("TAU".to_string(), Value::Float(std::f64::consts::TAU));

        math.insert("pow".to_string(), Value::builtin("pow", |args, env| {
            check_args_len(Value::symbol("math@pow"), &args, 2)?;

            match args[0].eval(env)? {
                Value::Float(base) => {
                    match args[1].eval(env)? {
                        Value::Float(n)   => Ok(Value::Float(base.powf(n))),
                        Value::Integer(n) => Ok(Value::Float(base.powi(n))),
                        _ => Err(Error::InvalidArguments(Value::symbol("math@pow"), args.clone()))
                    }
                }
                Value::Integer(base) => {
                    match args[1].eval(env)? {
                        Value::Float(n)   => Ok(Value::Float((base as f64).powf(n))),
                        Value::Integer(n) if n > 0 => Ok(Value::Integer(base.pow(n as u32))),
                        Value::Integer(n) => Ok(Value::Float((base as f64).powi(n))),
                        _ => Err(Error::InvalidArguments(Value::symbol("math@pow"), args.clone()))
                    }
                }
                _ => Err(Error::InvalidArguments(Value::symbol("math@pow"), args.clone()))
            }
        }));

        math.insert("log".to_string(), Value::builtin("log", |args, env| {
            check_args_len(Value::symbol("math@log"), &args, 2)?;

            let base = match args[0].eval(env)? {
                Value::Float(n)   => Ok(n),
                Value::Integer(n) => Ok(n as f64),
                _ => Err(Error::InvalidArguments(Value::symbol("math@log"), args.clone()))
            }?;

            let x = match args[1].eval(env)? {
                Value::Float(n)   => Ok(n),
                Value::Integer(n) => Ok(n as f64),
                _ => Err(Error::InvalidArguments(Value::symbol("math@log"), args.clone()))
            }?;
            
            Ok(Value::Float(x.log(base)))
        }));

        math.insert("log10".to_string(), Value::builtin("log10", |args, env| {
            check_args_len(Value::symbol("math@log10"), &args, 1)?;

            let x = match args[0].eval(env)? {
                Value::Float(n)   => Ok(n),
                Value::Integer(n) => Ok(n as f64),
                _ => Err(Error::InvalidArguments(Value::symbol("math@log10"), args.clone()))
            }?;
            
            Ok(Value::Float(x.log10()))
        }));

        math.insert("log2".to_string(), Value::builtin("log2", |args, env| {
            check_args_len(Value::symbol("math@log2"), &args, 1)?;

            let x = match args[0].eval(env)? {
                Value::Float(n)   => Ok(n),
                Value::Integer(n) => Ok(n as f64),
                _ => Err(Error::InvalidArguments(Value::symbol("math@log2"), args.clone()))
            }?;
            
            Ok(Value::Float(x.log2()))
        }));

        math.insert("sqrt".to_string(), Value::builtin("sqrt", |args, env| {
            check_args_len(Value::symbol("math@sqrt"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.sqrt())),
                Value::Integer(n) => Ok(Value::Float((n as f64).sqrt())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@sqrt"), args.clone()))
            }
        }));


        math.insert("cbrt".to_string(), Value::builtin("cbrt", |args, env| {
            check_args_len(Value::symbol("math@cbrt"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.cbrt())),
                Value::Integer(n) => Ok(Value::Float((n as f64).cbrt())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@cbrt"), args.clone()))
            }
        }));

        math.insert("sin".to_string(), Value::builtin("sin", |args, env| {
            check_args_len(Value::symbol("math@sin"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.sin())),
                Value::Integer(n) => Ok(Value::Float((n as f64).sin())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@sin"), args.clone()))
            }
        }));

        math.insert("cos".to_string(), Value::builtin("cos", |args, env| {
            check_args_len(Value::symbol("math@cos"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.cos())),
                Value::Integer(n) => Ok(Value::Float((n as f64).cos())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@cos"), args.clone()))
            }
        }));

        math.insert("tan".to_string(), Value::builtin("tan", |args, env| {
            check_args_len(Value::symbol("math@tan"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.tan())),
                Value::Integer(n) => Ok(Value::Float((n as f64).tan())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@tan"), args.clone()))
            }
        }));

        math.insert("asin".to_string(), Value::builtin("asin", |args, env| {
            check_args_len(Value::symbol("math@asin"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.asin())),
                Value::Integer(n) => Ok(Value::Float((n as f64).asin())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@asin"), args.clone()))
            }
        }));

        math.insert("acos".to_string(), Value::builtin("acos", |args, env| {
            check_args_len(Value::symbol("math@acos"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.acos())),
                Value::Integer(n) => Ok(Value::Float((n as f64).acos())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@acos"), args.clone()))
            }
        }));

        math.insert("atan".to_string(), Value::builtin("atan", |args, env| {
            check_args_len(Value::symbol("math@atan"), &args, 1)?;

            match args[0].eval(env)? {
                Value::Float(n)   => Ok(Value::Float(n.atan())),
                Value::Integer(n) => Ok(Value::Float((n as f64).atan())),
                _ => Err(Error::InvalidArguments(Value::symbol("math@atan"), args.clone()))
            }
        }));

        Value::Table(math)
    };
}

#[derive(Clone)]
pub struct Environment {
    symbols: BTreeMap<String, Value>
}

const TYPES: &'static [Type] = &[
    Type::Alpine,
    Type::Amazon,
    Type::Android,
    Type::Arch,
    Type::CentOS,
    Type::Debian,
    Type::Emscripten,
    Type::EndeavourOS,
    Type::Fedora,
    Type::Linux,
    Type::Macos,
    Type::Manjaro,
    Type::Mint,
    Type::openSUSE,
    Type::OracleLinux,
    Type::Pop,
    Type::Redhat,
    Type::RedHatEnterprise,
    Type::Redox,
    Type::Solus,
    Type::SUSE,
    Type::Ubuntu,
    Type::Unknown,
    Type::Windows
];

fn get_os_name(t: &Type) -> String {
    match t {
        Type::Alpine           => "alpine",
        Type::Amazon           => "amazon",
        Type::Android          => "android",
        Type::Arch             => "arch",
        Type::CentOS           => "centos",
        Type::Debian           => "debian",
        Type::Macos            => "macos",
        Type::Fedora           => "fedora",
        Type::Linux            => "linux",
        Type::Manjaro          => "manjaro",
        Type::Mint             => "mint",
        Type::openSUSE         => "opensuse",
        Type::EndeavourOS      => "endeavouros",
        Type::OracleLinux      => "oraclelinux",
        Type::Pop              => "pop",
        Type::Redhat           => "redhat",
        Type::RedHatEnterprise => "redhatenterprise",
        Type::Redox            => "redox",
        Type::Solus            => "solus",
        Type::SUSE             => "suse",
        Type::Ubuntu           => "ubuntu",
        Type::Windows          => "windows",
        Type::Unknown | _ => "unknown",
    }.to_string()
}

fn get_os_family(t: &Type) -> String {
    match t {
        Type::Amazon
        | Type::Android => "android",
        Type::Alpine
        | Type::Arch
        | Type::CentOS
        | Type::Debian
        | Type::Fedora
        | Type::Linux
        | Type::Manjaro
        | Type::Mint
        | Type::openSUSE
        | Type::EndeavourOS
        | Type::OracleLinux
        | Type::Pop
        | Type::Redhat
        | Type::RedHatEnterprise
        | Type::SUSE
        | Type::Ubuntu
         => "linux",
        
        Type::Macos
        | Type::Solus
        | Type::Redox => "unix",

        Type::Windows => "windows",

        Type::Unknown | _ => "unknown",
    }.to_string()
}

pub const REPORT: &str = "report";
pub const PROMPT: &str = "prompt";
pub const INCOMPLETE_PROMPT: &str = "incomplete-prompt";

pub const CWD:   &str = "CWD";
const HOME:      &str = "HOME";
const VIDEOS:    &str = "VIDS";
const DESKTOP:   &str = "DESK";
const PICTURES:  &str = "PICS";
const DOCUMENTS: &str = "DOCS";
const DOWNLOADS: &str = "DOWN";


fn check_args_len(func: Value, args: &Vec<Value>, len: usize) -> Result<(), Error> {
    if args.len() > len { Err(Error::TooManyArguments(func, args.clone())) }
    else if args.len() < len { Err(Error::TooFewArguments(func, args.clone())) }
    else { Ok(()) }
}

impl Environment {
    pub fn new() -> Self {
        let mut result = Self { symbols: BTreeMap::new() };
        result.define(CWD, Value::Path(if let Ok(path) = result.get_cwd() {
            path
        } else {
            PathBuf::new()
        }));
        result
    }

    pub fn get(&self, name: impl ToString) -> Result<Value, Error> {
        let name = name.to_string();
        if let Some(value) = self.symbols.get(&name) {
            Ok(value.clone())
        } else {
            Ok(match name.as_str() {
                REPORT => Value::builtin(REPORT, |args, env| {
                    check_args_len(env.get(REPORT)?, &args, 1)?;

                    let val = args[0].eval(env)?;

                    match val {
                        Value::Nil | Value::Integer(0) => {}
                        Value::Error(e) => println!("error: {}", e),
                        other => println!(" => {:?}", other),
                    }

                    Ok(Value::Nil)
                }),

                PROMPT => Value::builtin(PROMPT, |args, env| {
                    check_args_len(env.get(PROMPT)?, &args, 1)?;
                    Ok(Value::String(format!("{}> ", args[0].eval(env)?)))
                }),

                INCOMPLETE_PROMPT => Value::builtin(INCOMPLETE_PROMPT, |args, env| {
                    check_args_len(env.get(INCOMPLETE_PROMPT)?, &args, 1)?;
                    Ok(Value::String(format!("{}> ", " ".repeat(format!("{}", args[0].eval(env)?).len()))))
                }),

                "absolute" => Value::builtin("absolute", |args, env| {
                    check_args_len(env.get("absolute")?, &args, 1)?;

                    match args[0].eval(env)? {
                        Value::Path(path) => if let Ok(result) = dunce::canonicalize(path) {
                            Ok(Value::Path(result))
                        } else {
                            Err(Error::CustomError(String::from("could not canonicalize path")))
                        },

                        Value::Symbol(path) | Value::String(path) => if let Ok(result) = dunce::canonicalize(path) {
                            Ok(Value::Path(result))
                        } else {
                            Err(Error::CustomError(String::from("could not canonicalize path")))
                        },

                        _ => Err(Error::InvalidArguments(env.get("absolute")?, args.clone()))
                    }
                }),

                "exists" => Value::builtin("exists", |args, env| {
                    check_args_len(env.get("exists")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::Path(path) => Ok(Value::Boolean(path.exists())),
                        Value::String(path) | Value::Symbol(path) => Ok(Value::Boolean(PathBuf::from(path).exists())),
                        _ => Err(Error::InvalidArguments(env.get("exists")?, args.clone()))
                    }
                }),

                "cards" => CARDS.clone(),
                "chess" => CHESS.clone(),

                "rand" => {
                    let mut random = BTreeMap::new();
                    random.insert("int".to_string(), Value::builtin("rand@int", |args, env| {
                        check_args_len(Value::Symbol("rand@int".to_string()), &args, 2)?;
                        if let (Value::Integer(l), Value::Integer(h)) = (args[0].eval(env)?, args[1].eval(env)?) {
                            let mut rng = thread_rng();
                            let n = Uniform::new(l, h);
                            Ok(Value::Integer(rng.sample(n)))
                        } else {
                            Err(Error::InvalidArguments(Value::Symbol("rand@int".to_string()), args.clone()))
                        }
                    }));
                    
                    random.insert("shuffle".to_string(), Value::builtin("rand@shuffle", |args, env| {
                        check_args_len(Value::Symbol("rand@shuffle".to_string()), &args, 1)?;

                        if let Value::List(mut list) = args[0].eval(env)? {
                            let mut rng = thread_rng();
                            list.shuffle(&mut rng);
                            Ok(Value::List(list))
                        } else {
                            Err(Error::InvalidArguments(Value::Symbol("rand@shuffle".to_string()), args.clone()))
                        }
                    }));
                    
                    random.insert("choose".to_string(), Value::builtin("rand@choose", |args, env| {
                        check_args_len(Value::Symbol("rand@choose".to_string()), &args, 1)?;

                        if let Value::List(list) = args[0].eval(env)? {
                            let mut rng = thread_rng();
                            let n = Uniform::new(0, list.len());
                            Ok(list[rng.sample(n)].clone())
                        } else {
                            Err(Error::InvalidArguments(Value::Symbol("rand@choose".to_string()), args.clone()))
                        }
                    }));

                    Value::Table(random)
                },

                "file" => {
                    let mut file = BTreeMap::new();
                    file.insert("read".to_string(), Value::builtin("file@read", |args, env| {
                        check_args_len(Value::Symbol("file@read".to_string()), &args, 1)?;
    
                        match args[0].eval(env)? {
                            Value::Path(path) => {
                                if let Ok(contents) = read_to_string(&env.get_cwd()?.join(&path)) {
                                    Ok(Value::String(contents))
                                } else {
                                    Err(Error::CustomError(format!("could not read file {:?}", path)))
                                }
                            },
    
                            Value::String(path) | Value::Symbol(path) => {
                                if let Ok(contents) = read_to_string(&env.get_cwd()?.join(&path)) {
                                    Ok(Value::String(contents))
                                } else {
                                    Err(Error::CustomError(format!("could not read file {:?}", path)))
                                }
                            },
                            _ => Err(Error::InvalidArguments(Value::Symbol("file@read".to_string()), args.clone()))
                        }
                    }));

                    file.insert("write".to_string(), Value::builtin("file@write", |args, env| {
                        check_args_len(Value::Symbol("file@write".to_string()), &args, 2)?;
    
                        if let Value::String(contents) = args[1].eval(env)? {
                            match args[0].eval(env)? {
                                Value::Path(path) => match write(&env.get_cwd()?.join(&path), contents) {
                                    Ok(_) => Ok(Value::Nil),
                                    _ => Err(Error::CustomError(format!("could not write to file {:?}", path)))
                                },
                                Value::String(path) | Value::Symbol(path) => match write(&env.get_cwd()?.join(&path), contents) {
                                    Ok(_) => Ok(Value::Nil),
                                    _ => Err(Error::CustomError(format!("could not write to file {:?}", path)))
                                },

                                _ => Err(Error::InvalidArguments(Value::Symbol("file@write".to_string()), args.clone()))
                            }
                        } else {
                            Err(Error::InvalidArguments(Value::Symbol("file@write".to_string()), args.clone()))
                        }
                    }));

                    file.insert("append".to_string(), Value::builtin("file@append", |args, env| {
                        check_args_len(Value::Symbol("file@append".to_string()), &args, 2)?;
    
                        let contents = match args[0].eval(env)? {
                            Value::Path(path) => {
                                if let Ok(contents) = read_to_string(&env.get_cwd()?.join(&path)) {
                                    contents
                                } else {
                                    String::new()
                                }
                            },
    
                            Value::String(path) | Value::Symbol(path) => {
                                if let Ok(contents) = read_to_string(&env.get_cwd()?.join(path)) {
                                    contents
                                } else {
                                    String::new()
                                }
                            },
                            _ => return Err(Error::InvalidArguments(Value::Symbol("file@append".to_string()), args.clone()))
                        };

                        if let Value::String(new_contents) = args[1].eval(env)? {
                            match args[0].eval(env)? {
                                Value::Path(path) => match write(&env.get_cwd()?.join(&path), contents.to_string() + &new_contents) {
                                    Ok(_) => Ok(Value::Nil),
                                    _ => Err(Error::CustomError(format!("could not write to file {:?}", path)))
                                },
                                Value::String(path) | Value::Symbol(path) => match write(&env.get_cwd()?.join(&path), contents.to_string() + &new_contents) {
                                    Ok(_) => Ok(Value::Nil),
                                    _ => Err(Error::CustomError(format!("could not write to file {:?}", path)))
                                },

                                _ => Err(Error::InvalidArguments(Value::Symbol("file@append".to_string()), args.clone()))
                            }
                        } else {
                            Err(Error::InvalidArguments(Value::Symbol("file@append".to_string()), args.clone()))
                        }
                    }));

                    Value::Table(file)
                }

                "is-err" => Value::builtin("is-err", |args, env| {
                    check_args_len(env.get("is-err")?, &args, 1)?;

                    Ok(Value::Boolean(match args[0].eval(env) {
                        Ok(Value::Error(_)) | Err(_) => true,
                        _ => false
                    }))
                }),

                "is-syntax-err" => Value::builtin("is-syntax-err", |args, env| {
                    check_args_len(env.get("is-syntax-err")?, &args, 1)?;

                    Ok(Value::Boolean(match args[0].eval(env) {
                        Ok(Value::Error(e)) => match *e {
                            Error::SyntaxError(_) => true,
                            _ => false,
                        },
                        _ => false
                    }))
                }),


                "widget" => {
                    let mut widget = BTreeMap::new();

                    widget.insert(String::from("create"), Value::builtin("widget@create", |args, env| {
                        check_args_len(Value::Symbol("widget@create".to_string()), &args, 4)?;
                        // print(widget@create("♔", "testing", 7))
                        // print(widget@create("Chess", "testing!", 8))
                        // print(widget@add-vertical(widget@create("Chess", "testing!", 8), widget@create("♔", "hmm", 8)))
                        let title = match args[0].eval(env)? {
                            Value::String(x) => x,
                            _ => return Err(Error::InvalidArguments(Value::Symbol("widget@create".to_string()), args.clone()))
                        };
                        
                        let text = match args[1].eval(env)? {
                            Value::String(x) => x,
                            _ => return Err(Error::InvalidArguments(Value::Symbol("widget@create".to_string()), args.clone())),
                        };
                        
                        let text_width = match args[2].eval(env)? {
                            Value::Integer(x) if x > 4 => x as usize,
                            _ => return Err(Error::InvalidArguments(Value::Symbol("widget@create".to_string()), args.clone())),
                        } - 2;
                        
                        let widget_height = match args[3].eval(env)? {
                            Value::Integer(x) if x >= 3 => x as usize,
                            _ => return Err(Error::InvalidArguments(Value::Symbol("widget@create".to_string()), args.clone())),
                        };

                        if text_width < title.len() {
                            Err(Error::CustomError(String::from("width is less than title length")))
                        } else {
                            let title_len = title.chars().collect::<Vec<char>>().len();
                            let mut left_border_half = "─".repeat(((text_width - title_len) as f64 / 2.0).round() as usize);
                            let right_border_half = left_border_half.clone();
                            let left_len = left_border_half.chars().collect::<Vec<char>>().len();
                            if (left_len * 2 + title_len + 2) > text_width + 2 {
                                left_border_half.pop();
                            }

                            let mut result = format!("┌{left_side}{}{right_side}┐\n", title, left_side=left_border_half, right_side=right_border_half);
                            let width = result.chars().collect::<Vec<char>>().len() - 1;
                            
                            let mut i = 0;
                            for ch in text.chars() {
                                if i == 0 {
                                    result.push(' ');
                                    i += 1;
                                }

                                if ch == '\n' {
                                    result += &" ".repeat(width-i);
                                    i = width;
                                } else {
                                    result.push(ch);
                                }
                                
                                if i >= width-1 {
                                    result += "\n";
                                    i = 0;
                                } else {
                                    i += 1;
                                }
                            }


                            result += &" ".repeat(width-i);

                            while result.lines().collect::<Vec<&str>>().len() < widget_height - 1 {
                                result += &(String::from("\n") + &" ".repeat(width));
                            }

                            result += &format!("\n└{left_side}{}{right_side}┘", "─".repeat(title_len), left_side=left_border_half, right_side=right_border_half);

                            Ok(Value::String(result))
                        }
                    }));

                    widget.insert(String::from("add-horizontal"), Value::builtin("widget@add-horizontal", |args, env| {
                        if args.is_empty() {
                            Err(Error::TooFewArguments(Value::Symbol("widget@add-horizontal".to_string()), args.clone()))
                        } else {
                            let mut string_args = vec![];
                            let mut height = 0;
                            for (i, arg) in args.iter().enumerate() {
                                if let Value::String(s) = arg.eval(env)? {
                                    let lines = s.lines().map(ToString::to_string).collect::<Vec<String>>();
                                    string_args.push(lines.clone());

                                    height = string_args[0].len();
                                    
                                    if height != lines.len() {
                                        return Err(Error::CustomError(format!("Heights of horizontally added widgets must be equal, 0={}, {}={}", height, i, lines.len())))
                                    }
                                } else {
                                    return Err(Error::InvalidArguments(Value::Symbol("widget@add-horizontal".to_string()), args.clone()));
                                }
                            }

                            let mut result = String::new();

                            for line_n in 0..height {
                                for arg in &string_args {
                                    result += &arg[line_n];
                                }
                                result += "\n";
                            }


                            Ok(Value::String(result))
                        }
                    }));

                    widget.insert(String::from("add-vertical"), Value::builtin("widget@add-vertical", |args, env| {
                        if args.is_empty() {
                            Err(Error::TooFewArguments(Value::Symbol("widget@add-vertical".to_string()), args.clone()))
                        } else {
                            let mut string_args = vec![];
                            for (i, arg) in args.iter().enumerate() {
                                if let Value::String(s) = arg.eval(env)? {
                                    string_args.push(s.trim().to_string());

                                    let width = string_args[0].lines().next().unwrap().chars().collect::<Vec<char>>().len();
                                    

                                    let this_width = string_args[i].lines().next().unwrap().chars().collect::<Vec<char>>().len();
                                    if width != this_width {
                                        return Err(Error::CustomError(format!("Widths of vertically added widgets must be equal, 0={}, {}={}", width, i, this_width)))
                                    }
                                } else {
                                    return Err(Error::InvalidArguments(Value::Symbol("widget@add-vertical".to_string()), args.clone()));
                                }
                            }

                            Ok(Value::String(string_args.join("\n")))
                        }
                    }));

                    Value::Table(widget)
                },

                "fmt" => FMT.clone(),
                "math" => MATH.clone(),

                "sleep" => Value::builtin("sleep", |args, env| {
                    check_args_len(env.get("sleep")?, &args, 1)?;

                    match args[0].eval(env)? {
                        Value::Float(n)   => sleep(Duration::from_millis((n.abs() * 1000.0) as u64)),
                        Value::Integer(n) => sleep(Duration::from_millis((n.abs() * 1000) as u64)),
                        _ => return Err(Error::InvalidArguments(env.get("sleep")?, args.clone()))
                    }
                    
                    Ok(Value::Nil)
                }),

                "to-path" => Value::builtin("to-path", |args, env| {
                    check_args_len(env.get("to-path")?, &args, 1)?;
                    
                    match args[0].eval(env)? {
                        Value::Path(x) => Ok(Value::Path(x)),
                        Value::String(s) | Value::Symbol(s) => Ok(Value::Path(PathBuf::from(s))),
                        _ => Err(Error::InvalidArguments(env.get("to-path")?, args.clone()))
                    }
                }),

                "to-str" => Value::builtin("to-str", |args, env| {
                    
                    let mut result = String::new();
                    for (i, arg) in args.iter().enumerate() {
                        result += &format!("{}", arg.eval(env)?);
                        if i < args.len()-1 {
                            result += " ";
                        }
                    }
                    
                    Ok(Value::String(result))
                }),

                "to-float" => Value::builtin("to-float", |args, env| {
                    check_args_len(env.get("to-float")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::String(s) => match s.parse::<f64>() {
                            Ok(n) => Ok(Value::Float(n)),
                            Err(_) => Err(Error::CouldNotParseInteger(Value::String(s))),
                        },
                        Value::Float(x) => Ok(Value::Float(x)),
                        Value::Integer(x) => Ok(Value::Float(x as f64)),
                        Value::Boolean(x) => Ok(Value::Float(if x { 1.0 } else { 0.0 })),
                        _ => Err(Error::InvalidArguments(env.get("to-float")?, args.clone()))
                    }
                }),

                "to-int" => Value::builtin("to-int", |args, env| {
                    check_args_len(env.get("to-int")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::String(s) => match s.parse::<i32>() {
                            Ok(i) => Ok(Value::Integer(i)),
                            Err(_) => Err(Error::CouldNotParseInteger(Value::String(s))),
                        },
                        Value::Float(x) => Ok(Value::Integer(x as i32)),
                        Value::Integer(x) => Ok(Value::Integer(x)),
                        Value::Boolean(x) => Ok(Value::Integer(if x { 1 } else { 0 })),
                        _ => Err(Error::InvalidArguments(env.get("to-int")?, args.clone()))
                    }
                }),

                "input" => Value::builtin("input", |args, env| {
                    let mut result = String::new();

                    for (i, arg) in args.iter().enumerate() {
                        print!("{}", arg.eval(env)?);
                        if i < args.len()-1 {
                            print!(" ");
                        }
                    }
                    let _ = stdout().flush();
                    if stdin().read_line(&mut result).is_err() {
                        Err(Error::ReadInputError)
                    } else {
                        Ok(Value::String(result.trim_end_matches("\n").trim_end_matches("\r").to_string()))
                    }
                }),

                "rev" => Value::builtin("rev", |args, env| {
                    check_args_len(env.get("rev")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::String(s) => Ok(Value::String(s.chars().rev().collect())),
                        Value::List(l) => Ok(Value::List(l.into_iter().rev().collect())),
                        _ => Err(Error::InvalidArguments(env.get("rev")?, args.clone()))
                    }
                }),

                "split" => Value::builtin("split", |args, env| {
                    check_args_len(env.get("split")?, &args, 2)?;

                    if let Value::String(s) = args[0].eval(env)? {
                        Ok(Value::List(
                            s.split(&args[1].eval(env)?.to_string())
                                .map(|x| Value::String(x.to_string()))
                                .collect()
                        ))
                    } else {
                        Err(Error::InvalidArguments(env.get("join")?, args.clone()))
                    }
                }),

                "sort" => Value::builtin("sort", |args, env| {
                    check_args_len(env.get("sort")?, &args, 1)?;

                    if let Value::List(list) = args[0].eval(env)? {
                        let mut num_list = vec![];
                        for item in list {
                            match item {
                                Value::Integer(i) => num_list.push(i),
                                _ => return Err(Error::InvalidArguments(env.get("sort")?, args.clone()))
                            }
                        }
                        num_list.sort();
                        Ok(Value::List(num_list.iter().map(|x| Value::Integer(*x)).collect()))
                    } else {
                        Err(Error::InvalidArguments(env.get("sort")?, args.clone()))
                    }
                }),

                "join" => Value::builtin("join", |args, env| {
                    check_args_len(env.get("join")?, &args, 2)?;

                    if let Value::List(list) = args[0].eval(env)? {
                        Ok(Value::String(
                            list
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(&args[1].eval(env)?.to_string())
                        ))
                    } else {
                        Err(Error::InvalidArguments(env.get("join")?, args.clone()))
                    }
                }),

                "date" => {
                    let now = Local::now().date();

                    let mut date = BTreeMap::new();
                    date.insert(String::from("day"),     Value::Integer(now.day() as i32));
                    date.insert(String::from("weekday"), Value::Integer(now.weekday().num_days_from_sunday() as i32));
                    date.insert(String::from("month"),   Value::Integer(now.month() as i32));
                    date.insert(String::from("year"),    Value::Integer(now.year() as i32));
                    date.insert(String::from("str"),     Value::String(now.format("%D").to_string()));
                    Value::Table(date)
                }

                "time" => {
                    let now = Local::now();

                    let mut time = BTreeMap::new();
                    time.insert(String::from("hour"),   Value::Integer(now.hour() as i32));
                    time.insert(String::from("minute"), Value::Integer(now.minute() as i32));
                    time.insert(String::from("second"), Value::Integer(now.second() as i32));
                    time.insert(String::from("str"),    Value::String(now.time().format("%-I:%M %p").to_string()));
                    Value::Table(time)
                }

                "sh" => {
                    let mut shell = BTreeMap::new();
                    if let Ok(path) = current_exe() {
                        shell.insert(String::from("exe"), Value::Path(path.clone()));
                        if let Some(parent) = path.parent() {
                            shell.insert(String::from("dir"), Value::Path(PathBuf::from(parent)));
                        }
                    }

                    shell.insert(String::from("version"), Value::List(VERSION.iter().map(|x| Value::Integer(*x as i32)).collect()));
                    if let Ok(path) = self.get_home_dir() {
                        shell.insert(String::from("prelude"), Value::Path(path.join(PRELUDE_FILENAME)));
                    }
                    Value::Table(shell)
                }

                "os" => {
                    let os = os_info::get();
                    let mut os_table = BTreeMap::new();
                    os_table.insert(String::from("name"),   Value::String(get_os_name(&os.os_type())));
                    os_table.insert(String::from("family"), Value::String(get_os_family(&os.os_type())));
                    os_table.insert(String::from("version"), Value::String(format!("{}", os.version())));
                    Value::Table(os_table)
                }

                "env" => Value::Table(self.get_symbols().clone()),

                HOME => Value::Path(self.get_home_dir()?),
                VIDEOS => Value::Path(self.get_vids_dir()?),
                DESKTOP => Value::Path(self.get_desk_dir()?),
                PICTURES => Value::Path(self.get_pics_dir()?),
                DOCUMENTS => Value::Path(self.get_docs_dir()?),
                DOWNLOADS => Value::Path(self.get_down_dir()?),
                
                "home" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(HOME)?)))),
                "vids" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(VIDEOS)?)))),
                "desk" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(DESKTOP)?)))),
                "pics" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(PICTURES)?)))),
                "docs" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(DOCUMENTS)?)))),
                "down" => Value::Macro(vec![], Box::new(Value::Define(CWD.to_string(), Box::new(self.get(DOWNLOADS)?)))),

                "exit" | "quit" => Value::builtin("exit", |_, _| exit(0)),

                "unbind" => Value::builtin("unbind", |args, env| {
                    check_args_len(env.get("unbind")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::String(name) => {
                            if env.is_defined(&name) {
                                let result = env.get(&name)?;
                                env.symbols.remove(&name);
                                Ok(result)
                            } else {
                                Err(Error::SymbolNotDefined(name))
                            }
                        }
                        _ => Err(Error::InvalidArguments(env.get("unbind")?, args.clone()))
                    }
                }),

                "print" => Value::builtin("print", |args, env| {
                    let mut acc = Value::Nil;
                    for (i, arg) in args.iter().enumerate() {
                        acc = arg.eval(env)?;
                        print!("{}", acc);
                        if i < args.len()-1 {
                            print!(" ");
                        } else {
                            println!("");
                        }
                    }
        
                    Ok(acc)
                }),

                "echo" => Value::builtin("print", |args, env| {
                    for (i, arg) in args.iter().enumerate() {
                        print!("{}", arg.eval(env)?);
                        if i < args.len()-1 {
                            print!(" ");
                        } else {
                            println!("");
                        }
                    }
        
                    Ok(Value::Nil)
                }),
                
                "pwd" | "cwd" => Value::builtin("pwd", |args, env| {
                    check_args_len(env.get("pwd")?, &args, 0)?;
                    println!("{}", env.get("CWD")?);
                    Ok(Value::Nil)
                }),

                
                "cd-eval" => Value::builtin("cd-eval", |args, env| {
                    if args.is_empty() {
                        env.define(CWD, Value::Path(env.get_home_dir()?));
                        Ok(Value::Integer(0))
                    } else {
                        check_args_len(env.get("cd-eval")?, &args, 1)?;
                        let mut cwd = env.get_cwd()?;
                            
                        cwd.push(PathBuf::from(args[0].eval(env)?.to_string()));
                        if cwd.exists() && cwd.is_dir() {
                            env.define(CWD, Value::Path(match dunce::canonicalize(&cwd) {
                                Ok(path) => path,
                                Err(_) => cwd.clone()
                            }));

                            Ok(Value::Integer(0))
                        } else {
                            Err(Error::CannotChangeDir(cwd))
                        }
                    }
                }),

                "cd" => Value::builtin("cd", |args, env| {
                    if args.is_empty() {
                        env.define(CWD, Value::Path(env.get_home_dir()?));
                        Ok(Value::Integer(0))
                    } else {
                        check_args_len(env.get("cd")?, &args, 1)?;
                        let mut cwd = env.get_cwd()?;
                        
                        cwd.push(PathBuf::from(args[0].to_string()));
                        
                        if cwd.exists() && cwd.is_dir() {
                            env.define(CWD, Value::Path(match dunce::canonicalize(&cwd) {
                                Ok(path) => path,
                                Err(_) => cwd.clone()
                            }));
                            
                            Ok(Value::Integer(0))
                        } else {
                            Err(Error::CannotChangeDir(cwd))
                        }
                    }
                }),

                "clear" | "cls" => Value::builtin("cls", move |args, env| {
                    check_args_len(env.get("cls")?, &args, 0)?;

                    let family = get_os_family(&os_info::get().os_type());
                    if family == "linux" || family == "unix" {
                        Value::Run(Box::new(Value::Path(PathBuf::from("clear"))), vec![]).eval(env)
                    } else if family == "windows" {
                        Value::Run(Box::new(Value::Path(PathBuf::from("cls"))), vec![]).eval(env)
                    } else {
                        println!("{}", "\n".repeat(255));
                        Ok(Value::Nil)
                    }
                }),

                "keys" => Value::builtin("keys", |args, env| {
                    check_args_len(env.get("keys")?, &args, 1)?;

                    if let Value::Table(table) = args[0].eval(env)? {
                        Ok(Value::List(table.keys().map(|x| Value::String(x.clone())).collect()))
                    } else {
                        Err(Error::InvalidArguments(env.get("keys")?, args.clone()))
                    }
                }),

                "vals" => Value::builtin("vals", |args, env| {
                    check_args_len(env.get("vals")?, &args, 1)?;

                    if let Value::Table(table) = args[0].eval(env)? {
                        Ok(Value::List(table.values().map(|x| x.clone()).collect()))
                    } else {
                        Err(Error::InvalidArguments(env.get("vals")?, args.clone()))
                    }
                }),
                
                "insert" => Value::builtin("insert", |args, env| {
                    check_args_len(env.get("insert")?, &args, 3)?;

                    if let Value::Table(mut t) = args[0].eval(env)? {
                        if let Value::String(key) = args[1].eval(env)? {
                            t.insert(key, args[2].eval(env)?);
                            Ok(Value::Table(t))
                        } else {
                            Err(Error::InvalidArguments(env.get("insert")?, args.clone()))
                        }
                    } else {
                        Err(Error::InvalidArguments(env.get("insert")?, args.clone()))
                    }
                }),

                "remove" => Value::builtin("remove", |args, env| {
                    check_args_len(env.get("remove")?, &args, 2)?;

                    if let Value::Table(mut t) = args[0].eval(env)? {
                        if let Value::String(key) = args[1].eval(env)? {
                            t.remove(&key);
                            Ok(Value::Table(t))
                        } else {
                            Err(Error::InvalidArguments(env.get("remove")?, args.clone()))
                        }
                    } else {
                        Err(Error::InvalidArguments(env.get("remove")?, args.clone()))
                    }
                }),

                "len" => Value::builtin("len", |args, env| {
                    check_args_len(env.get("len")?, &args, 1)?;

                    match args[0].eval(env)? {
                        Value::List(list) => Ok(Value::Integer(list.len() as i32)),
                        Value::Table(t) => Ok(Value::Integer(t.len() as i32)),
                        Value::String(s) => Ok(Value::Integer(s.chars().collect::<Vec<char>>().len() as i32)),
                        Value::Path(path) => Ok(Value::Integer(path.components().collect::<Vec<Component>>().len() as i32)),
                        _ => Err(Error::InvalidArguments(env.get("len")?, args.clone()))
                    }
                }),

                "push" => Value::builtin("push", |args, env| {
                    check_args_len(env.get("push")?, &args, 2)?;
                    if let Value::List(mut list) = args[0].eval(env)? {
                        for arg in &args[1..] {
                            list.push(arg.eval(env)?);
                        }
                        
                        Ok(Value::List(list))
                    } else {
                        Err(Error::InvalidArguments(env.get("push")?, args.clone()))
                    }
                }),
                
                "pop" => Value::builtin("pop", |args, env| {
                    check_args_len(env.get("pop")?, &args, 1)?;
                    match args[0].eval(env)? {
                        Value::List(mut list) => {
                            Ok(match list.pop() {
                                Some(val) => val,
                                None => Value::Nil
                            })
                        }

                        Value::String(mut s) => {
                            Ok(if let Some(ch) = s.pop() {
                                Value::String(ch.to_string())
                            } else {
                                Value::Nil
                            })
                        }

                        Value::Path(path) => {
                            Ok(Value::Path(if let Some(parent) = path.parent() {
                                PathBuf::from(parent)
                            } else {
                                path
                            }))
                        }

                        _ => Err(Error::InvalidArguments(env.get("pop")?, args.clone()))
                    }
                }),
                
                "zip" => Value::builtin("zip", |args, env| {
                    check_args_len(env.get("zip")?, &args, 2)?;
                    match (args[0].eval(env)?, args[1].eval(env)?) {
                        (Value::List(a), Value::List(b)) => Ok(Value::List(a.into_iter().zip(b.into_iter()).map(|(a, b)| Value::List(vec![a, b])).collect())),
                        _ => Err(Error::InvalidArguments(env.get("zip")?, args.clone()))
                    }
                }),

                "head" => Value::builtin("head", |args, env| {
                    check_args_len(env.get("head")?, &args, 1)?;
                    if let Value::List(list) = args[0].eval(env)? {
                        if list.is_empty() {
                            Err(Error::IndexNotFound(Value::List(list), Value::Integer(0)))
                        } else {
                            Ok(list[0].clone())
                        }
                    } else {
                        Err(Error::InvalidArguments(env.get("head")?, args.clone()))
                    }
                }),

                "tail" => Value::builtin("tail", |args, env| {
                    check_args_len(env.get("tail")?, &args, 1)?;
                    if let Value::List(list) = args[0].eval(env)? {
                        if list.is_empty() {
                            Ok(Value::List(vec![]))
                        } else {
                            Ok(Value::List(list[1..].to_vec()))
                        }
                    } else {
                        Err(Error::InvalidArguments(env.get("tail")?, args.clone()))
                    }
                }),

                "map" => Value::builtin("map", |args, env| {
                    check_args_len(env.get("map")?, &args, 2)?;
                    let func = args[0].eval(env)?;
                    if let Value::List(list) = args[1].eval(env)? {
                        let mut result = vec![];
                        for item in list {
                            result.push(Value::Apply(Box::new(func.clone()), vec![item]).eval(env)?)
                        }
                        Ok(Value::List(result))
                    } else {
                        Err(Error::InvalidArguments(env.get("map")?, args.clone()))
                    }
                }),

                "filter" => Value::builtin("filter", |args, env| {
                    check_args_len(env.get("filter")?, &args, 2)?;
                    let func = args[0].eval(env)?;
        
                    if let Value::List(list) = args[1].eval(env)? {
                        let mut result = vec![];
                        for item in list {
                            let cond = Value::Apply(Box::new(func.clone()), vec![item.clone()]).eval(env)?;
                            if let Value::Boolean(b) = cond {
                                if b {
                                    result.push(item)
                                }
                            } else {
                                return Err(Error::InvalidCondition(cond))
                            }
                        }
                        Ok(Value::List(result))
                    } else {
                        Err(Error::InvalidArguments(env.get("map")?, args.clone()))
                    }
                }),

                "reduce" => Value::builtin("reduce", |args, env| {
                    check_args_len(env.get("reduce")?, &args, 3)?;
                    let func = args[0].eval(env)?;
                    let mut acc = args[1].eval(env)?;
        
                    if let Value::List(list) = args[2].eval(env)? {
                        for item in list {
                            acc = Value::Apply(Box::new(func.clone()), vec![acc.clone(), item.clone()]).eval(env)?;
                        }
                        Ok(acc)
                    } else {
                        Err(Error::InvalidArguments(env.get("reduce")?, args.clone()))
                    }
                }),

                "back" => Value::Macro(vec![], Box::new(Value::Apply(Box::new(Value::Symbol("cd".to_string())), vec![Value::String("..".to_string())]))),

                "add" => Value::Lambda(vec!["x".to_string(), "y".to_string()], Box::new(Value::Add(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Symbol("y".to_string())))), Self::new()),
                "mul" => Value::Lambda(vec!["x".to_string(), "y".to_string()], Box::new(Value::Multiply(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Symbol("y".to_string())))), Self::new()),
                "sub" => Value::Lambda(vec!["x".to_string(), "y".to_string()], Box::new(Value::Subtract(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Symbol("y".to_string())))), Self::new()),
                "div" => Value::Lambda(vec!["x".to_string(), "y".to_string()], Box::new(Value::Divide(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Symbol("y".to_string())))), Self::new()),
                "rem" => Value::Lambda(vec!["x".to_string(), "y".to_string()], Box::new(Value::Remainder(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Symbol("y".to_string())))), Self::new()),

                "sum" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Apply(Box::new(Value::Symbol("reduce".to_string())), vec![Value::Symbol("add".to_string()), Value::Integer(0), Value::Symbol("x".to_string())])), Self::new()),
                "prod" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Apply(Box::new(Value::Symbol("reduce".to_string())), vec![Value::Symbol("mul".to_string()), Value::Integer(1), Value::Symbol("x".to_string())])), Self::new()),

                "inc" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Add(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(1)))), Self::new()),
                "dec" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Subtract(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(1)))), Self::new()),
                
                "double" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Multiply(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(2)))), Self::new()),
                "triple" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Multiply(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(3)))), Self::new()),
                "quadruple" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Multiply(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(4)))), Self::new()),
                "quintuple" => Value::Lambda(vec!["x".to_string()], Box::new(Value::Multiply(Box::new(Value::Symbol("x".to_string())), Box::new(Value::Integer(5)))), Self::new()),

                x => {
                    for t in TYPES {
                        if x == get_os_name(t) {
                            return Ok(Value::String(x.to_lowercase()))
                        } else if x == get_os_family(t) {
                            return Ok(Value::String(x.to_lowercase()))
                        }
                    }

                    return Err(Error::SymbolNotDefined(name.clone()))
                }
            })
        }
    }

    pub fn get_pics_dir(&self) -> Result<PathBuf, Error> {
        dirs::picture_dir().ok_or(Error::PicturesDirectoryNotFound)
    }

    pub fn get_vids_dir(&self) -> Result<PathBuf, Error> {
        dirs::video_dir().ok_or(Error::VideosDirectoryNotFound)
    }

    pub fn get_down_dir(&self) -> Result<PathBuf, Error> {
        dirs::download_dir().ok_or(Error::DownloadsDirectoryNotFound)
    }

    pub fn get_desk_dir(&self) -> Result<PathBuf, Error> {
        dirs::desktop_dir().ok_or(Error::DesktopDirectoryNotFound)
    }

    pub fn get_docs_dir(&self) -> Result<PathBuf, Error> {
        dirs::document_dir().ok_or(Error::DocumentsDirectoryNotFound)
    }

    pub fn get_home_dir(&self) -> Result<PathBuf, Error> {
        dirs::home_dir().ok_or(Error::HomeDirectoryNotFound)
    }

    pub fn get_cwd(&self) -> Result<PathBuf, Error> {
        if let Ok(Value::Path(result)) = self.get(CWD) {
            Ok(result)
        } else {
            Ok(PathBuf::from(self.get_home_dir()?))
        }
    }

    pub(crate) fn get_symbols(&self) -> &BTreeMap<String, Value> {
        &self.symbols
    }

    pub fn is_defined(&self, name: &String) -> bool {
        self.symbols.contains_key(name)
    }

    pub fn define(&mut self, name: impl ToString, value: Value) {
        self.symbols.insert(name.to_string(), value);
    }

    pub fn combine(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.symbols.extend(other.symbols.clone());
        result
    }
}