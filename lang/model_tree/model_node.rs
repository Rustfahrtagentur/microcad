// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{
    GetPropertyValue, eval::*, model_tree::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*,
};

use microcad_core::*;

/// The origin is the [`Symbol`] and [`ArgumentMap`] from which the node has been created.
#[derive(Clone, Default, Debug)]
pub struct ModelNodeOrigin {
    /// The original symbol that has been called.
    creator: Option<Symbol>,

    /// The original arguments.
    arguments: ArgumentMap,

    /// The original source file.
    source_file: Option<std::rc::Rc<SourceFile>>,

    /// Source code reference of the call.
    call_src_ref: SrcRef,
}

impl std::fmt::Display for ModelNodeOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.creator {
            Some(creator) => {
                write!(
                    f,
                    "{symbol}({arguments})",
                    symbol = creator.full_name(),
                    arguments = self.arguments.to_one_line_string(Some(32))
                )
            }
            None => Ok(()),
        }
    }
}

/// The actual node contents
#[derive(custom_debug::Debug, Default)]
pub struct ModelNodeInner {
    /// Optional id.
    ///
    /// The id is set when the model node was created by an assignment: `a = cube(50mm)`.
    id: Option<Identifier>,
    /// Parent object.
    #[debug(skip)]
    parent: Option<ModelNode>,
    // Children of the model node.
    children: ModelNodes,
    /// Element of the node with [SrcRef].
    element: Refer<Element>,
    /// Attributes used for export.
    attributes: Attributes,
    /// The symbol (e.g. [`WorkbenchDefinition`]) that created this [`ModelNode`].
    origin: ModelNodeOrigin,
    /// The output type of the this node.
    output: ModelNodeOutput,
}

impl ModelNodeInner {
    /// Create a new [`ModelNodeInner`] with a specific element.
    pub fn new(element: Refer<Element>) -> Self {
        Self {
            element,
            ..Default::default()
        }
    }

    /// Return reference to the children of this node.
    pub fn children(&self) -> &ModelNodes {
        &self.children
    }

    /// Return element of this node.
    pub fn element(&self) -> &Element {
        &self.element
    }

    /// Return a mutable reference of the element of this node.
    pub fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }

    /// Return reference to the attributes of this node.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Return mutable reference for the attributes of this node
    pub fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    /// Set attribute for this node.
    pub fn set_attributes(&mut self, attributes: Attributes) {
        self.attributes = attributes;
    }

    /// Return reference to the attributes of this node.
    pub fn output(&self) -> &ModelNodeOutput {
        &self.output
    }

    /// Return mutable reference for the attributes of this node
    pub fn output_mut(&mut self) -> &mut ModelNodeOutput {
        &mut self.output
    }

    /// Set output for this node.
    pub fn set_output(&mut self, output: ModelNodeOutput) {
        self.output = output;
    }

    /// Return a reference to the model node origin.
    pub fn origin(&self) -> &ModelNodeOrigin {
        &self.origin
    }

    /// Clone only the content of this node without children and parent.
    pub fn clone_content(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: None,
            element: self.element.clone(),
            attributes: self.attributes.clone(),
            origin: self.origin.clone(),
            output: self.output.clone(),
            ..Default::default()
        }
    }
}

/// A reference counted, mutable [`ModelNode`].
#[derive(Debug, Clone)]
pub struct ModelNode(RcMut<ModelNodeInner>);

impl ModelNode {
    /// Create new model node from inner.
    pub fn new(inner: ModelNodeInner) -> Self {
        Self(RcMut::new(inner))
    }

    /// Return id of this object node.
    pub fn id(&self) -> Option<Identifier> {
        self.0.borrow().id.clone()
    }

    /// Set new id for this node.
    pub fn set_id(&mut self, id: Identifier) -> Self {
        self.0.borrow_mut().id = Some(id);
        self.clone()
    }

