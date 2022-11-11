#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_else(|| 0.0)]
    pub value: f32,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
#[function_component(Progress)]
pub fn progress(ProgressProps { classes, max, value }: &ProgressProps) -> Html {
    basic_comp!(<progress [max={max.to_string()} value={value.to_string()}]>, {format!("{value}%")}, classes.clone())
}
