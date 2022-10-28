use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Align child columns vertically.
    #[prop_or_default]
    pub vcentered: bool,
    /// Allow for multiline rows.
    #[prop_or_default]
    pub multiline: bool,
    /// Center all child columns within their row.
    #[prop_or_default]
    pub centered: bool,
    // Remove gaps between columns
    #[prop_or_default]
    pub gapless: bool,
}

/// The container for a set of responsive columns.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
#[function_component(Columns)]
pub fn columns(
    ColumnsProps {
        children,
        classes,
        vcentered,
        multiline,
        centered,
        gapless,
    }: &ColumnsProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "columns",
        vcentered.then_some("is-vcentered"),
        multiline.then_some("is-multiline"),
        centered.then_some("is-centered"),
        gapless.then_some("is-gapless"),
    );

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub narrow: bool,
}

/// A flexbox-based responsive column.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
#[function_component(Column)]
pub fn column(ColumnProps { children, classes, narrow }: &ColumnProps) -> Html {
    let classes = classes!(classes.clone(), "column", narrow.then_some("is-narrow"));

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}
