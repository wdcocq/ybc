use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A simple menu, for any type of vertical navigation.
///
/// <https://bulma.io/documentation/components/menu/>
#[function_component(Menu)]
pub fn menu(MenuProps { children, classes }: &MenuProps) -> Html {
    basic_comp!(<aside>, children, classes.clone(), "menu")
}

//////////////////////////////////////////////////////////////////////////////

/// A container for menu list `li` elements.
///
/// <https://bulma.io/documentation/components/menu/>
#[function_component(MenuList)]
pub fn menu_list(MenuProps { children, classes }: &MenuProps) -> Html {
    basic_comp!(<ul>, children, classes.clone(), "menu-list")
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq, Eq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The text of the label.
    pub text: AttrValue,
}

/// A label for a section of the menu.
///
/// <https://bulma.io/documentation/components/menu/>
#[function_component(MenuLabel)]
pub fn menu_label(MenuLabelProps { classes, text }: &MenuLabelProps) -> Html {
    basic_comp!(<p>, {text}, classes.clone(), "menu-label")
}
