use std::collections::{HashMap};
use super::grammar::{Grammar};

/// Estructura de un parser genérico.
pub struct Parser {
  pub stack: Vec<String>,
  pub input: Vec<String>,
  pub rule: Vec<String>,
}

/// Estructura que representa un analizador LL1.
pub struct LL1Analyzer {
  /// Tabla de parseo predictivo del analizador.
  pub table: HashMap<String, HashMap<String, Vec<i8>>>,
  /// Parser predictivo no recursivo.
  pub parser: Parser,
  pub grammar: Grammar,
}

impl LL1Analyzer {
  pub fn new(grammar: Grammar) -> Self {
    LL1Analyzer {
      table: HashMap::new(),
      parser: Parser {
        stack: vec![],
        input: vec![],
        rule: vec![],
      },
      grammar,
    }
  }
}
