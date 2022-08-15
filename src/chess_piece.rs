use stylist::{yew::styled_component, Style};
use yew::html;
use yew::prelude::*;

use crate::chess_game::Group;

#[derive(Properties, Clone, PartialEq)]
pub struct ChessPieceProps {
    pub name: &'static str,
    pub group: Group,
    pub x: i32,
    pub y: i32,
}

#[styled_component(ChessPiece)]
pub fn chess_piece(props: &ChessPieceProps) -> Html {
    let style = Style::new(
        "
        width: 70%;
        height: 70%;
        border-radius: 50%;
        display:flex;
        align-items:center;
        justify-content:center;
        background-color: brown;
        font-size: 32px;
        z-index:50;
        :hover {
            background-color: green;
        }
    ",
    )
    .expect("style出错");
    html! {
        <div class={style}>{props.name}</div>
    }
}
