//! A [Yew](https://yew.rs) component library based on the [Bulma CSS](https://bulma.io) framework.
//!
//! YBC encapsulates all of the structure, style and functionality of the Bulma CSS framework as a
//! set of Yew components. YBC also ships with support for the Yew Router, adding Bulma-styled
//! components which wrap the Yew Router components for clean integration.
//!
//! As a guiding principal, YBC does not attempt to encapsulate every single Bulma style as a Rust
//! type, let alone the many valid style combinations. That would be far too complex, and probably
//! limiting to the user in many ways. Instead, YBC handles strucutre, required classes,
//! functionality, sane defaults and every component can be customized with any additional classes
//! for an exact look and feel.
//!
//! What does it look like to use YBC? The following is a snippet of a component's `view` method
//! rendering a navbar, a fluid container, and some tiles.
//!
//! Please see [this project's README](https://github.com/thedodd/ybc) for the example. **Docs.rs is
//! currently (2021.07.26) slightly broken and having trouble including external docs as it has
//! historically.**
// TODO: add this back in once it is no longer a land mine:
//  #![cfg_attr(feature = "docinclude", doc = include_str!("../README.md"))]
#![recursion_limit = "1024"]

#[macro_use]
mod macros;

mod columns;
mod common;
mod components;
mod elements;
mod form;
mod layout;

// columns
pub use columns::*;

// common
pub use common::*;

// components
pub use components::breadcrumb::*;
pub use components::card::*;
pub use components::dropdown::*;
pub use components::menu::*;
pub use components::message::*;
pub use components::modal::*;
pub use components::navbar::*;
pub use components::pagination::*;
pub use components::panel::*;
pub use components::tabs::*;

// elements
pub use elements::block::*;
pub use elements::button::*;
pub use elements::content::*;
pub use elements::delete::*;
pub use elements::icon::*;
pub use elements::image::*;
pub use elements::notification::*;
pub use elements::progress::*;
pub use elements::r#box::*;
pub use elements::table::*;
pub use elements::tag::*;
pub use elements::title::*;

// form
pub use form::checkbox::*;
pub use form::control::*;
pub use form::field::*;
pub use form::file::*;
pub use form::input::*;
pub use form::label::*;
pub use form::radio::*;
pub use form::select::*;
pub use form::textarea::*;

// layout
pub use layout::container::*;
pub use layout::footer::*;
pub use layout::hero::*;
pub use layout::level::*;
pub use layout::media::*;
pub use layout::section::*;
pub use layout::tile::*;
