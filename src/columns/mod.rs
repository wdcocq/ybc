use strum::IntoStaticStr;
use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

use crate::Breakpoint;

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
    /// Remove gaps between columns
    #[prop_or_default]
    pub gapless: bool,
    /// Activate columns from mobile upwards
    /// <https://bulma.io/documentation/columns/responsiveness/#mobile-columns>
    #[prop_or_default]
    pub mobile: bool,
    /// Activate columns from desktop upwards
    /// <https://bulma.io/documentation/columns/responsiveness/#mobile-columns>
    #[prop_or_default]
    pub desktop: bool,
}

/// The container for a set of responsive columns.
///
/// <https://bulma.io/documentation/columns/>
#[function_component(Columns)]
pub fn columns(
    ColumnsProps {
        children,
        classes,
        vcentered,
        multiline,
        centered,
        gapless,
        mobile,
        desktop,
    }: &ColumnsProps,
) -> Html {
    basic_comp!(
        <div>,
        children,
        classes.clone(),
        "columns",
        vcentered.then_some("is-vcentered"),
        multiline.then_some("is-multiline"),
        centered.then_some("is-centered"),
        gapless.then_some("is-gapless"),
        mobile.then_some("is-mobile"),
        desktop.then_some("is-desktop")
    )
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Limit width to content
    #[prop_or_default]
    pub narrow: bool,
    #[prop_or_default]
    pub size: Option<ColumnSizes>,
    #[prop_or_default]
    pub offset: Option<ColumnOffset>,
}

/// A flexbox-based responsive column.
///
/// <https://bulma.io/documentation/columns/>
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
#[function_component(Column)]
pub fn column(ColumnProps { children, classes, narrow, size, offset }: &ColumnProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "column", narrow.then_some("is-narrow"), size.as_ref(), offset)
}

/// Column size classes.
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum ColumnSize {
    #[strum(to_string = "is-full")]
    Full,

    // quarters
    #[strum(to_string = "is-one-quarter")]
    OneQuarter,
    #[strum(to_string = "is-half")]
    Half,
    #[strum(to_string = "is-three-quarters")]
    ThreeQuarters,

    // thirds
    #[strum(to_string = "is-one-third")]
    OneThird,
    #[strum(to_string = "is-two-thirds")]
    TwoThirds,

    // fifths
    #[strum(to_string = "is-one-fifth")]
    OneFifth,
    #[strum(to_string = "is-two-fifths")]
    TwoFifths,
    #[strum(to_string = "is-three-fifths")]
    ThreeFifths,
    #[strum(to_string = "is-four-fifths")]
    FourFifths,

    // 12 columns
    #[strum(to_string = "is-1")]
    One,
    #[strum(to_string = "is-2")]
    Two,
    #[strum(to_string = "is-3")]
    Three,
    #[strum(to_string = "is-4")]
    Four,
    #[strum(to_string = "is-5")]
    Five,
    #[strum(to_string = "is-6")]
    Six,
    #[strum(to_string = "is-7")]
    Seven,
    #[strum(to_string = "is-8")]
    Eight,
    #[strum(to_string = "is-9")]
    Nine,
    #[strum(to_string = "is-10")]
    Ten,
    #[strum(to_string = "is-11")]
    Eleven,
    #[strum(to_string = "is-12")]
    Twelve,
}

impl ImplicitClone for ColumnSize {}

impl_classes_from!(ColumnSize);

impl IntoPropValue<ColumnSizeBreakpoint> for ColumnSize {
    fn into_prop_value(self) -> ColumnSizeBreakpoint {
        self.into()
    }
}

impl IntoPropValue<Option<ColumnSizeBreakpoint>> for ColumnSize {
    fn into_prop_value(self) -> Option<ColumnSizeBreakpoint> {
        Some(self.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColumnSizeBreakpoint(pub ColumnSize, pub Option<Breakpoint>);

impl From<ColumnSizeBreakpoint> for Classes {
    fn from(size: ColumnSizeBreakpoint) -> Self {
        match size {
            ColumnSizeBreakpoint(size, None) => size.into(),
            ColumnSizeBreakpoint(size, Some(breakpoint)) => Classes::from(format!(
                "{}-{}",
                <_ as Into<&'static str>>::into(size),
                <_ as Into<&'static str>>::into(breakpoint)
            )),
        }
    }
}

impl From<ColumnSize> for ColumnSizeBreakpoint {
    fn from(value: ColumnSize) -> Self {
        ColumnSizeBreakpoint(value, None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnSizes(pub Vec<ColumnSizeBreakpoint>);

impl From<&ColumnSizes> for Classes {
    fn from(sizes: &ColumnSizes) -> Self {
        let mut classes = Classes::new();

        for size in &sizes.0 {
            classes.push(Classes::from(*size));
        }
        classes
    }
}

impl IntoPropValue<Option<ColumnSizes>> for ColumnSize {
    fn into_prop_value(self) -> Option<ColumnSizes> {
        Some(ColumnSizes(vec![self.into_prop_value()]))
    }
}

impl IntoPropValue<Option<ColumnSizes>> for ColumnSizeBreakpoint {
    fn into_prop_value(self) -> Option<ColumnSizes> {
        Some(ColumnSizes(vec![self]))
    }
}

// impl IntoPropValue<Option<ColumnSizes>> for &[ColumnSizeBreakpoint] {
//     fn into_prop_value(self) -> Option<ColumnSizes> {
//         Some(ColumnSizes(self.into().clopied().collect()))
//     }
// }

/// Column size classes.
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum ColumnOffset {
    // quarters
    #[strum(to_string = "is-offset-one-quarter")]
    OneQuarter,
    #[strum(to_string = "is-offset-half")]
    Half,
    #[strum(to_string = "is-offset-three-quarters")]
    ThreeQuarters,

    // thirds
    #[strum(to_string = "is-offset-one-third")]
    OneThird,
    #[strum(to_string = "is-offset-two-thirds")]
    TwoThirds,

    // fifths
    #[strum(to_string = "is-offset-one-fifth")]
    OneFifth,
    #[strum(to_string = "is-offset-two-fifths")]
    TwoFifths,
    #[strum(to_string = "is-offset-three-fifths")]
    ThreeFifths,
    #[strum(to_string = "is-offset-four-fifths")]
    FourFifths,

    // 12 columns
    #[strum(to_string = "is-offset-1")]
    One,
    #[strum(to_string = "is-offset-2")]
    Two,
    #[strum(to_string = "is-offset-3")]
    Three,
    #[strum(to_string = "is-offset-4")]
    Four,
    #[strum(to_string = "is-offset-5")]
    Five,
    #[strum(to_string = "is-offset-6")]
    Six,
    #[strum(to_string = "is-offset-7")]
    Seven,
    #[strum(to_string = "is-offset-8")]
    Eight,
    #[strum(to_string = "is-offset-9")]
    Nine,
    #[strum(to_string = "is-offset-10")]
    Ten,
    #[strum(to_string = "is-offset-11")]
    Eleven,
    #[strum(to_string = "is-offset-12")]
    Twelve,
}

impl ImplicitClone for ColumnOffset {}

impl_classes_from!(ColumnOffset);
