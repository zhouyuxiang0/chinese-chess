use crate::chessplaceholder::ChessPlaceHolder;
use stylist::yew::{styled_component, use_media_query};
use yew::prelude::*;

#[styled_component(Chessboard)]
pub fn chessboard() -> Html {
    let is_small = use_media_query("(max-width: 720px)");
    let placeholder_size = if is_small { 50 } else { 70 };
    let x = use_state(|| 9);
    let y = use_state(|| 10);
    let mut chess_map = vec![];
    for y in 1..=*y {
        for x in 1..=*x {
            chess_map.push((x, y));
        }
    }
    let (x_size, y_size) = (*x.clone() * placeholder_size, *y.clone() * placeholder_size);
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
                chess_map.iter().map(|(x, y)| {
                    html!{
                        <ChessPlaceHolder size={placeholder_size} x={*x} y={*y} />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
