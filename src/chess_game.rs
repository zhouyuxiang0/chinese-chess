use std::collections::HashMap;
use yew::prelude::*;

pub struct ChessGame {
    chess_map: Vec<(i32, i32)>,
    pieces: Vec<Piece>,
}

#[derive(Clone, PartialEq)]
pub enum Group {
    Red,
    Black,
}

pub struct Piece {
    location: (i32, i32),
    name: &'static str,
    group: Group,
}

impl ChessGame {
    pub fn init_view(&self, x: i32, y: i32) -> Html {
        ChessGame::init_map();
        ChessGame::init_piece_info();
        html! {}
    }

    fn init_map() -> Vec<(i32, i32)> {
        let x = 9;
        let y = 10;
        let mut chess_map = vec![];
        for y in 1..=y {
            for x in 1..=x {
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
