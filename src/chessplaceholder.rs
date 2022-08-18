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
    pub piece: Option<Piece>,
}

#[styled_component(ChessPlaceHolder)]
pub fn chess_placeholder(props: &PlaceHolderProp) -> html {
    let size = props.clone().size;
    let style = make_style(props.x, props.y, size);
    let piece = use_state(|| props.piece);
    let onclick = {
        let piece = piece.clone();
        Callback::from(move |e: MouseEvent| {
            log::log!(log::Level::Debug, "{:?}", e);
            match *piece {
                Some(p) => piece.set(Option::Some(Piece {
                    location: (p.location.0 + 1, p.location.1 + 1),
                    ..p
                })),
                None => {
                    //
                }
            };
            // let Some(p) = piece;
        })
    };
    match *piece {
        Some(piece) => {
            html! {
                <div class={style} {onclick}>
                    <ChessPiece piece={piece} />
                </div>
            }
        }
        None => {
            html! {
                <div class={style} {onclick}>
                </div>
            }
        }
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
