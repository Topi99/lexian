use std::collections::{HashMap};
use super::grammar::{Grammar};

/// Estructura de un parser genérico.
pub struct Parser {
  pub stack: Vec<String>,
  pub input: Vec<String>,
  pub rule: Vec<String>,
}

/// Estructura que representa un analizador LL1.
pub struct LL1Analyzer<'analyzer> {
  /// Tabla de parseo predictivo del analizador.
  pub table: HashMap<String, HashMap<String, Vec<i8>>>,
  /// Parser predictivo no recursivo.
  pub parser: Parser,
  pub grammar: &'analyzer Grammar,
}

impl<'analyzer> LL1Analyzer<'analyzer> {
  pub fn new(
    grammar: &'analyzer Grammar
  ) -> LL1Analyzer {
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
