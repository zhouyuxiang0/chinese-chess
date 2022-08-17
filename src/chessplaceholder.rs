use crate::{
    chess_game::{ChessGame, Group, Piece},
    chess_piece::ChessPiece,
};
use stylist::{yew::styled_component, Style};
use yew::prelude::{Properties, *};

#[derive(Properties, Clone, PartialEq)]
pub struct PlaceHolderProp {
    pub size: i32,
    pub x: i32,
    pub y: i32,
}

#[styled_component(ChessPlaceHolder)]
pub fn chess_placeholder(props: &PlaceHolderProp) -> html {
    let chess_game = ChessGame::get().to_owned();
    let size = props.clone().size;
    let style = make_style(props.x, props.y, size);
    let game_state = use_state(|| chess_game);
    let piece = game_state
        .pieces
        .iter()
        .find(|piece| piece.location.0 == props.x && piece.location.1 == props.y);
    let onclick: Callback<_> = {
        let pieces = game_state.pieces.clone();
        let game = game_state.clone();
        match piece {
            Some(p) => Callback::from(move |_: MouseEvent| {
                let pieces: Vec<Piece> = pieces
                    .iter()
                    .map(|&p| {
                        if p.location == p.location {
                            Piece {
                                location: (p.location.0 + 1, p.location.1 + 1),
                                name: p.name,
                                group: p.group,
                            }
                        } else {
                            p
                        }
                    })
                    .collect();
                let n = ChessGame {
                    selected: Option::None,
                    chess_map: game.chess_map.clone(),
                    pieces,
                    current_round: if game.current_round == Group::Red {
                        Group::Black
                    } else {
                        Group::Red
                    },
                };
                game.set(n);
            }),
            None => Callback::from(|_| {}),
        }
    };
    html! {
        <div class={style}>
            {
                match piece {
                    Some(p) => {
                        html! {
                           <ChessPiece piece={*p}/>
                        }
                    }
                    None => html! {
                    },
                }
            }
        </div>
    }
}

fn has_after(x: i32) -> bool {
    if x == 9 {
        return false;
    }
    true
}

fn has_before(y: i32) -> bool {
    if y == 10 {
        return false;
    }
    true
}

fn match_content(x: i32, y: i32) -> &'static str {
    if y == 5 && x == 2 {
        "楚"
    } else if y == 5 && x == 3 {
        "河"
    } else if y == 5 && x == 6 {
        "汉"
    } else if y == 5 && x == 7 {
        "界"
    } else {
        ""
    }
}

fn make_style(x: i32, y: i32, size: i32) -> Style {
    let mut base_style = format!(
        "
        height:{}px;
        width:{}px;
        position: relative;
        display:flex;
        align-items:center;
        justify-content:center;",
        size, size
    );
    if has_after(x) {
        let content = match_content(x, y);
        if content == "" {
            base_style += "
            &:after {
                content:'';
                position: absolute;
                left: 50%;
                top: calc(50% - 1px);
                border-top: 1px solid black;
                width: 100%
            }
            "
        } else {
            base_style += format!(
                "
            &:after {{
                content:'{}';
                position: absolute;
                left: 50%;
                top: calc(50% - 1px);
                border-top: 1px solid black;
                width: 100%;
                height: {}px;
                line-height: {}px;
                text-align: center;
                font-size: 37px;
            }}
            ",
                content, size, size
            )
            .as_str();
        }
    }
    if x == 5 && (y == 2 || y == 9) {
        base_style += "
            background:linear-gradient(45deg,transparent 49.6%, black 49.6%, black 50.4%, transparent 50.4%),
            linear-gradient(-45deg,transparent 49.6%, black 49.6%, black 50.4%, transparent 50.4%);
            transform: scale(2);
            &:before {
                transform: scale(0.5) !important;
                top: 25% !important;
            }
            &:after {
                transform: scale(0.5) !important;
                left: 25% !important;
            }
        ";
    }
    if has_before(y) {
        if y == 5 && x != 1 && x != 9 {
            base_style += "
            &:before {
                content: '';
                position: absolute;
                left: calc(50% - 1px);
                top: -50%;
                border-right: 1px solid black;
                height: 100%;
            }
            ";
        } else {
            base_style += "
            &:before {
                content: '';
                position: absolute;
                left: calc(50% - 1px);
                top: 50%;
                border-right: 1px solid black;
                height: 100%
            }
            ";
        }
    }
    Style::new(base_style.to_owned()).expect("style出错")
}