    /// Return the [`ModelNodeOrigin`] that created this node.
    pub fn origin(&self) -> ModelNodeOrigin {
        self.borrow().origin.clone()
    }

    /// Return output type.
    pub fn output_type(&self) -> ModelNodeOutputType {
        self.borrow().output.model_node_output_type()
    }

    /// Get borrowed reference to the inner of this node.
    pub fn borrow(&'_ self) -> std::cell::Ref<'_, ModelNodeInner> {
        self.0.borrow()
    }

    /// Get mutable borrowed reference to the inner of this node.
    pub fn borrow_mut(&'_ self) -> std::cell::RefMut<'_, ModelNodeInner> {
        self.0.borrow_mut()
    }
    /// Calculate Depth of the node.
    pub fn depth(&self) -> usize {
        self.parents().count()
    }

    /// Make a deep copy if this node.
    pub fn make_deep_copy(&self) -> Self {
        let copy = Self(RcMut::new(self.0.borrow().clone_content()));
        for child in self.children() {
            copy.append(child.make_deep_copy());
        }
        copy
    }

    /// Return address of this node.
    pub fn addr(&self) -> usize {
        self.0.as_ptr().addr()
    }

    /// Check if `other` is and `self` have the same address.
    pub fn is_same_as(&self, other: &ModelNode) -> bool {
        self.addr() == other.addr()
    }

    /// Remove child from this node.
    pub fn remove_child(&self, child: &ModelNode) {
        let mut s = self.0.borrow_mut();
        s.children.retain(|node| !node.is_same_as(child));
    }

    /// Detaches a node from its parent. Children are not affected.
    pub fn detach(&self) {
        match self.0.borrow_mut().parent {
            Some(ref mut parent) => {
                parent.remove_child(self);
            }
            None => return,
        }

        self.0.borrow_mut().parent = None;
    }

    /// Set parent of this node.
    pub fn set_parent(&mut self, parent: ModelNode) {
        self.0.borrow_mut().parent = Some(parent);
    }

    /// Append a single node as child.
    ///
    /// Also tries to set the output type if it has not been determined yet.
    pub fn append(&self, node: ModelNode) -> ModelNode {
        let mut node = node;
        node.set_parent(self.clone());

        let mut b = self.0.borrow_mut();
        // If this node's output type has not been determined, try to get it from child node
        if b.output.model_node_output_type() == ModelNodeOutputType::NotDetermined {
            b.set_output(ModelNodeOutput::new(node.output_type()));
        }
        b.children.push(node.clone());

        node
    }

    /// Append multiple nodes as children.
    ///
    /// Return self.
    pub fn append_children(&self, nodes: ModelNodes) -> Self {
        for node in nodes.iter() {
            self.append(node.clone());
        }
        self.clone()
    }

    /// Short cut to generate boolean operator as binary operation with two nodes.
    pub fn binary_op(self, op: BooleanOp, other: ModelNode) -> ModelNode {
        assert!(self != other, "lhs and rhs must be distinct.");

        let container = ModelNodeBuilder::new_object_body()
            .add_children(vec![self.clone(), other].into())
            .expect("No error")
            .build();

        ModelNodeBuilder::new_operation(op, SrcRef(None))
            .add_children(vec![container].into())
            .expect("No error")
            .build()
    }

    /// Find children node placeholder in node descendants.
    pub fn find_children_placeholder(&self) -> Option<ModelNode> {
        self.descendants().find(|n| {
            n.id().is_none() && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
        })
    }

    /// Find the original source file of this node
    pub fn find_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.ancestors().find_map(|node| {
            let b = node.borrow();
            let origin = b.origin();
            origin.source_file.clone()
        })
    }

    /// Set the transformation matrices for this node and its children.
    pub fn set_matrix(&self, mat: Mat4) {
        let new_mat = {
            let mut b = self.borrow_mut();
            let new_mat = match b.element() {
                Element::Transform(affine_transform) => mat * affine_transform.mat3d(),
                _ => mat,
            };
            b.output_mut().matrix = new_mat;
            new_mat
        };

        self.children().for_each(|node| {
            node.set_matrix(new_mat);
        });
    }

    /// Set the resolution for this node.
    pub fn set_resolution(&self, resolution: RenderResolution) {
        let new_resolution = {
            let mut b = self.borrow_mut();
            let new_resolution = resolution * b.output().matrix;
            b.output_mut().resolution = new_resolution.clone();
            new_resolution
        };

        self.children().for_each(|node| {
            node.set_resolution(new_resolution.clone());
        });
    }

    /// Fetch output 2d geometries.
    ///
    /// Panics if the node does not contain any 2d geometry.
    pub fn fetch_output_geometries_2d(&self) -> Geometries2D {
        let b = self.borrow();
        match &b.output().geometry {
            ModelNodeGeometryOutput::Geometries2D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 2D geometry."),
        }
    }

    /// Fetch output 3d geometries.
    ///
    /// Panics if the node does not contain any 3d geometry.
    pub fn fetch_output_geometries_3d(&self) -> Geometries3D {
        let b = self.borrow();
        match &b.output().geometry {
            ModelNodeGeometryOutput::Geometries3D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 3D geometry."),
        }
    }

    /// Render the node.
    ///
    /// Rendering the node means that all geometry is calculated and stored
    /// in the respective model node output.
    /// This means after rendering, the rendered geometry can be retrieved via:
    /// * `fetch_output_geometries_2d()` for 2D geometries.
    /// * `fetch_output_geometries_3d()` for 3D geometries.
    pub fn render(&self) {
        fn render_geometries_2d(node: &ModelNode) -> Geometries2D {
            let b = node.borrow();
            match b.element() {
                Element::Primitive2D(geometry) => geometry.clone().into(),
                Element::Operation(operation) => operation.process_2d(node),
                _ => Geometries2D::default(),
            }
        }

        fn is_operation(node: &ModelNode) -> bool {
            let b = node.borrow();
            matches!(b.element(), Element::Operation(_))
        }

        match self.output_type() {
            ModelNodeOutputType::Geometry2D => {
                let geometries = render_geometries_2d(self);
                if !is_operation(self) {
                    self.children().for_each(|node| {
                        node.render();
                    });
                }

                let mut b = self.borrow_mut();
                b.output_mut().geometry = ModelNodeGeometryOutput::Geometries2D(geometries);
            }
            ModelNodeOutputType::Geometry3D => todo!(),
            _ => unreachable!(),
        }
    }

    /// Test if the node has this specific source file.
    pub fn has_source_file(&self, source_file: &std::rc::Rc<SourceFile>) -> bool {
        match (source_file.as_ref(), self.find_source_file()) {
            (a, Some(b)) => a.hash == b.hash,
            _ => false,
        }
    }

    /// Return inner node if we are in an [`Object`] node.
    pub fn into_inner_object_node(&self) -> Option<ModelNode> {
        self.children().next().and_then(|n| {
            if let Element::Object(_) = n.0.borrow().element.value {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
    pub fn signature(&self) -> String {
        let mut s = String::new();
        if let Some(id) = self.id() {
            s += format!("{id}: ").as_str();
        }
        s += self.borrow().element().to_string().as_str();
        if self.origin().creator.is_some() {
            s += format!(" = {origin}", origin = self.origin()).as_str();
        }
        if !matches!(self.output_type(), ModelNodeOutputType::NotDetermined) {
            s += format!(" -> \"{output_type}\"", output_type = self.output_type()).as_str();
        }
        if self.has_children() {
            s += ":";
        }
        s
    }
}

/// Implementation to store information about the node origin.
impl ModelNode {
    /// Set the information about the creator of this node.
    ///
    /// This function is called after the resulting nodes of a call of a part
    /// have been retrieved.   
    pub(crate) fn set_creator(&self, creator: Symbol, call_src_ref: SrcRef) {
        let origin = &mut self.0.borrow_mut().origin;
        origin.creator = Some(creator);
        origin.call_src_ref = call_src_ref;
    }

    /// Set the arguments with have been passed to this node.
    pub(crate) fn set_original_arguments(&self, arguments: ArgumentMap) -> Self {
        let origin = &mut self.0.borrow_mut().origin;
        origin.arguments = arguments;
        self.clone()
    }

    /// Set the original source file this node has been created from.
    pub(crate) fn set_original_source_file(&self, source_file: Rc<SourceFile>) -> Self {
        let origin = &mut self.0.borrow_mut().origin;
        origin.source_file = Some(source_file);
        self.clone()
    }
}

/// Iterator methods.
impl ModelNode {
    /// Return parent of this node.
    pub fn parent(&self) -> Option<ModelNode> {
        self.0.borrow().parent.clone()
    }

    /// Returns `true` if this node has children.
    pub fn has_children(&self) -> bool {
        !self.borrow().children().is_empty()
    }

    /// Children iterator.
    pub fn children(&self) -> Children {
        Children::new(self.clone())
    }

    /// Returns an iterator of nodes to this node and its unnamed descendants, in tree order.
    ///
    /// Includes the current node.
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    /// Returns an iterator of nodes that belong to the same source file as this one
    pub fn source_file_descendants(&self) -> SourceFileDescendants {
        SourceFileDescendants::new(self.clone())
    }

    /// Parents iterator.
    pub fn parents(&self) -> Parents {
        Parents::new(self.clone())
    }

    /// Ancestors iterator.
    pub fn ancestors(&self) -> Ancestors {
        Ancestors::new(self.clone())
    }
}

/// Model node attribute setter
impl ModelNode {
    /// Set attributes.
    pub fn set_attributes(&self, attributes: Attributes) -> Self {
        self.0.borrow_mut().set_attributes(attributes);
        self.clone()
    }
}

impl Operation for ModelNode {
    fn output_type(&self, node: &ModelNode) -> ModelNodeOutputType {
        node.output_type()
    }

    fn process_2d(&self, node: &ModelNode) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        let b = node.borrow();
        match b.element() {
            Element::Transform(_) | Element::Object(_) => {
                node.children()
                    .for_each(|node| geometries.append(node.process_2d(&node)));
            }
            Element::Primitive2D(geo) => {
                geometries.push(geo.clone());
                node.children()
                    .for_each(|node| geometries.append(node.process_2d(&node)));
            }
            Element::Operation(operation) => geometries.append(operation.process_2d(node)),
            _ => {}
        }

        geometries
    }
}

impl GetPropertyValue for ModelNode {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.borrow().element().get_property_value(id)
    }
}

impl GetAttribute for ModelNode {
    fn get_attribute(&self, id: &Identifier) -> Option<crate::model_tree::Attribute> {
        self.borrow().attributes().get_attribute(id)
    }
}

impl FetchBounds2D for ModelNode {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        let mut bounds = Bounds2D::default();

        self.descendants().for_each(|node| {
            let b = node.borrow();
            let output = b.output();
            if let ModelNodeGeometryOutput::Geometries2D(geometries) = &output.geometry {
                let mat = output.matrix_2d();
                let resolution = &output.resolution;
                bounds = bounds.clone().extend(
                    geometries
                        .fetch_bounds_2d()
                        .transformed_2d(resolution, &mat),
                );
            }
        });

        bounds
    }
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

/// Prints a [`ModelNode`].
///
/// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
/// The examplary output will look like this:
///
/// ```custom
/// id: Object:
///     Object = std::geo2d::circle(radius = 3.0mm) -> Geometry2D:
///         Primitive = __builtin::geo2d::circle(radius = 3.0) -> Geometry2D`
/// ```
impl std::fmt::Display for ModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.depth() * 2;
        writeln!(f, "{:depth$}{signature}", "", signature = self.signature())?;
        for child in self.children() {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}
