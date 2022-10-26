#![allow(clippy::redundant_closure_call)]

use std::borrow::Cow;

use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: Cow<'static, str>,
    /// The context modifier to use for this tile element, else none.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
#[function_component(Tile)]
pub fn tile(TileProps { children, classes, tag, ctx, vertical, size }: &TileProps) -> Html {
    let classes = classes!(classes.clone(), "tile", ctx, size, vertical.then_some("is-vertical"));

    html! {
        <@{tag.clone()} class={classes}>
            {children.clone()}
        </@>
    }
}

/// Tile context modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileCtx {
    #[display(fmt = "ancestor")]
    Ancestor,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "child")]
    Child,
}

impl From<TileCtx> for Classes {
    fn from(ctx: TileCtx) -> Self {
        Classes::from(ctx.to_string())
    }
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileSize {
    #[display(fmt = "1")]
    One,
    #[display(fmt = "2")]
    Two,
    #[display(fmt = "3")]
    Three,
    #[display(fmt = "4")]
    Four,
    #[display(fmt = "5")]
    Five,
    #[display(fmt = "6")]
    Six,
    #[display(fmt = "7")]
    Seven,
    #[display(fmt = "8")]
    Eight,
    #[display(fmt = "9")]
    Nine,
    #[display(fmt = "10")]
    Ten,
    #[display(fmt = "11")]
    Eleven,
    #[display(fmt = "12")]
    Twelve,
}

impl From<TileSize> for Classes {
    fn from(size: TileSize) -> Self {
        Classes::from(size.to_string())
    }
}
