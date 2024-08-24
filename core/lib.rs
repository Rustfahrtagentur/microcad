use compact_str::CompactString;

pub type Scalar = f64;
pub type Vec2 = cgmath::Vector2<Scalar>;
pub type Vec3 = cgmath::Vector3<Scalar>;
pub type Vec4 = cgmath::Vector4<Scalar>;
pub type Mat2 = cgmath::Matrix2<Scalar>;
pub type Mat3 = cgmath::Matrix3<Scalar>;
pub type Mat4 = cgmath::Matrix4<Scalar>;
pub type Angle = cgmath::Rad<Scalar>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(pub compact_str::CompactString);

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Identifier {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(CompactString::from(s)))
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        CompactString::into_string(value.0)
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(value: &'a Identifier) -> Self {
        &value.0
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
