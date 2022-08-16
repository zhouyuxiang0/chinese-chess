use stylist::{yew::styled_component, Style};
use yew::html;
use yew::prelude::*;

use crate::chess_game::Group;
use crate::chess_game::Piece;

#[derive(Properties, Clone, PartialEq)]
pub struct ChessPieceProps {
    pub piece: Piece,
}

#[styled_component(ChessPiece)]
pub fn chess_piece(props: &ChessPieceProps) -> Html {
    let font_color = if props.piece.group == Group::Black {
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
        background-color: brown;
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
    let name = use_state(|| props.piece.name);
    let onclick = {
        let name = name.clone();
        Callback::from(move |_| name.set("value"))
    };
    html! {
        <div class={style} {onclick}>{*name}</div>
    }
}
