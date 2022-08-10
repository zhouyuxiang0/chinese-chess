use stylist::yew::styled_component;
use yew::prelude::{Properties, *};

#[derive(Properties, Clone, PartialEq)]
pub struct PlaceHolderProp {
    pub size: i32,
    pub x: i32,
    pub y: i32
}

#[styled_component(ChessPlaceHolder)]
pub fn chess_placeholder(props: &PlaceHolderProp) -> html {
    let size = props.clone().size;
    html! {
        <div class={css!("
            &:before {
                content: '';
                position: absolute;
                left: calc(50% - 1px);
                top: 50%;
                border-right: 1px solid black;
                height: 100%
            }
            &:after {
                content:'';
                position: absolute;
                left: 50%;
                top: calc(50% - 1px);
                border-top: 1px solid black;
                width: 100%
            }
            :hover {
                background-color: black;
            }
            position: relative;
        ")} style={format!("height:{}px;width:{}px;", size, size)}>{"x"}{props.clone().x}{"y"}{props.clone().y}</div>
    }
}
