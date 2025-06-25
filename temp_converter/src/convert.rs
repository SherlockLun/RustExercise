#[derive(Debug, PartialEq)]
pub enum Scale
{
    Celsius,
    Fahrenheit,
}

pub fn parse_scale(scale: &str) -> Option<Scale>
{
    match scale.to_lowercase().as_str()
    {
        "celsius" | "c" => Some(Scale::Celsius),
        "fahrenheit" | "f" => Some(Scale::Fahrenheit),
        _ => None,
    }
}

pub fn convert(value: f64, scale: &Scale) -> f64
{
    match scale
    {
        Scale::Celsius => value * 9.0 / 5.0 + 32.0,
        Scale::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_scale()
    {
        assert_eq!(parse_scale("Celsius"), Some(Scale::Celsius));
        assert_eq!(parse_scale("c"), Some(Scale::Celsius));
        assert_eq!(parse_scale("Fahrenheit"), Some(Scale::Fahrenheit));
        assert_eq!(parse_scale("f"), Some(Scale::Fahrenheit));
        assert_eq!(parse_scale("Kelvin"), None);
        assert_eq!(parse_scale("K"), None);
    }

    #[test]
    fn test_convert()
    {
        assert_eq!(convert(0.0, &Scale::Celsius), 32.0);
        assert_eq!(convert(32.0, &Scale::Fahrenheit), 0.0);
        assert_eq!(convert(100.0, &Scale::Celsius), 212.0);
        assert_eq!(convert(212.0, &Scale::Fahrenheit), 100.0);
        assert_eq!(convert(-40.0, &Scale::Celsius), -40.0);
        assert_eq!(convert(-40.0, &Scale::Fahrenheit), -40.0);
    }
}