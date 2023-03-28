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
        $crate::macros::basic_comp!(<$tag []>, $children, $($classes),*)
    };
    (<@$tag:ident [$($attr:tt)*]>, $children:expr, $($classes:expr),*) => {
        html! {
            <@{$crate::macros::convert($tag)} $($attr)* class={::yew::classes!($($classes),*)}>
                {$children.clone()}
            </@>
        }
    };
    (<@$tag:ident>, $children:expr, $($classes:expr),*) => {
        $crate::macros::basic_comp!(<@$tag []>, $children, $($classes),*)
    };
}

pub(crate) use basic_comp;

pub(crate) fn convert(val: &AttrValue) -> Cow<'static, str> {
    match val {
        AttrValue::Static(s) => Cow::Borrowed(s),
        AttrValue::Rc(s) => Cow::Owned((**s).to_owned()),
    }
}
