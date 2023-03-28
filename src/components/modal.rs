use std::{cell::RefCell, collections::HashMap, rc::Rc};

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: AttrValue,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Option<Html>,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on [`ModalAgent`] to be able to open and close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(Modal)]
pub fn modal(ModalProps { children, classes, id, trigger }: &ModalProps) -> Html {
    let modal_agent = use_modal_agent();
    let active = use_state(|| false);

    use_effect_with_deps(
        {
            let active = active.setter();
            move |id: &AttrValue| {
                if let Some(modal_agent) = &modal_agent {
                    modal_agent.register(id.clone(), move |s| active.set(s));
                }

                let id = id.clone();
                move || {
                    if let Some(modal_agent) = modal_agent {
                        modal_agent.unregister(&id);
                    }
                }
            }
        },
        id.clone(),
    );

    let opencb = {
        let active = active.setter();
        move |_| active.set(true)
    };
    let closecb = {
        let active = active.setter();
        move |_| active.set(false)
    };

    let classes = classes!("modal", classes.clone(), active.then_some("is-active"));

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
    pub id: AttrValue,
    /// The title of this modal.
    pub title: AttrValue,
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
    pub classes: Classes,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on [`ModalAgent`] to be able to open and close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(ModalCard)]
pub fn modal_card(ModalCardProps { classes, id, title, children, footer, trigger }: &ModalCardProps) -> Html {
    let modal_agent = use_modal_agent();
    let active = use_state(|| false);

    use_effect_with_deps(
        {
            let active = active.setter();
            move |id: &AttrValue| {
                if let Some(modal_agent) = &modal_agent {
                    modal_agent.register(id.clone(), move |s| active.set(s));
                }

                let id = id.clone();
                move || {
                    if let Some(modal_agent) = modal_agent {
                        modal_agent.unregister(&id);
                    }
                }
            }
        },
        id.clone(),
    );

    let opencb = {
        let active = active.setter();
        move |_| active.set(true)
    };
    let closecb = {
        let active = active.setter();
        move |_| active.set(false)
    };

    let classes = classes!("modal", classes.clone(), active.then_some("is-active"));

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

//////////////////////////////////////////////////////////////////////////////////

/// An agent used to open and close [`Modal`] & [`ModalCard`] instances by ID.
/// Requires a [`ModalAgentProvider`] to be present.
/// Can be retrieved with [`use_modal_agent()`]
#[derive(Clone, Debug, PartialEq)]
pub struct ModalAgent {
    callbacks: Rc<RefCell<HashMap<AttrValue, Callback<bool>>>>,
}

impl ModalAgent {
    pub fn register<T, F>(&self, id: T, callback: F)
    where
        T: Into<AttrValue>,
        F: Into<Callback<bool>>,
    {
        self.callbacks.borrow_mut().insert(id.into(), callback.into());
    }

    pub fn unregister<T: ?Sized>(&self, id: &T)
    where
        T: AsRef<str>,
    {
        self.callbacks.borrow_mut().remove(id.as_ref());
    }

    pub fn update<T: ?Sized>(&self, id: &T, active: bool)
    where
        T: AsRef<str>,
    {
        if let Some(callback) = self.callbacks.borrow().get(id.as_ref()) {
            callback.emit(active);
        }
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ModalAgentProviderProps {
    pub children: Children,
}

/// A provider for [`ModalAgent`]
#[function_component(ModalAgentProvider)]
pub fn modal_agent_provider(ModalAgentProviderProps { children }: &ModalAgentProviderProps) -> Html {
    let modal_agent = ModalAgent { callbacks: use_mut_ref(Default::default) };

    html! {
        <ContextProvider<ModalAgent> context={modal_agent}>
            {children.clone()}
        </ContextProvider<ModalAgent>>
    }
}

/// Hook to retrieve a [`ModalAgent`].
/// Returns `None` if no [`ModalAgentProvider`] is present.
#[hook]
pub fn use_modal_agent() -> Option<ModalAgent> {
    use_context::<ModalAgent>()
}
