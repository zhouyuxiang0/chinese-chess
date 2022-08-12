use stylist::{yew::styled_component, Style};
use yew::prelude::{Properties, *};
use crate::chess_piece::ChessPiece;

#[derive(Properties, Clone, PartialEq)]
pub struct PlaceHolderProp {
    pub size: i32,
    pub x: i32,
    pub y: i32,
}

#[styled_component(ChessPlaceHolder)]
pub fn chess_placeholder(props: &PlaceHolderProp) -> html {
    let size = props.clone().size;
    let style = make_style(props.x, props.y, size);
    html! {
        <div class={style}>
        // {"x"}{props.x}{"y"}{props.y}
            // {
                if props.x == 1 && props.y == 1 {
                    <ChessPiece name="车" group=true x=1 y=1 />
                } 
            // else {
            //         html!{}
            //     }
            // }
        </div>
    }
}

fn has_after(x: i32) -> bool {
    if x == 9 {
        return false;
    }
    true
}

fn has_before(y: i32) -> bool {
    if y == 10 {
        return false;
    }
    true
}

fn match_content(x: i32, y: i32) -> &'static str {
    if y == 5 && x == 2 {
        "楚"
    } else if y == 5 && x == 3 {
        "河"
    } else if y == 5 && x == 6 {
        "汉"
    } else if y == 5 && x == 7 {
        "界"
    } else {
        ""
    }
}

fn make_style(x: i32, y: i32, size: i32) -> Style {
    let mut base_style = format!(
        "
        height:{}px;
        width:{}px;
        position: relative;",
        size, size
    );
    if has_after(x) {
        let content = match_content(x, y);
        if content == "" {
            base_style += "
            &:after {
                content:'';
                position: absolute;
                left: 50%;
                top: calc(50% - 1px);
                border-top: 1px solid black;
                width: 100%
            }
            "
        } else {
            base_style += format!(
                "
            &:after {{
                content:'{}';
                position: absolute;
                left: 50%;
                top: calc(50% - 1px);
                border-top: 1px solid black;
                width: 100%;
                height: {}px;
                line-height: {}px;
                text-align: center;
                font-size: 37px;
            }}
            ",
                content, size, size
            )
            .as_str();
        }
    }
    if x == 5 && (y == 2 || y == 9) {
        base_style += "
            background:linear-gradient(45deg,transparent 49.6%, black 49.6%, black 50.4%, transparent 50.4%),
            linear-gradient(-45deg,transparent 49.6%, black 49.6%, black 50.4%, transparent 50.4%);
            transform: scale(2);
            &:before {
                transform: scale(0.5) !important;
                top: 25% !important;
            }
            &:after {
                transform: scale(0.5) !important;
                left: 25% !important;
            }
        ";
    }
    if has_before(y) {
        if y == 5 && x != 1 && x != 9 {
            base_style += "
            &:before {
                content: '';
                position: absolute;
                left: calc(50% - 1px);
                top: -50%;
                border-right: 1px solid black;
                height: 100%;
            }
            ";
        } else {
            base_style += "
            &:before {
                content: '';
                position: absolute;
                left: calc(50% - 1px);
                top: 50%;
                border-right: 1px solid black;
                height: 100%
            }
            ";
        }
    }
    Style::new(base_style.to_owned()).expect("style出错")
}
