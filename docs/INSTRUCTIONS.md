# INSTRUCCIONES
## Definición del Núcleo

### 1. `core/token.rs`
* **Propósito:** Representar los componentes individuales de una expresión matemática.
* **Requisitos:**
  * Debe ser un `enum` público con las siguientes variantes:
    * `Number`: Para guardar valores numéricos flotantes (`f64`).
    * `Variable`: Para almacenar el nombre/identificador de una variable (`String`).
    * `Operator`: Para envolver un objeto de tipo `Operator`.
    * `LeftParen` y `RightParen`: Para representar paréntesis de apertura y cierre.
  * Debe derivar los traits básicos para permitir comparación e inmutabilidad eficiente (`Debug`, `PartialEq`, `Clone`).

---

### 2. `core/operator.rs`
* **Propósito:** Representar los operadores aritméticos válidos y sus reglas de precedencia y ejecución.
* **Requisitos:**
  * Debe ser un `enum` público con las variantes: `Add`, `Subtract`, `Multiply`, `Divide`.
  * **Método `precedence(&self) -> u8`:** Retornar la jerarquía de operaciones (multiplicar/dividir tiene mayor precedencia que sumar/restar).
  * **Método `apply(&self, left: f64, right: f64) -> Result<f64, EvaluationError>`:** 
    * Realizar la operación aritmética indicada entre dos números.
    * Retornar un error `EvaluationError::DivisionByZero` si se intenta dividir por cero (`0.0`).

---

### 3. `core/evaluation_error.rs`
* **Propósito:** Catalogar todos los fallos posibles que pueden surgir durante el parseo o la evaluación matemática.
* **Requisitos:**
  * Debe ser un `enum` público que incluya casos para:
    * División entre cero.
    * Variable no definida o no encontrada en la tabla.
    * Error de sintaxis en la expresión ingresada.
    * Paréntesis desbalanceados o no coincidentes.
    * Expresión vacía o sin tokens válidos.

---

### 4. `core/expression.rs`
* **Propósito:** Encapsular una expresión matemática junto con su representación de tokens.
* **Requisitos:**
  * Debe ser un `struct` inmutable con dos campos privados:
    * La cadena de texto original (`raw`).
    * La lista ordenada de tokens (`tokens`).
  * **Constructor `new`:** Debe recibir el texto original y el vector de `Token`s.
  * **Getters:** Proveer métodos de solo lectura para consultar la cadena original y la lista de tokens.

---

### 5. `core/symbol_table.rs`
* **Propósito:** Representar la entidad mutable del sistema que administra el estado persistente durante la sesión (variables registradas y contadores de uso de operadores).
* **Requisitos:**
  * Debe mantener internamente dos mapas hash (`HashMap`):
    * Un registro de variables (nombre -> valor numérico).
    * Un contador de apariciones de operadores (símbolo -> cantidad de veces usado).
  * **Métodos públicos obligatorios:**
    * `new()`: Inicializar la tabla vacía.
    * `set_variable(name, value)`: Registrar o actualizar el valor de una variable.
    * `get_variable(name)`: Consultar el valor asignado a un identificador (retorna `Option`).
    * `register_operator(symbol)`: Incrementar el contador de apariciones de un operador (`+`, `-`, `*`, `/`).
    * `get_variables()` y `get_operators()`: Retornar referencias de lectura a las colecciones internas para las vistas de consulta.

---

## Casos de Uso

### 1. `use_cases/parse_expression.rs`
* **Propósito:** Transformar una cadena de texto en un objeto `Expression` compuesto por tokens válidos.
* **Requisitos:**
  * **Función `execute(raw_input: &str) -> Result<Expression, EvaluationError>`:**
    * Recibir la cadena ingresada por el usuario y remover espacios en blanco innecesarios.
    * Recorrer el texto identificando números flotantes, variables, operadores aritméticos (`+`, `-`, `*`, `/`) y paréntesis.
    * Retornar un `EvaluationError::SyntaxError` si encuentra caracteres no reconocidos o paréntesis desbalanceados.
    * Retornar un `EvaluationError::EmptyExpression` si la entrada está vacía.
    * Construir y retornar una nueva `Expression` conteniendo el texto original y el vector de tokens generado.

---

### 2. `use_cases/evaluate_expression.rs`
* **Propósito:** Orquestar el análisis, registro y cálculo matemático de una expresión o asignación.
* **Requisitos:**
  * **Función `execute(symbol_table: &mut SymbolTable, input: &str) -> Result<f64, EvaluationError>`:**
    * Detectar si la entrada es una asignación de variable (ej. `x = 10` o `x = y + 5`).
    * Invocar a `parse_expression::execute` para obtener el objeto `Expression`.
    * Registrar en el `SymbolTable` los operadores encontrados durante el análisis utilizando `register_operator`.
    * Evaluar los tokens respetando la jerarquía de operadores y paréntesis:
      * Para variables: buscar su valor en `SymbolTable` vía `get_variable` (retornar `EvaluationError::UndefinedVariable` si no existe).
      * Para operaciones: invocar el método `apply` de `Operator`.
    * Si es una asignación (`var = expr`), guardar/actualizar el resultado en `SymbolTable` usando `set_variable`.
    * Retornar el resultado numérico (`f64`) de la evaluación.

---

### 3. `use_cases/get_stored_variables.rs`
* **Propósito:** Proveer a las vistas el listado de identificadores y sus valores numéricos persistidos en la sesión.
* **Requisitos:**
  * **Función `execute(symbol_table: &SymbolTable) -> &HashMap<String, f64>`:**
    * Recibir una referencia de solo lectura del `SymbolTable`.
    * Invocar el método `get_variables()` del `SymbolTable`.
    * Retornar la colección de variables para ser iterada y renderizada por la vista correspondiente.

---

### 4. `use_cases/get_stored_operators.rs`
* **Propósito:** Proveer a las vistas el reporte de frecuencia de uso de los operadores aritméticos registrados en la sesión.
* **Requisitos:**
  * **Función `execute(symbol_table: &SymbolTable) -> &HashMap<String, usize>`:**
    * Recibir una referencia de solo lectura del `SymbolTable`.
    * Invocar el método `get_operators()` del `SymbolTable`.
    * Retornar la colección de contadores de operadores para su despliegue en la vista.