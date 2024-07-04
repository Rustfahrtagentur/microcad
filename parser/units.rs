pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
}

impl LengthUnit {
    pub fn to_millimeters(&self) -> f64 {
        match self {
            LengthUnit::Millimeter => 1.0,
            LengthUnit::Centimeter => 10.0,
            LengthUnit::Meter => 1000.0,
            LengthUnit::Kilometer => 1_000_000.0,
            LengthUnit::Inch => 25.4,
            LengthUnit::Foot => 304.8,
            LengthUnit::Yard => 914.4,
            LengthUnit::Mile => 1_609_344.0,
        }
    }

    pub fn sign(&self) -> &'static str {
        match self {
            LengthUnit::Millimeter => "mm",
            LengthUnit::Centimeter => "cm",
            LengthUnit::Meter => "m",
            LengthUnit::Kilometer => "km",
            LengthUnit::Inch => "in",
            LengthUnit::Foot => "ft",
            LengthUnit::Yard => "yd",
            LengthUnit::Mile => "mi",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mm" => Some(LengthUnit::Millimeter),
            "cm" => Some(LengthUnit::Centimeter),
            "m" => Some(LengthUnit::Meter),
            "km" => Some(LengthUnit::Kilometer),
            "in" => Some(LengthUnit::Inch),
            "ft" => Some(LengthUnit::Foot),
            "yd" => Some(LengthUnit::Yard),
            "mi" => Some(LengthUnit::Mile),
            _ => None,
        }
    }
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sign())
    }
}
