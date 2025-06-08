// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree tests

#[cfg(test)]
use crate::{model_tree::*, src_ref::SrcRef, syntax::*};

#[cfg(test)]
fn sample_nodes() -> ModelNodes {
    fn obj(id: &str) -> ModelNode {
        ModelNode::new_empty_object(SrcRef(None)).set_id(Identifier::no_ref(id))
    }

    let nodes = vec![
        vec![obj("a0"), obj("a1")].into(),
        vec![obj("b0")].into(),
        vec![obj("c0"), obj("c1"), obj("c2")].into(),
        vec![obj("d0")].into(),
    ];

    // This should result in following node multiplicity:
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
    ModelNodes::from_node_stack(&nodes)
}

#[cfg(test)]
fn sample_tree() -> ModelNode {
    ModelNode::new_empty_object(SrcRef(None)).append_children(sample_nodes())
}

#[test]
fn model_nodes_nest() {
    let nodes = sample_nodes();
    assert_eq!(nodes.len(), 2, "Must contain a0 and a1 as root");

    let a0 = nodes.first().expect("a0");
    let a1 = nodes.last().expect("a1");
    assert_eq!(a0.id().expect("a0"), Identifier::no_ref("a0"));
    assert_eq!(a1.id().expect("a1"), Identifier::no_ref("a1"));

    assert_eq!(a0.children().count(), 1); // Contains b0
    assert_eq!(
        a0.children().next().expect("b0").id().expect("b0"),
        Identifier::no_ref("b0")
    );

    log::trace!("Nodes:\n{nodes}");
}

#[test]
fn model_node_iter_descendants() {
    let node = sample_tree();

    for node in node.descendants() {
        let depth = node.depth() * 2;
        log::info!("{:depth$}{signature}", "", signature = node.signature());
    }
}
