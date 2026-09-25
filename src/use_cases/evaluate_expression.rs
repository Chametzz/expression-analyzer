use crate::core::evaluation_error::EvaluationError;
use crate::core::expression::Expression;
use crate::core::operator::Operator;
use crate::core::symbol_table::SymbolTable;
use crate::core::token::Token;

pub fn execute(symbol_table: &mut SymbolTable, input: &str) -> Result<f64, EvaluationError> {
    let (target_variable, expression_input) = if let Some(position) = input.find('=') {
        let variable = input[..position].trim().to_string();
        let expression = input[position + 1..].trim();
        (Some(variable), expression)
    } else {
        (None, input.trim())
    };

    let expression: Expression = crate::use_cases::parse_expression::execute(expression_input)?;
    let tokens = expression.tokens();

    if tokens.is_empty() {
        return Err(EvaluationError::InvalidExpression(
            "La expresión está vacía".to_string(),
        ));
    }

    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) | Token::Identifier(_) => {
                output_queue.push(token.clone());
            }
            Token::Operator(op) => {
                // Registrar el operador en la SymbolTable usando su símbolo correspondiente
                let symbol_str = match op {
                    Operator::Add => "+",
                    Operator::Subtract => "-",
                    Operator::Multiply => "*",
                    Operator::Divide => "/",
                };
                symbol_table.register_operator(symbol_str);

                while let Some(Token::Operator(top_op)) = operator_stack.last() {
                    if top_op.precedence() >= op.precedence() {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token.clone());
            }
            Token::LeftParen => {
                operator_stack.push(token.clone());
            }
            Token::RightParen => {
                let mut found_left = false;
                while let Some(top) = operator_stack.pop() {
                    if matches!(top, Token::LeftParen) {
                        found_left = true;
                        break;
                    }
                    output_queue.push(top);
                }
                if !found_left {
                    return Err(EvaluationError::InvalidExpression(
                        "Paréntesis desbalanceados".to_string(),
                    ));
                }
            }
        }
    }

    while let Some(top) = operator_stack.pop() {
        if matches!(top, Token::LeftParen | Token::RightParen) {
            return Err(EvaluationError::InvalidExpression(
                "Paréntesis desbalanceados".to_string(),
            ));
        }
        output_queue.push(top);
    }

    let mut eval_stack: Vec<f64> = Vec::new();

    for token in output_queue {
        match token {
            Token::Number(val) => eval_stack.push(val),
            Token::Identifier(var_name) => {
                let val = symbol_table
                    .get_variable(&var_name)
                    .copied()
                    .ok_or_else(|| EvaluationError::UnknownIdentifier(var_name))?;
                eval_stack.push(val);
            }
            Token::Operator(op) => {
                let right = eval_stack
                    .pop()
                    .ok_or(EvaluationError::MissingOperand)?;
                let left = eval_stack
                    .pop()
                    .ok_or(EvaluationError::MissingOperand)?;

                let result = op.apply(left, right).map_err(|_| EvaluationError::DivisionByZero)?;
                eval_stack.push(result);
            }
            _ => {}
        }
    }

    let final_result = eval_stack
        .pop()
        .ok_or(EvaluationError::MissingOperand)?;

    if !eval_stack.is_empty() {
        return Err(EvaluationError::MissingOperator);
    }

    if let Some(var_name) = target_variable {
        symbol_table.set_variable(var_name, final_result);
    }

    Ok(final_result)
}