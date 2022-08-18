use stylist::yew::{styled_component, Global};
use yew::prelude::*;
mod chess_game;
mod chessboard;
use chessboard::Chessboard;
use yew::ContextProvider;
use yew_router::prelude::*;

use crate::chess_game::ChessGame;

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
    let ctx = use_state(|| ChessGame::get());
    html! {
        <ContextProvider<ChessGame> context={(*ctx).clone()}>
            <BrowserRouter>
                <Global css="font-family:楷体;" />
                <nav></nav>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer>
                </footer>
            </BrowserRouter>
        </ContextProvider<ChessGame>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
