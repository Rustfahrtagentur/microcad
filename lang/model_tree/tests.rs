// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree tests

#[cfg(test)]
use crate::{model_tree::*, syntax::*};

#[cfg(test)]
fn sample_models() -> Models {
    fn obj(id: &str) -> Model {
        let model = ModelBuilder::new_object_body().build();
        model.borrow_mut().id = Some(Identifier::no_ref(id));
        model
    }

    let models = vec![
        vec![obj("a0"), obj("a1")].into(),
        vec![obj("b0")].into(),
        vec![obj("c0"), obj("c1"), obj("c2")].into(),
        vec![obj("d0")].into(),
    ];

    // This should result in following model multiplicity:
    // a0
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    // a1
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    Models::from_stack(&models)
}

#[cfg(test)]
fn sample_tree() -> Model {
    ModelBuilder::new_object_body()
        .add_children(sample_models())
        .expect("No error")
        .build()
}

#[test]
fn model_nest() {
    let models = sample_models();
    assert_eq!(models.len(), 2, "Must contain a0 and a1 as root");

    let a0 = models.first().expect("a0");
    let a1 = models.last().expect("a1");
    assert_eq!(a0.borrow().id, Some(Identifier::no_ref("a0")));
    assert_eq!(a1.borrow().id, Some(Identifier::no_ref("a1")));

    let a0_ = a0.borrow();
    assert_eq!(a0_.children().count(), 1); // Contains b0
    assert_eq!(
        a0_.children().next().expect("b0").borrow().id,
        Some(Identifier::no_ref("b0"))
    );

    log::trace!("Models:\n{models}");
}

#[test]
fn model_iter_descendants() {
    let model = sample_tree();

    for model in model.descendants() {
        let depth = model.depth() * 2;
        log::info!("{:depth$}{signature}", "", signature = model.signature());
    }
}
