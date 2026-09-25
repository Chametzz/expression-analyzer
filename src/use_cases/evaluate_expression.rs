use crate::core::evaluation_error::EvaluationError;
use crate::core::expression::Expression;
use crate::core::symbol_table::SymbolTable;

pub fn execute(symbol_table: &mut SymbolTable, input: &str) -> Result<f64, EvaluationError> {
    // Detectar si la entrada es una asignación
    let (variable, expression_input) = if let Some(position) = input.find('=') {
        let variable = input[..position].trim().to_string();
        let expression = input[position + 1..].trim();

        (Some(variable), expression)
    } else {
        (None, input.trim())
    };

    let expression: Expression = crate::use_cases::parse_expression::execute(expression_input)?;

    let _ = symbol_table;

    Err(EvaluationError::InvalidExpression(
        "La evaluación todavía no está conectada con los tokens".to_string(),
    ))
}
