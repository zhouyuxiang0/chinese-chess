use stylist::yew::{styled_component, use_media_query, Global};
use yew::prelude::*;
mod chessboard;
mod chessplaceholder;
mod chess_gird;
use chessboard::Chessboard;

#[styled_component(App)]
pub fn app() -> Html {
    let is_small = use_media_query("(max-width: 720px)");
    let placeholder_size = if is_small { 50 } else { 70 };
    html! {
        <>
            <Global css="font-family:楷体;" />
            <Chessboard placeholder_size={placeholder_size} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
