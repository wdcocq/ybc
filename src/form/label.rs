use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub horizontal: bool,
}

#[function_component(Label)]
pub fn label(LabelProps { children, classes, horizontal }: &LabelProps) -> Html {
    if *horizontal {
        html! {
            <div class={classes!(classes.clone(), "field-label")}>
                <label class="label">{children.clone()}</label>
            </div>
        }
    } else {
        basic_comp!(<label>, children, classes.clone(), "label")
    }
}
