use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Option<Html>,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(Modal)]
pub fn modal(ModalProps { children, classes, id, trigger }: &ModalProps) -> Html {
    let modal_agent = use_context::<ModalAgent>().expect("Failed to get ModalAgent");
    let opencb = use_callback(
        {
            let modal_agent = modal_agent.clone();
            move |_, id: &String| {
                modal_agent.open(id.clone());
            }
        },
        id.clone(),
    );
    let closecb = use_callback(
        {
            let modal_agent = modal_agent.clone();
            move |_, id: &String| {
                modal_agent.close(id.clone());
            }
        },
        id.clone(),
    );

    let is_active = modal_agent.is_active(id);
    let classes = classes!("modal", classes, is_active.then_some("is-active"));

    html! {
        <>
            if let Some(trigger) = trigger {
                <div onclick={opencb}>
                    {trigger.clone()}
                </div>
            }
            <div id={id.clone()} class={classes}>
                <div class="modal-background" onclick={closecb.clone()}></div>
                <div class="modal-content">
                    {children.clone()}
                </div>
                <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
            </div>
        </>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `title` prop.
    #[prop_or_default]
    pub children: Children,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Option<Html>,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(ModalCard)]
pub fn modal_card(ModalCardProps { classes, id, title, children, footer, trigger }: &ModalCardProps) -> Html {
    let modal_agent = use_context::<ModalAgent>().expect("Failed to get ModalAgent");
    let opencb = use_callback(
        {
            let modal_agent = modal_agent.clone();
            move |_, id: &String| {
                modal_agent.open(id.clone());
            }
        },
        id.clone(),
    );
    let closecb = use_callback(
        {
            let modal_agent = modal_agent.clone();
            move |_, id: &String| {
                modal_agent.close(id.clone());
            }
        },
        id.clone(),
    );

    let is_active = modal_agent.is_active(id);
    let classes = classes!("modal", classes, is_active.then_some("is-active"));

    html! {
        <>
            if let Some(trigger) = trigger {
                <div onclick={opencb}>
                    {trigger.clone()}
                </div>
            }
            <div id={id.clone()} class={classes}>
                <div class="modal-background" onclick={closecb.clone()}></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{title.clone()}</p>
                        <button class="delete" aria-label="close" onclick={closecb.clone()}></button>
                    </header>
                    <section class="modal-card-body">
                        {children.clone()}
                    </section>
                    <footer class="modal-card-foot">
                        {footer.clone()}
                    </footer>
                </div>
                <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
            </div>
        </>
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ModalAgentContext {
    modals: RefCell<HashMap<Cow<'static, str>, bool>>,
}

#[derive(Clone, Debug, PartialEq)]
enum ModalAgentContextMsg {
    Open(Cow<'static, str>),
    Close(Cow<'static, str>),
}

impl Reducible for ModalAgentContext {
    type Action = ModalAgentContextMsg;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let (id, state) = match action {
            ModalAgentContextMsg::Open(id) => (id, true),
            ModalAgentContextMsg::Close(id) => (id, false),
        };

        Rc::make_mut(&mut self)
            .modals
            .borrow_mut()
            .entry(id)
            .and_modify(|v| *v = state)
            .or_insert(state);

        self
    }
}

// // //////////////////////////////////////////////////////////////////////////////
// // //////////////////////////////////////////////////////////////////////////////

/// An agent used for being able to open and close `Modal` & `ModalCard` instances by ID.
/// Requires a `ModalAgentProvider` to be present
#[derive(Clone, Debug, PartialEq)]
pub struct ModalAgent(UseReducerHandle<ModalAgentContext>);

impl ModalAgent {
    pub fn is_active<T>(&self, id: T) -> bool
    where
        T: std::hash::Hash + Eq + AsRef<str>,
    {
        (*self.0).modals.borrow().get(id.as_ref()).copied().unwrap_or_default()
    }

    pub fn open<T>(&self, id: T)
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.dispatch(ModalAgentContextMsg::Open(id.into()));
    }

    pub fn close<T>(&self, id: T)
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.dispatch(ModalAgentContextMsg::Close(id.into()));
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ModalAgentProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// A provider for `ModalAgent`
#[function_component(ModalAgentProvider)]
pub fn modal_agent_provider(ModalAgentProviderProps { children }: &ModalAgentProviderProps) -> Html {
    let modal_agent_context = use_reducer_eq(ModalAgentContext::default);

    html! {
        <ContextProvider<ModalAgent> context={ModalAgent(modal_agent_context)}>
            {children.clone()}
        </ContextProvider<ModalAgent>>
    }
}
