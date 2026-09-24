#[derive(Debug, Clone)]
pub enum EvaluationError {
    InvalidExpression(String),
    InvalidToken(String),
    DivisionByZero,
    UnknownIdentifier(String),
    MissingOperand,
    MissingOperator,
    UnexpectedToken(String),
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::InvalidExpression(message) => {
                write!(formatter, "Expresión inválida: {}", message)
            }

            EvaluationError::InvalidToken(token) => {
                write!(formatter, "Token inválido: {}", token)
            }

            EvaluationError::DivisionByZero => {
                write!(formatter, "Error: división entre cero")
            }

            EvaluationError::UnknownIdentifier(identifier) => {
                write!(formatter, "Identificador desconocido: {}", identifier)
            }

            EvaluationError::MissingOperand => {
                write!(formatter, "Error: falta un operando")
            }

            EvaluationError::MissingOperator => {
                write!(formatter, "Error: falta un operador")
            }

            EvaluationError::UnexpectedToken(token) => {
                write!(formatter, "Token inesperado: {}", token)
            }
        }
    }
}

impl std::error::Error for EvaluationError {}
