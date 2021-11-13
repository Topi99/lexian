use std::collections::{HashMap};
use super::grammar::{Grammar};

pub type TableTerminals = HashMap<String, i8>;

/// Estructura de un parser genérico.
pub struct Parser {
  pub stack: Vec<String>,
  pub input: Vec<String>,
  pub rule: Vec<String>,
}

/// Estructura que representa un analizador LL1.
pub struct LL1Analyzer<'analyzer> {
  /// Tabla de parseo predictivo del analizador.
  pub table: HashMap<String, TableTerminals>,
  /// Parser predictivo no recursivo.
  pub parser: Parser,
  pub grammar: &'analyzer Grammar,
}

impl<'analyzer> LL1Analyzer<'analyzer> {
  pub fn new(
    grammar: &'analyzer Grammar
  ) -> LL1Analyzer {
    let mut analyzer = LL1Analyzer {
      table: HashMap::new(),
      parser: Parser {
        stack: vec![],
        input: vec![],
        rule: vec![],
      },
      grammar,
    };

    analyzer.build_table_struct();

    analyzer
  }

  fn build_table_struct(&mut self) {
    for non_terminal in self.grammar.non_terminals.to_owned() {
      self.table.insert(non_terminal, HashMap::new());
    }
  }
}
