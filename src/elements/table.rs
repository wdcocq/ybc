use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// NodeRef to the `<table>` element
    #[prop_or_default]
    pub table_ref: NodeRef,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// <https://bulma.io/documentation/elements/table/>

#[function_component(Table)]
pub fn table(
    TableProps {
        children,
        classes,
        table_ref,
        bordered,
        striped,
        narrow,
        hoverable,
        fullwidth,
        scrollable,
    }: &TableProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "table",
        bordered.then_some("is-bordered"),
        striped.then_some("is-striped"),
        narrow.then_some("is-narrow"),
        hoverable.then_some("is-hoverable"),
        fullwidth.then_some("is-fullwidth")
    );

    html! {
        if *scrollable {
            <div class="table-container">
                <table ref={table_ref} class={classes}>
                    {children.clone()}
                </table>
            </div>
        } else {
            <table ref={table_ref} class={classes}>
                {children.clone()}
            </table>
        }
    }
}
