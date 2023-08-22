use std::borrow::Cow;

use yew::AttrValue;

macro_rules! impl_classes_from (
    ($($t:ty),+) => {
        $(
            impl From<&$t> for ::yew::html::Classes {
                fn from(val: &$t) -> ::yew::html::Classes{
                    <::yew::html::Classes as From<&'static str>>::from(val.into())
                }
            }

            impl From<$t> for ::yew::html::Classes {
                fn from(val: $t) -> ::yew::html::Classes {
                    ::yew::html::Classes::from(&val)
                }
            }

            impl ::yew::html::IntoPropValue<::yew::html::Classes> for $t {
                fn into_prop_value(self) -> ::yew::html::Classes {
                   self.into()
                }
            }
        )*
    }
);

macro_rules! basic_comp {
    (<$tag:tt [$($attr:tt)*]>, $children:expr, $($classes:expr),*) => {
        html! {
            <$tag $($attr)* class={::yew::classes!($($classes),*)}>
                {$children.clone()}
            </$tag>
        }
    };
    (<$tag:tt>, $children:expr, $($classes:expr),*) => {
        basic_comp!(<$tag []>, $children, $($classes),*)
    };
    (<@$tag:ident [$($attr:tt)*]>, $children:expr, $($classes:expr),*) => {
        html! {
            <@{$crate::macros::convert($tag)} $($attr)* class={::yew::classes!($($classes),*)}>
                {$children.clone()}
            </@>
        }
    };
    (<@$tag:ident>, $children:expr, $($classes:expr),*) => {
        basic_comp!(<@$tag []>, $children, $($classes),*)
    };
}

pub(crate) fn convert(val: &AttrValue) -> Cow<'static, str> {
    match val {
        AttrValue::Static(s) => Cow::Borrowed(s),
        AttrValue::Rc(s) => Cow::Owned((**s).to_owned()),
    }
}

#[macro_export]
macro_rules! properties_with_globals {
    ($(#[$attributes:meta])* $visibility:vis struct $name:ident { $($tokens:tt)+ }) => {
        $(#[$attributes])*
        $visibility struct $name {
            $($tokens)+

            /// Global attribute [tabindex](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/tabindex/)
            #[prop_or_default]
            pub tabindex: ::std::option::Option<::yew::AttrValue>,
            /// Global attribute [hidden](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden/)
            #[prop_or_default]
            pub hidden: ::std::primitive::bool,
        }
    };
}
