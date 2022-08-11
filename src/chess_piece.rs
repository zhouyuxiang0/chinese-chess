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
        "{
        width: 70px;
        height: 70px;
        border-radius: 50%;
        :hover {
            background-color: green;
        }
    }",
    )
    .expect("style出错");
    html! {
        <div class={style}>{"---------"}</div>
    }
}
