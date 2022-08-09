use stylist::yew::styled_component;
use yew::prelude::{Properties, *};

#[derive(Properties, Clone, PartialEq)]
pub struct PlaceHolderProp {
    pub size: i32,
}

#[styled_component(ChessPlaceHolder)]
pub fn chess_placeholder(props: &PlaceHolderProp) -> html {
    props.clone().size;
    html! {
        <div class={css!(r#"
        background-color: rgb(61, 42, 204);
    "#)}></div>
    }
}
