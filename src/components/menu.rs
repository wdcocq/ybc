use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple menu, for any type of vertical navigation.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(Menu)]
pub fn menu(MenuProps { children, classes }: &MenuProps) -> Html {
    let classes = classes!(classes, "menu");
    html! {
        <aside class={classes}>
            {children.clone()}
        </aside>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for menu list `li` elements.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuList)]
pub fn menu_list(MenuListProps { children, classes }: &MenuListProps) -> Html {
    let classes = classes!(classes, "menu-list");
    html! {
        <ul class={classes}>
            {children.clone()}
        </ul>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The text of the label.
    pub text: String,
}

/// A label for a section of the menu.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuLabel)]
pub fn menu_label(MenuLabelProps { classes, text }: &MenuLabelProps) -> Html {
    let classes = classes!(classes, "menu-label");
    html! {
        <p class={classes}>
            {text.as_str()}
        </p>
    }
}
