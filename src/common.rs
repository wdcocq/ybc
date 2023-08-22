use strum::IntoStaticStr;
use yew::{html::ImplicitClone, prelude::*};

/// Common alignment classes.
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum Alignment {
    #[strum(to_string = "is-left")]
    Left,
    #[strum(to_string = "is-centered")]
    Centered,
    #[strum(to_string = "is-right")]
    Right,
}

impl ImplicitClone for Alignment {}

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

impl ImplicitClone for Size {}

impl_classes_from!(Alignment, Size);

macro_rules! color_str {
    ($prefix:literal, $base:literal) => {
        concat!($prefix, $base)
    };
    ("is-", $base:literal, $tint:literal) => {
        concat!("is-", $base, " is-", $tint)
    };
    ($prefix:literal, $base:literal, $tint:literal) => {
        concat!($prefix, $base, '-', $tint)
    };
}

/// Common flex grow classes.
///
/// <https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum FlexGrow {
    #[strum(to_string = "is-flex-grow-0")]
    Zero,
    #[strum(to_string = "is-flex-grow-1")]
    One,
    #[strum(to_string = "is-flex-grow-2")]
    Two,
    #[strum(to_string = "is-flex-grow-3")]
    Three,
    #[strum(to_string = "is-flex-grow-4")]
    Four,
    #[strum(to_string = "is-flex-grow-5")]
    Five,
}

/// Common flex shrink classes.
///
/// <https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum FlexShrink {
    #[strum(to_string = "is-flex-shrink-0")]
    Zero,
    #[strum(to_string = "is-flex-shrink-1")]
    One,
    #[strum(to_string = "is-flex-shrink-2")]
    Two,
    #[strum(to_string = "is-flex-shrink-3")]
    Three,
    #[strum(to_string = "is-flex-shrink-4")]
    Four,
    #[strum(to_string = "is-flex-shrink-5")]
    Five,
}

impl ImplicitClone for FlexGrow {}
impl ImplicitClone for FlexShrink {}

impl_classes_from!(FlexGrow, FlexShrink);

#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum Breakpoint {
    #[strum(to_string = "mobile")]
    Mobile,
    #[strum(to_string = "tablet")]
    Tablet,
    #[strum(to_string = "touch")]
    Touch,
    #[strum(to_string = "desktop")]
    Desktop,
    #[strum(to_string = "widescreen")]
    Widescreen,
    #[strum(to_string = "fullhd")]
    FullHD,
}

impl ImplicitClone for Breakpoint {}

macro_rules! colors {
    ($(($variant:ident: $($base:tt)+),)*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        /// Color classes for elements
        pub enum Color {
            $( $variant, )*
            /// Define a custom color with a base and tint, will result in the classes 'is-{base} is-{tint}' or 'is-{base}' if tint is None.
            Custom(
                /// Base color
                &'static str,
                /// Tint
                Option<&'static str>),
        }

        impl From<Color> for Classes {
            fn from(color: Color) -> Classes {
                match color {
                    $( Color::$variant => color_str!("is-", $($base)+).into(), )*
                    Color::Custom(base, Some(tint)) => format!("is-{base} is-{tint}").into(),
                    Color::Custom(base, _) => format!("is-{base}").into(),
                }
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        /// Color classes for elment text
        pub enum TextColor {
            $( $variant, )*
            /// Define a custom color with a base and tint, will result in the class 'has-text-{base}-{tint}' or 'has-text-{base}' if tint is None.
            Custom(
                /// Base color
                &'static str,
                /// Tint
                Option<&'static str>)
        }

        impl From<TextColor> for Classes {
            fn from(color: TextColor) -> Classes {
                match color {
                    $( TextColor::$variant => color_str!("has-text-", $($base)+).into(), )*
                    TextColor::Custom(base, Some(tint)) => format!("has-text-{base}-{tint}").into(),
                    TextColor::Custom(base, _) => format!("has-text-{base}").into(),
                }
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        /// Color classes for element backgrounds
        pub enum BackgroundColor {
            $( $variant, )*
            /// Define a custom color with a base and tint, will result in the class 'has-background-{base}-{tint}' or 'has-background-{base}' if tint is None.
            Custom(
                /// Base color
                &'static str,
                /// Tint
                Option<&'static str>)
        }

        impl From<BackgroundColor> for Classes {
            fn from(color: BackgroundColor) -> Classes {
                match color {
                    $( BackgroundColor::$variant => color_str!("has-background-", $($base)+).into(), )*
                    BackgroundColor::Custom(base, Some(tint)) => format!("has-background-{base}-{tint}").into(),
                    BackgroundColor::Custom(base, _) => format!("has-background-{base}").into(),
                }
            }
        }
    };
}

colors! {
    (White: "white"),
    (WhiteBis: "white-bis"),
    (WhiteTer: "white-ter"),
    (Grey: "grey"),
    (GreyLighter: "grey-lighter"),
    (GreyLight: "grey-light"),
    (GreyDark: "grey-dark"),
    (GreyDarker: "grey-darker"),
    (Black: "black"),
    (BlackBis: "black-bis"),
    (BlackTer: "black-ter"),
    (Light: "light"),
    (Dark: "dark"),
    (Primary: "primary"),
    (PrimaryLight: "primary", "light"),
    (PrimaryDark: "primary", "dark"),
    (Link: "link"),
    (LinkLight: "link", "light"),
    (LinkDark: "link", "dark"),
    (Info: "info"),
    (InfoLight: "info", "light"),
    (InfoDark: "info", "dark"),
    (Success: "success"),
    (SuccessLight: "success", "light"),
    (SuccessDark: "success", "dark"),
    (Warning: "warning"),
    (WarningLight: "warning", "light"),
    (WarningDark: "warning", "dark"),
    (Danger: "danger"),
    (DangerLight: "danger", "light"),
    (DangerDark: "danger", "dark"),
    (Ghost: "ghost"),
}

impl ImplicitClone for Color {}
impl ImplicitClone for TextColor {}
impl ImplicitClone for BackgroundColor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        assert_eq!(Classes::from(Color::Primary), classes!("is-primary"));
        assert_eq!(Classes::from(Color::PrimaryDark), classes!("is-primary", "is-dark"));
        assert_eq!(Classes::from(Color::Custom("secondary", None)), classes!("is-secondary"));
        assert_eq!(
            Classes::from(Color::Custom("secondary", Some("light"))),
            classes!("is-secondary", "is-light")
        );
        assert_eq!(Classes::from(TextColor::Info), classes!("has-text-info"));
        assert_eq!(Classes::from(TextColor::InfoLight), classes!("has-text-info-light"));
        assert_eq!(Classes::from(TextColor::Custom("secondary", None)), classes!("has-text-secondary"));
        assert_eq!(Classes::from(BackgroundColor::Success), classes!("has-background-success"));
        assert_eq!(
            Classes::from(BackgroundColor::Custom("secondary", Some("light"))),
            classes!("has-background-secondary-light")
        );
    }
}
