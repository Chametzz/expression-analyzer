#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn from_char(character: char) -> Option<Self> {
        match character {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _ => None,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }

    pub fn apply(&self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            Operator::Add => Ok(left + right),

            Operator::Subtract => Ok(left - right),

            Operator::Multiply => Ok(left * right),

            Operator::Divide => {
                if right == 0.0 {
                    Err("No se puede dividir entre cero".to_string())
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}