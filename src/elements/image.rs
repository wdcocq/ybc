use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,
}

/// A container for responsive images.
///
/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
#[function_component(Image)]
pub fn image(ImageProps { children, classes, size }: &ImageProps) -> Html {
    basic_comp!(<figure>, children, classes.clone(), "image", size)
}

/// Available placeholder sizes for figures.
///
/// https://bulma.io/documentation/elements/image/
#[derive(Clone, Debug, IntoStaticStr, PartialEq)]
pub enum ImageSize {
    #[strum(to_string = "is-16x16")]
    Is16x16,
    #[strum(to_string = "is-24x24")]
    Is24x24,
    #[strum(to_string = "is-32x32")]
    Is32x32,
    #[strum(to_string = "is-48x48")]
    Is48x48,
    #[strum(to_string = "is-64x64")]
    Is64x64,
    #[strum(to_string = "is-96x96")]
    Is96x96,
    #[strum(to_string = "is-128x128")]
    Is128x128,
    #[strum(to_string = "is-Square")]
    IsSquare,
    #[strum(to_string = "is-1by1")]
    Is1by1,
    #[strum(to_string = "is-5by4")]
    Is5by4,
    #[strum(to_string = "is-4by3")]
    Is4by3,
    #[strum(to_string = "is-3by2")]
    Is3by2,
    #[strum(to_string = "is-5by3")]
    Is5by3,
    #[strum(to_string = "is-16by9")]
    Is16by9,
    #[strum(to_string = "is-2by1")]
    Is2by1,
    #[strum(to_string = "is-3by1")]
    Is3by1,
    #[strum(to_string = "is-4by5")]
    Is4by5,
    #[strum(to_string = "is-3by4")]
    Is3by4,
    #[strum(to_string = "is-2by3")]
    Is2by3,
    #[strum(to_string = "is-3by5")]
    Is3by5,
    #[strum(to_string = "is-9by16")]
    Is9by16,
    #[strum(to_string = "is-1by2")]
    Is1by2,
    #[strum(to_string = "is-1by3")]
    Is1by3,
}

impl_classes_from!(ImageSize);
