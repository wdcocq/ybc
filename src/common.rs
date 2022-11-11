use strum::IntoStaticStr;
use yew::prelude::*;

/// Common alignment classes.
#[derive(Clone, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum Alignment {
    #[strum(to_string = "is-left")]
    Left,
    #[strum(to_string = "is-centered")]
    Centered,
    #[strum(to_string = "is-right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum Size {
    #[strum(to_string = "is-small")]
    Small,
    #[strum(to_string = "is-normal")]
    Normal,
    #[strum(to_string = "is-medium")]
    Medium,
    #[strum(to_string = "is-large")]
    Large,
}

impl_classes_from!(Alignment, Size);

enum ColorPrinter {
    Common,
    Text,
    Background,
}

/// Common color classes
pub type Color = BaseColor<{ ColorPrinter::Common as usize }>;

/// Text color classes
pub type TextColor = BaseColor<{ ColorPrinter::Text as usize }>;

/// Background color classes
pub type BackgroundColor = BaseColor<{ ColorPrinter::Background as usize }>;

/// Don't use this type directly, use [`Color`], [`TextColor`] or [`BackgroundColor`] instead
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum BaseColor<const P: usize> {
    #[strum(to_string = "white")]
    White,
    #[strum(to_string = "white-bis")]
    WhiteBis,
    #[strum(to_string = "white-ter")]
    WhiteTer,
    #[strum(to_string = "grey")]
    Grey,
    #[strum(to_string = "grey-lighter")]
    GreyLighter,
    #[strum(to_string = "grey-light")]
    GreyLight,
    #[strum(to_string = "grey-dark")]
    GreyDark,
    #[strum(to_string = "grey-darker")]
    GreyDarker,
    #[strum(to_string = "black-ter")]
    BlackTer,
    #[strum(to_string = "black-bis")]
    BlackBis,
    #[strum(to_string = "black")]
    Black,
    #[strum(to_string = "light")]
    Light,
    #[strum(to_string = "dark")]
    Dark,
    #[strum(to_string = "primary")]
    Primary,
    #[strum(to_string = "primary-light")]
    PrimaryLight,
    #[strum(to_string = "primary-dark")]
    PrimaryDark,
    #[strum(to_string = "link")]
    Link,
    #[strum(to_string = "link-light")]
    LinkLight,
    #[strum(to_string = "link-dark")]
    LinkDark,
    #[strum(to_string = "info")]
    Info,
    #[strum(to_string = "info-light")]
    InfoLight,
    #[strum(to_string = "info-dark")]
    InfoDark,
    #[strum(to_string = "success")]
    Success,
    #[strum(to_string = "success-light")]
    SuccessLight,
    #[strum(to_string = "success-dark")]
    SuccessDark,
    #[strum(to_string = "warning")]
    Warning,
    #[strum(to_string = "warning-light")]
    WarningLight,
    #[strum(to_string = "warning-dark")]
    WarningDark,
    #[strum(to_string = "danger")]
    Danger,
    #[strum(to_string = "danger-light")]
    DangerLight,
    #[strum(to_string = "danger-dark")]
    DangerDark,
}

impl From<Color> for Classes {
    fn from(color: Color) -> Self {
        format!("is-{}", <&'static str>::from(color)).into()
    }
}

impl From<TextColor> for Classes {
    fn from(color: TextColor) -> Self {
        format!("has-text-{}", <&'static str>::from(color)).into()
    }
}

impl From<BackgroundColor> for Classes {
    fn from(color: BackgroundColor) -> Self {
        format!("has-background-{}", <&'static str>::from(color)).into()
    }
}
