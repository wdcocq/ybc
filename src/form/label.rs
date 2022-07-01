use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub horizontal: bool,
}

#[function_component(Label)]
pub fn label(LabelProps { children, classes, horizontal }: &LabelProps) -> Html {
    html! {
        if *horizontal {
            <div class={classes!(classes, "field-label")}>
                <label class="label">{children.clone()}</label>
            </div>
        } else {
            <label class={classes!(classes, "label")}>{children.clone()}</label>
        }
    }
}
