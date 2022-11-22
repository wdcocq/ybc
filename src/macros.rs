#[macro_export]
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

#[macro_export]
macro_rules! basic_comp {
    (<$tag:tt [$($attr:tt)*]>, $children:expr, $($classes:expr),*) => {
        html! {
            <$tag $($attr)* class={::yew::classes!($($classes),*)}>
                {$children.clone()}
            </$tag>
        }
    };
    (<$tag:tt>, $children:expr, $($classes:expr),*) => {
        $crate::basic_comp!(<$tag []>, $children, $($classes),*)
    };
    (<@$tag:ident [$($attr:tt)*]>, $children:expr, $($classes:expr),*) => {
        html! {
            <@{$tag.clone()} $($attr)* class={::yew::classes!($($classes),*)}>
                {$children.clone()}
            </@>
        }
    };
    (<@$tag:ident>, $children:expr, $($classes:expr),*) => {
        $crate::basic_comp!(<@$tag []>, $children, $($classes),*)
    };
}

// pub(crate) fn convert(val: &AttrValue) -> Cow<'static, str> {
//     match val {
//         AttrValue::Static(s) => Cow::Borrowed(s),
//         AttrValue::Rc(s) => Cow::Owned((**s).to_owned()),
//     }
// }
