use stylist::{yew::styled_component, Style};
use yew::html;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChessPieceProps {
    pub name: &'static str,
    pub group: bool,
    pub x: i32,
    pub y: i32,
}

#[styled_component(ChessPiece)]
pub fn chess_piece(props: &ChessPieceProps) -> Html {
    let style = Style::new(
        "
        width: 45px;
        height: 45px;
        border-radius: 50%;
        line-height: 45px;
        text-align: center;
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
