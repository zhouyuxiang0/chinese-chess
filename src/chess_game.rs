use std::sync::Once;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ChessGame {
    pub chess_map: Vec<(i32, i32)>,
    pub pieces: Vec<Piece>,
    pub selected: Option<Piece>,
    pub current_round: Group,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Group {
    Red,
    Black,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Piece {
    pub id: usize,
    pub location: (i32, i32),
    pub name: &'static str,
    pub group: Group,
}

impl Piece {
    pub fn move_to(&mut self) -> &Self {
        self.location = (3, 3);
        self
    }
}

static mut CHESS_GAME: ChessGame = ChessGame {
    chess_map: vec![],
    pieces: vec![],
    selected: Option::None,
    current_round: Group::Red,
};
static INIT: Once = Once::new();

impl ChessGame {
    pub fn get() -> &'static mut ChessGame {
        unsafe {
            INIT.call_once(|| {
                CHESS_GAME = ChessGame {
                    chess_map: ChessGame::init_map(),
                    pieces: ChessGame::init_piece_info(),
                    selected: Option::None,
                    current_round: Group::Red,
                };
            });
            &mut CHESS_GAME
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
                id: 1,
                location: (5, 1),
                name: "将",
                group: Group::Black,
            },
            Piece {
                id: 2,
                location: (1, 1),
                name: "车",
                group: Group::Black,
            },
            Piece {
                id: 3,
                location: (9, 1),
                name: "车",
                group: Group::Black,
            },
            Piece {
                id: 4,
                location: (2, 1),
                name: "马",
                group: Group::Black,
            },
            Piece {
                id: 5,
                location: (8, 1),
                name: "马",
                group: Group::Black,
            },
            Piece {
                id: 6,
                location: (2, 3),
                name: "炮",
                group: Group::Black,
            },
            Piece {
                id: 7,
                location: (8, 3),
                name: "炮",
                group: Group::Black,
            },
            Piece {
                id: 8,
                location: (3, 1),
                name: "象",
                group: Group::Black,
            },
            Piece {
                id: 9,
                location: (7, 1),
                name: "象",
                group: Group::Black,
            },
            Piece {
                id: 10,
                location: (4, 1),
                name: "士",
                group: Group::Black,
            },
            Piece {
                id: 11,
                location: (6, 1),
                name: "士",
                group: Group::Black,
            },
            Piece {
                id: 12,
                location: (1, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                id: 13,
                location: (3, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                id: 14,
                location: (5, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                id: 15,
                location: (7, 4),
                name: "卒",
                group: Group::Black,
            },
            Piece {
                id: 16,
                location: (9, 4),
                name: "卒",
                group: Group::Black,
            },
            // -------------------------------
            Piece {
                id: 17,
                location: (5, 10),
                name: "帅",
                group: Group::Red,
            },
            Piece {
                id: 18,
                location: (1, 10),
                name: "车",
                group: Group::Red,
            },
            Piece {
                id: 19,
                location: (9, 10),
                name: "车",
                group: Group::Red,
            },
            Piece {
                id: 10,
                location: (2, 10),
                name: "马",
                group: Group::Red,
            },
            Piece {
                id: 21,
                location: (8, 10),
                name: "马",
                group: Group::Red,
            },
            Piece {
                id: 22,
                location: (2, 8),
                name: "炮",
                group: Group::Red,
            },
            Piece {
                id: 23,
                location: (8, 8),
                name: "炮",
                group: Group::Red,
            },
            Piece {
                id: 24,
                location: (3, 10),
                name: "相",
                group: Group::Red,
            },
            Piece {
                id: 25,
                location: (7, 10),
                name: "相",
                group: Group::Red,
            },
            Piece {
                id: 26,
                location: (4, 10),
                name: "仕",
                group: Group::Red,
            },
            Piece {
                id: 27,
                location: (6, 10),
                name: "仕",
                group: Group::Red,
            },
            Piece {
                id: 28,
                location: (1, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                id: 29,
                location: (3, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                id: 30,
                location: (5, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                id: 31,
                location: (7, 7),
                name: "兵",
                group: Group::Red,
            },
            Piece {
                id: 32,
                location: (9, 7),
                name: "兵",
                group: Group::Red,
            },
        ];
        pieces
    }
}
