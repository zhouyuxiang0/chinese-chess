use std::sync::Once;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ChessGame {
    pub chess_map: Vec<(i32, i32)>,
    pub pieces: Vec<Piece>,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Group {
    Red,
    Black,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Piece {
    pub location: (i32, i32),
    pub name: &'static str,
    pub group: Group,
}

static mut CHESS_GAME: ChessGame = ChessGame {
    chess_map: vec![],
    pieces: vec![],
};
static INIT: Once = Once::new();

impl ChessGame {
    pub fn get() -> &'static ChessGame {
        unsafe {
            INIT.call_once(|| {
                CHESS_GAME = ChessGame {
                    chess_map: ChessGame::init_map(),
                    pieces: ChessGame::init_piece_info(),
                };
            });
            &CHESS_GAME
        }
    }

    fn init_map() -> Vec<(i32, i32)> {
        let mut chess_map = vec![];
        for y in 1..=10 {
            for x in 1..=9 {
                chess_map.push((x, y))
            }
        }
        chess_map
    }

    fn init_piece_info() -> Vec<Piece> {
        let pieces = vec![
            Piece {
                location: (5, 1),
                name: "将",
                group: Group::Black,
            },
            Piece {
                location: (1, 1),
                name: "车",
                group: Group::Black,
            },
            Piece {
                location: (9, 1),
                name: "车",
                group: Group::Black,
            },
            Piece {
                location: (2, 1),
                name: "马",
                group: Group::Black,
            },
            Piece {
                location: (8, 1),
                name: "马",
                group: Group::Black,
            },
            Piece {
                location: (2, 3),
                name: "炮",
                group: Group::Black,
            },
            Piece {
                location: (8, 3),
                name: "炮",
                group: Group::Black,
            },
            Piece {
                location: (3, 1),
                name: "象",
                group: Group::Black,
            },
            Piece {
                location: (7, 1),
                name: "象",
                group: Group::Black,
            },
            Piece {
                location: (4, 1),
                name: "士",
                group: Group::Black,
            },
            Piece {
                location: (6, 1),
                name: "士",
                group: Group::Black,
            },
            Piece {
                location: (1, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                location: (3, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                location: (5, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                location: (7, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                location: (9, 4),
                name: "卒",
                group: Group::Black,
            },
            // -------------------------------
            Piece {
                location: (5, 10),
                name: "帅",
                group: Group::Red,
            },
            Piece {
                location: (1, 10),
                name: "车",
                group: Group::Red,
            },
            Piece {
                location: (9, 10),
                name: "车",
                group: Group::Red,
            },
            Piece {
                location: (2, 10),
                name: "马",
                group: Group::Red,
            },
            Piece {
                location: (8, 10),
                name: "马",
                group: Group::Red,
            },
            Piece {
                location: (2, 8),
                name: "炮",
                group: Group::Red,
            },
            Piece {
                location: (8, 8),
                name: "炮",
                group: Group::Red,
            },
            Piece {
                location: (3, 10),
                name: "相",
                group: Group::Red,
            },
            Piece {
                location: (7, 10),
                name: "相",
                group: Group::Red,
            },
            Piece {
                location: (4, 10),
                name: "仕",
                group: Group::Red,
            },
            Piece {
                location: (6, 10),
                name: "仕",
                group: Group::Red,
            },
            Piece {
                location: (1, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                location: (3, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                location: (5, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                location: (7, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                location: (9, 7),
                name: "兵",
                group: Group::Red,
            },
        ];
        pieces
    }
}
