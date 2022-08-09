use stylist::yew::{styled_component, use_media_query};
use yew::prelude::*;
mod chessboard;
mod chessplaceholder;
use chessboard::Chessboard;

#[styled_component(App)]
pub fn app() -> Html {
    let is_small = use_media_query("(max-width: 720px)");
    let (board_size, placeholder_size) = if is_small { (720, 80) } else { (540, 60) };
    html! {
        <>
            <Chessboard board_size={board_size} placeholder_size={placeholder_size} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
