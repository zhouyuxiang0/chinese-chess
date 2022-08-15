use stylist::yew::{styled_component, Global};
use yew::prelude::*;
mod chess_game;
mod chess_piece;
mod chessboard;
mod chessplaceholder;
use chessboard::Chessboard;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <Chessboard />
            }
        }
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Global css="font-family:楷体;" />
            <nav></nav>
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer>
            </footer>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
