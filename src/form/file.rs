#![allow(clippy::redundant_closure_call)]

use web_sys::{File as SysFile, HtmlInputElement};
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FileProps {
    /// The `name` attribute for this form element.
    pub name: AttrValue,
    /// The controlled form value for the currently selected files.
    pub files: Vec<SysFile>,
    /// The callback to be used for propagating changes to this form element.
    pub update: Callback<Vec<SysFile>>,

    /// The display text for the file selector.
    #[prop_or_else(|| "Choose a file...".into())]
    pub selector_label: AttrValue,
    /// The HTML contents to use for the file selector icon.
    #[prop_or_default]
    pub selector_icon: Html,

    #[prop_or_default]
    pub classes: Classes,
    /// An option to control if file names will be displayed; if a value is provided, then the
    /// `has-name` class will be added to this form element and the given value will be used as a
    /// placeholder until files are selected.
    #[prop_or_default]
    pub has_name: bool,
    /// Move the CTA element to the right side of the component.
    #[prop_or_default]
    pub right: bool,
    /// Expand the file display name to the full width of the parent.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Display as a boxed block.
    #[prop_or_default]
    pub boxed: bool,
    /// Allow multiple files to be selected.
    #[prop_or_default]
    pub multiple: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component within its parent.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A custom file upload input.
///
/// <https://bulma.io/documentation/form/file/>
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(File)]
pub fn file(
    FileProps {
        name,
        files,
        update,
        selector_label,
        selector_icon,
        classes,
        has_name,
        right,
        fullwidth,
        boxed,
        multiple,
        size,
        alignment,
    }: &FileProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "file",
        has_name.then_some("has-name"),
        right.then_some("is-right"),
        fullwidth.then_some("is-fullwidth"),
        boxed.then_some("is-boxed"),
        size,
        alignment
    );

    let onchange = update.reform(|e: Event| {
        let file = e.target_unchecked_into::<HtmlInputElement>();
        if let Some(list) = file.files() {
            (0..list.length())
                .into_iter()
                .filter_map(|idx| list.item(idx))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    html! {
        <div class={classes}>
            <label class="file-label">
                <input
                    type="file"
                    class="file-input"
                    {name}
                    multiple={*multiple}
                    {onchange}
                    />
                <span class="file-cta">
                    <span class="file-icon">
                        {selector_icon.clone()}
                    </span>
                    <span class="file-label">
                        {selector_label.clone()}
                    </span>
                </span>
                if *has_name {
                    { for files.iter().map(|file| html! {
                        <span classes="file-name">{file.name()}</span>
                    })}
                }
            </label>
        </div>
    }
}
