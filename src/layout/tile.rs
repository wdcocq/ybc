#![allow(clippy::redundant_closure_call)]

use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,
    /// The context modifier to use for this tile element, else none.
    ///
    /// <https://bulma.io/documentation/layout/tiles/#modifiers>
    #[prop_or_default]
    pub ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// <https://bulma.io/documentation/layout/tiles/#modifiers>
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
    /// <https://bulma.io/documentation/layout/tiles/#modifiers>
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// <https://bulma.io/documentation/layout/tiles/>
#[function_component(Tile)]
pub fn tile(TileProps { children, classes, tag, ctx, vertical, size }: &TileProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "tile", ctx, size, vertical.then_some("is-vertical"))
}

/// Tile context modifiers.
///
/// <https://bulma.io/documentation/layout/tiles/#modifiers>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum TileCtx {
    #[strum(to_string = "is-ancestor")]
    Ancestor,
    #[strum(to_string = "is-parent")]
    Parent,
    #[strum(to_string = "is-child")]
    Child,
}

/// Tile size modifiers.
///
/// <https://bulma.io/documentation/layout/tiles/#modifiers>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum TileSize {
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

impl_classes_from!(TileSize, TileCtx);
