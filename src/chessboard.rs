use crate::{
    chess_game::{ChessGame, Piece},
    chessplaceholder::ChessPlaceHolder,
};
use stylist::yew::{styled_component, use_media_query};
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
                    match piece {
                        Some(&p) => {
                            html!{
                                <ChessPlaceHolder size={placeholder_size} x={*x} y={*y} piece={p} />
                            }
                        },
                        None => {
                            html!{
                                <ChessPlaceHolder size={placeholder_size} x={*x} y={*y} piece={Option::None} />
                            }
                        },
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
