use crate::chessplaceholder::ChessPlaceHolder;
use stylist::yew::styled_component;
use web_sys::console;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChessboardProp {
    pub placeholder_size: i32,
}

#[styled_component(Chessboard)]
pub fn chessboard(prop: &ChessboardProp) -> Html {
    let x = use_state(|| 9);
    let y = use_state(|| 10);
    let mut chess_map = vec![];
    for x in 1..=*x {
        for y in 1..=*y {
            chess_map.push((x, y));
        }
    }
    println!("{:?}", chess_map);
    let placeholder_size = prop.clone().placeholder_size;
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
                    html!{<ChessPlaceHolder size={placeholder_size} x={*x} y={*y} />}
                }).collect::<Html>()
            }
        </div>
    }
}
