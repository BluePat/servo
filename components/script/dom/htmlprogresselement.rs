/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::HTMLProgressElementBinding::HTMLProgressElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::num::Finite;
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::document::Document;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::node::Node;
use crate::dom::nodelist::NodeList;
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use std::cell::Cell;

#[dom_struct]
pub struct HTMLProgressElement {
    htmlelement: HTMLElement,
    labels_node_list: MutNullableDom<NodeList>,
    max: Cell<Finite<f64>>,
    value: Cell<Option<Finite<f64>>>,
}

impl HTMLProgressElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLProgressElement {
        HTMLProgressElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
            labels_node_list: MutNullableDom::new(None),
            max: Cell::new(Finite::wrap(1.0)),
            value: Cell::new(None),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> DomRoot<HTMLProgressElement> {
        Node::reflect_node(
            Box::new(HTMLProgressElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
        )
    }
}

impl HTMLProgressElementMethods for HTMLProgressElement {
    // https://html.spec.whatwg.org/multipage/#dom-lfe-labels
    make_labels_getter!(Labels, labels_node_list);

    // https://html.spec.whatwg.org/multipage/#attr-progress-value
    fn Value(&self) -> Finite<f64> {
        let value = self.value.get().unwrap_or(Finite::wrap(0.0));
        if *value > *self.max.get() {
            self.max.get()
        } else {
            value
        }
    }

    // https://html.spec.whatwg.org/multipage/#attr-progress-value
    fn SetValue(&self, new_val: Finite<f64>) {
        if 0.0 <= *new_val {
            self.value.set(Some(new_val))
        }
    }

    // https://html.spec.whatwg.org/multipage/#attr-progress-max
    fn Max(&self) -> Finite<f64> {
        self.max.get()
    }

    // https://html.spec.whatwg.org/multipage/#attr-progress-max
    fn SetMax(&self, new_val: Finite<f64>) {
        if *new_val > 0.0 {
            self.max.replace(new_val);
        }
    }

    // https://html.spec.whatwg.org/multipage/#dom-progress-position
    fn Position(&self) -> Finite<f64> {
        match self.value.get() {
            Some(v) => {
                let current_value = if *v > *self.max.get() {
                    *self.max.get()
                } else {
                    *v
                };
                let ret = current_value / *self.max.get();
                // An unsafe Finite constructor might be nice here, as it's unlikely for the
                // compiler to infer the following guarantees. It is probably premature
                // optimization though.
                //
                // Safety: `ret` have to be a finite, defined number. This is the case since both
                // value and max is finite, max > 0, and a current_value >> max cannot exist as
                // current_value <= max
                Finite::wrap(ret)
            },
            None => Finite::wrap(-1.0),
        }
    }
}
