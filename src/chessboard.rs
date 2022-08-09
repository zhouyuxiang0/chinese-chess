use crate::chessplaceholder::ChessPlaceHolder;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChessboardProp {
    pub board_size: i32,
    pub placeholder_size: i32,
}

#[styled_component(Chessboard)]
pub fn chessboard(prop: &ChessboardProp) -> Html {
    let x = use_state(|| 9);
    let y = use_state(|| 10);
    html! {
        <div class={css!("border: 2px solid black;
        margin: 0 auto;
        width: 720px;
        height: 720px;
        position: relative;")}>
            <ChessPlaceHolder size={prop.clone().placeholder_size} />
        </div>
    }
}
