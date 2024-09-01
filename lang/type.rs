//! µCAD Basic Types

use crate::{
    eval::*,
    parse::*,
    parser::*,
    src_ref::{SrcRef, SrcReferrer},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // A 64-bit integer number
    Integer,
    /// A 64-bit floating-point number
    Scalar,
    /// A string
    String,
    /// An RGBA color
    Color,
    /// A physical length, e.g. 4.0mm
    Length,
    /// An angle, e.g. 90°
    Angle,
    /// A physical weight, e.g. 4.0kg
    Weight,
    /// A two-dimensional vector, maps from named tuple (x: length, y: length)    
    Vec2,
    /// A three-dimensional vector, maps from named tuple (x: length, y: length, z: length)
    Vec3,
    /// A three-dimensional vector, maps from named tuple (x: length, y: length, z: length, w: length)
    Vec4,
    /// A boolean: true, false
    Bool,
    /// A list of elements of the same type: `[scalar]`
    List(ListType),
    /// A map of elements: `[string => scalar]`
    Map(MapType),
    /// An unnamed tuple of elements: `(scalar, string)`
    UnnamedTuple(UnnamedTupleType),
    /// A named tuple of elements: `(x: scalar, y: string)`
    NamedTuple(NamedTupleType),
    /// A custom type or a module node in the syntax tree
    Custom(QualifiedName),
    /// Node
    Node,
}

impl Type {
    pub fn default_unit(&self) -> Unit {
        match self {
            Self::Length => Unit::Mm,
            Self::Angle => Unit::Rad,
            Self::List(t) => t.ty().default_unit(),
            _ => Unit::None,
        }
    }

    /// Check if the type is a named tuple
    pub fn is_named_tuple(&self) -> bool {
        matches!(self, Self::NamedTuple(_))
    }

    /// Check if the type is a list of the given type `ty`
    pub fn is_list_of(&self, ty: &Type) -> bool {
        match self {
            Self::List(list_type) => &list_type.ty() == ty,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation(pub Type, pub SrcRef);

impl SrcReferrer for TypeAnnotation {
    fn src_ref(&self) -> SrcRef {
        self.1.clone()
    }
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.clone()
    }
}

impl From<Type> for TypeAnnotation {
    fn from(value: Type) -> Self {
        TypeAnnotation(value, SrcRef(None))
    }
}

impl Parse for TypeAnnotation {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.clone().into_inner().next().unwrap();

        let s = match inner.as_rule() {
            Rule::list_type => Self(Type::List(ListType::parse(inner)?), pair.into()),
            Rule::map_type => Self(Type::Map(MapType::parse(inner)?), pair.into()),
            Rule::unnamed_tuple_type => Self(
                Type::UnnamedTuple(UnnamedTupleType::parse(inner)?),
                pair.into(),
            ),
            Rule::named_tuple_type => {
                Self(Type::NamedTuple(NamedTupleType::parse(inner)?), pair.into())
            }
            Rule::qualified_name => match inner.as_str() {
                "int" => Self(Type::Integer, pair.into()),
                "scalar" => Self(Type::Scalar, pair.into()),
                "string" => Self(Type::String, pair.into()),
                "color" => Self(Type::Color, pair.into()),
                "length" => Self(Type::Length, pair.into()),
                "angle" => Self(Type::Angle, pair.into()),
                "vec2" => Self(Type::Vec2, pair.into()),
                "vec3" => Self(Type::Vec3, pair.into()),
                "bool" => Self(Type::Bool, pair.into()),
                _ => Self(Type::Custom(QualifiedName::parse(inner)?), pair.into()),
            },
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        };

        Ok(s)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Scalar => write!(f, "scalar"),
            Self::String => write!(f, "string"),
            Self::Color => write!(f, "color"),
            Self::Length => write!(f, "length"),
            Self::Angle => write!(f, "angle"),
            Self::Weight => write!(f, "weight"),
            Self::Vec2 => write!(f, "vec2"),
            Self::Vec3 => write!(f, "vec3"),
            Self::Vec4 => write!(f, "vec4"),
            Self::Bool => write!(f, "bool"),
            Self::List(t) => write!(f, "{}", t),
            Self::Map(t) => write!(f, "{}", t),
            Self::UnnamedTuple(t) => write!(f, "{}", t),
            Self::NamedTuple(t) => write!(f, "{}", t),
            Self::Custom(qn) => write!(f, "{}", qn),
            Self::Node => write!(f, "{{}}"),
        }
    }
}

#[test]
fn builtin_type() {
    let ty = Parser::parse_rule_or_panic::<TypeAnnotation>(Rule::r#type, "int");
    assert_eq!(ty.0.to_string(), "int");
    assert_eq!(ty.0, Type::Integer);
}
