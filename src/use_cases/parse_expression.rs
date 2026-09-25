use crate::core::evaluation_error::EvaluationError;
use crate::core::expression::Expression;
use crate::core::operator::Operator;
use crate::core::token::Token;

pub fn execute(raw_input: &str) -> Result<Expression, EvaluationError> {
    let raw = raw_input.trim().to_string();

    // Validar que la expresión no esté vacía
    if raw.is_empty() {
        return Err(EvaluationError::InvalidExpression(
            "La expresión está vacía".to_string(),
        ));
    }

    let chars: Vec<char> = raw.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let mut parentheses_count = 0;

    while i < chars.len() {
        let current = chars[i];

        // Ignorar espacios en blanco
        if current.is_whitespace() {
            i += 1;
            continue;
        }

        // Detectar números enteros y decimales
        if current.is_ascii_digit()
            || (current == '.'
                && i + 1 < chars.len()
                && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            let mut decimal_count = 0;

            while i < chars.len()
                && (chars[i].is_ascii_digit() || chars[i] == '.')
            {
                if chars[i] == '.' {
                    decimal_count += 1;
                }

                if decimal_count > 1 {
                    return Err(EvaluationError::InvalidToken(
                        "Número decimal inválido".to_string(),
                    ));
                }

                i += 1;
            }

            let number_string: String =
                chars[start..i].iter().collect();

            let number = number_string
                .parse::<f64>()
                .map_err(|_| {
                    EvaluationError::InvalidToken(
                        number_string.clone(),
                    )
                })?;

            tokens.push(Token::Number(number));
            continue;
        }

        // Detectar identificadores y variables
        if current.is_alphabetic() || current == '_' {
            let start = i;

            while i < chars.len()
                && (chars[i].is_alphanumeric() || chars[i] == '_')
            {
                i += 1;
            }

            let identifier: String =
                chars[start..i].iter().collect();

            tokens.push(Token::Identifier(identifier));
            continue;
        }

        // Detectar operadores aritméticos
        if let Some(operator) = Operator::from_char(current) {
            tokens.push(Token::Operator(operator));
            i += 1;
            continue;
        }

        // Detectar paréntesis izquierdo
        if current == '(' {
            tokens.push(Token::LeftParen);
            parentheses_count += 1;
            i += 1;
            continue;
        }

        // Detectar paréntesis derecho
        if current == ')' {
            if parentheses_count == 0 {
                return Err(EvaluationError::InvalidExpression(
                    "Paréntesis desbalanceados".to_string(),
                ));
            }

            parentheses_count -= 1;
            tokens.push(Token::RightParen);
            i += 1;
            continue;
        }

        // Detectar caracteres no reconocidos
        return Err(EvaluationError::InvalidToken(
            current.to_string(),
        ));
    }

    // Validar paréntesis pendientes
    if parentheses_count != 0 {
        return Err(EvaluationError::InvalidExpression(
            "Paréntesis desbalanceados".to_string(),
        ));
    }

    // Crear la expresión con el texto original y los tokens
    Ok(Expression::new(raw, tokens))
}