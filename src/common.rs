use derive_more::Display;
use std::borrow::Cow;
use yew::{html::IntoPropValue, Classes};

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Alignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

impl From<Alignment> for Classes {
    fn from(alignment: Alignment) -> Self {
        alignment.to_string().into()
    }
}

/// Common size classes.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Size {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

impl IntoPropValue<Cow<'static, str>> for Size {
    fn into_prop_value(self) -> Cow<'static, str> {
        Cow::from(self.to_string())
    }
}

impl From<Size> for Classes {
    fn from(size: Size) -> Self {
        size.to_string().into()
    }
}

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
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum BaseColor<const P: usize> {
    #[display(fmt = "white")]
    White,
    #[display(fmt = "white-bis")]
    WhiteBis,
    #[display(fmt = "white-ter")]
    WhiteTer,
    #[display(fmt = "grey")]
    Grey,
    #[display(fmt = "grey-lighter")]
    GreyLighter,
    #[display(fmt = "grey-light")]
    GreyLight,
    #[display(fmt = "grey-dark")]
    GreyDark,
    #[display(fmt = "grey-darker")]
    GreyDarker,
    #[display(fmt = "black-ter")]
    BlackTer,
    #[display(fmt = "black-bis")]
    BlackBis,
    #[display(fmt = "black")]
    Black,
    #[display(fmt = "light")]
    Light,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "primary")]
    Primary,
    #[display(fmt = "primary-light")]
    PrimaryLight,
    #[display(fmt = "primary-dark")]
    PrimaryDark,
    #[display(fmt = "link")]
    Link,
    #[display(fmt = "link-light")]
    LinkLight,
    #[display(fmt = "link-dark")]
    LinkDark,
    #[display(fmt = "info")]
    Info,
    #[display(fmt = "info-light")]
    InfoLight,
    #[display(fmt = "info-dark")]
    InfoDark,
    #[display(fmt = "success")]
    Success,
    #[display(fmt = "success-light")]
    SuccessLight,
    #[display(fmt = "success-dark")]
    SuccessDark,
    #[display(fmt = "warning")]
    Warning,
    #[display(fmt = "warning-light")]
    WarningLight,
    #[display(fmt = "warning-dark")]
    WarningDark,
    #[display(fmt = "danger")]
    Danger,
    #[display(fmt = "danger-light")]
    DangerLight,
    #[display(fmt = "danger-dark")]
    DangerDark,
    #[display(fmt = "{_0}")]
    Custom(&'static str),
}

impl From<Color> for Classes {
    fn from(color: Color) -> Self {
        format!("is-{}", color).into()
    }
}

impl From<TextColor> for Classes {
    fn from(color: TextColor) -> Self {
        format!("has-text-{}", color).into()
    }
}

impl From<BackgroundColor> for Classes {
    fn from(color: BackgroundColor) -> Self {
        format!("has-background-{}", color).into()
    }
}
