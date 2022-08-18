use crate::chess_game::{ChessGame, Group, Piece};
use stylist::{
    yew::{styled_component, use_media_query},
    Style,
};
use yew::prelude::*;

#[styled_component(Chessboard)]
pub fn chessboard() -> Html {
    let is_small = use_media_query("(max-width: 720px)");
    let placeholder_size = if is_small { 50 } else { 70 };
    let game = ChessGame::get();
    let (x_size, y_size) = (9 * placeholder_size, 10 * placeholder_size);
    let map = game
        .chess_map
        .iter()
        .map(|(x, y)| {
            let piece = game
                .pieces
                .iter()
                .find(|&piece| piece.location.0 == *x && piece.location.1 == *y);
            (*x, *y, piece)
        })
        .collect::<Vec<(i32, i32, Option<&Piece>)>>();
    let map = use_state(|| map);
    let game = use_context::<ChessGame>().expect("no ctx found");
    html! {
        <div class={css!("
                border: 1px solid black;
                display: grid;
                border-collapse: collapse;
                margin: 0 auto;
                position: relative;
            ")}
            style={format!("
                width:{}px;
                height:{}px;
                grid-template-columns: repeat(9,{}px);
            ", x_size, y_size, placeholder_size)}>
            {
                map.iter().map(|(x, y, piece)| {
                    let placeholder_style = make_style(*x, *y, placeholder_size);
                    let onclick: Callback<_> = {
                        let map = map.clone();
                        let piece = piece.clone();
                        Callback::from(move |e: MouseEvent| {
                            log::log!(log::Level::Debug, "{:?}", piece);
                        })
                    };
                    html!{
                        <div class={placeholder_style} {onclick}>
                            {
                                match *piece {
                                    Some(piece) => {
                                        let font_color = if piece.group == Group::Black {
                                            "black"
                                        } else {
                                            "red"
                                        };
                                        let style = Style::new(format!(
                                            "
                                            width: 70%;
                                            height: 70%;
                                            border-radius: 50%;
                                            display:flex;
                                            align-items:center;
                                            justify-content:center;
                                            background-color: #e09a59;
                                            font-size: 32px;
                                            color: {};
                                            z-index:50;
                                            :hover {{
                                                background-color: green;
                                            }}
                                        ",
                                            font_color
                                        ))
                                        .expect("style出错");
                                        html!{
                                            <div class={style}>{piece.name}</div>
                                        }
                                    },
                                    None => {
                                        html!{}
                                    }
                                }
                            }
                        </div>
                    }
                }).collect::<Html>()
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
