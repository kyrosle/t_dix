mod controls;
mod test;
mod video;

use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

use dioxus::{core::IntoVNode, prelude::*, prelude::*};
pub use video::Video;

use crate::AppState;

#[derive(Clone, PartialEq)]
pub struct Title(String);
impl Title {
    fn changed(&mut self, title: &str) {
        self.0 = title.to_string();
    }
}
impl Deref for Title {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static TITLE: Atom<String> = |_| "feafea".to_string();

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    // let val = use_future(&cx, (), |_| async { AppState::new().await });
    // let state = match val.value() {
    //     None => return cx.render(rsx! {"Loading"}),
    //     Some(state) => state,
    // };
    // cx.provide_context(state.to_owned());
    // cx.provide_context(Context::new("hhh".to_string()));

    // let binding = cx.consume_context::<Context<Title>>().unwrap().inner();
    // let show_title = binding.borrow();
    // let mut title = binding.borrow_mut();

    // let title = use_atom(&cx, TITLE);

    cx.render(rsx! {
        div {
            rsx!{
                p {
                    // "{title}"
                }
                button {
                    onclick: move |_| {
                        // title.set("wwewe".to_string());
                    },
                    "Refresh"
                }
            }
            div{
                class: "container",
                // Video{}
            }
        }
    })
}

/// warp for type T
/// using for reference with mutable changed
pub struct Context<T>(Rc<RefCell<T>>)
where
    T: PartialEq;
impl<T> Context<T>
where
    T: PartialEq,
{
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
    pub fn inner(self) -> Rc<RefCell<T>> {
        self.0
    }
    pub fn inner_mut(&mut self) -> &mut Rc<RefCell<T>> {
        &mut self.0
    }
    pub fn get_value(&self) -> Ref<'_, T> {
        self.0.borrow()
    }
    pub fn change_value(&self, value: T) {
        (*self.0.borrow_mut()) = value;
    }
}
impl<T> Clone for Context<T>
where
    T: PartialEq,
{
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<T> PartialEq for Context<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.get_value() == *other.get_value()
    }
}
