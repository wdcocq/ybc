#![allow(clippy::redundant_closure_call)]

use web_sys::{File as SysFile, HtmlInputElement};
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FileProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled form value for the currently selected files.
    pub files: Vec<SysFile>,
    /// The callback to be used for propagating changes to this form element.
    pub update: Callback<Vec<SysFile>>,

    /// The display text for the file selector.
    #[prop_or_else(|| "Choose a file...".into())]
    pub selector_label: String,
    /// The HTML contents to use for the file selector icon.
    #[prop_or_default]
    pub selector_icon: Html,

    #[prop_or_default]
    pub classes: Classes,
    /// An option to control if file names will be displayed; if a value is provided, then the
    /// `has-name` class will be added to this form element and the given value will be used as a
    /// placeholder until files are selected.
    #[prop_or_default]
    pub has_name: Option<String>,
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
/// [https://bulma.io/documentation/form/file/](https://bulma.io/documentation/form/file/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct File;

impl Component for File {
    type Message = Vec<SysFile>;
    type Properties = FileProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let mut classes = Classes::from("file");
        classes.push(props.classes.clone());
        if props.has_name.is_some() {
            classes.push("has-name");
        }
        if props.right {
            classes.push("is-right");
        }
        if props.fullwidth {
            classes.push("is-fullwidth");
        }
        if props.boxed {
            classes.push("is-boxed");
        }
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &props.alignment {
            classes.push(&alignment.to_string());
        }
        let filenames = props
            .files
            .iter()
            .map(|file| html! {<span class="file-name">{file.name()}</span>})
            .collect::<Vec<_>>();
        html! {
            <div class={classes}>
                <label class="file-label">
                    <input
                        type="file"
                        class="file-input"
                        name={props.name.clone()}
                        multiple={props.multiple}
                        onchange={link.callback(|e: Event| {
                            let file = e.target_unchecked_into::<HtmlInputElement>();
                            if let Some(list) = file.files() {
                                (0..list.length()).into_iter()
                                    .filter_map(|idx| list.item(idx))
                                    .collect::<Vec<_>>()
                            } else {
                                vec![]
                            }
                        })}
                        />
                    <span class="file-cta">
                        <span class="file-icon">
                            {props.selector_icon.clone()}
                        </span>
                        <span class="file-label">
                            {props.selector_label.clone()}
                        </span>
                    </span>
                    {filenames}
                </label>
            </div>
        }
    }
}
