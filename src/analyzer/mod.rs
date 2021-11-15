use std::collections::{HashMap};
use super::grammar::{Grammar};

pub type TableTerminals = HashMap<String, usize>;

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
  /// Built grammar.
  pub grammar: &'analyzer mut Grammar,
}

impl<'analyzer> LL1Analyzer<'analyzer> {
  /// Crea la estructura inicial del LL1Analyzer.
  pub fn new(
    grammar: &'analyzer mut Grammar
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

  /// Crea la estructura inicial de la tabla del parser.
  fn build_table_struct(&mut self) {
    for non_terminal in self.grammar.non_terminals.to_owned() {
      self.table.insert(non_terminal, HashMap::new());
    }
  }

  /// Builds the whole parsing table with the corresponding data and rules.
  /// 1. For each terminal a in FIRST(A), add A -> a to M[A, a].
  /// 2. If EPSILON is in FIRST(a), then for each terminal b in FOLLOW(A),
  ///    add A -> a to M[A, b]. If EPSILON is in FIRST(a) and $ in FOLLOW(A),
  ///    add A -> a to M[A, $] as well.
  pub fn build_table(&mut self) {
    // key is a non terminal of the grammar.
    // value is the content of the cell in the table.
    // for (key, value) in self.table.iter() {
    //   for terminal in self.grammar.firsts.get(key) {
    //     let rule_index = 
    //     value.insert(terminal, );
    //   }
    // }
    let left = self.grammar.sides.left.to_owned();
    for (index, non_terminal) in left.iter().enumerate() {
      let production = self.grammar.productions.get(&index).unwrap();
      let first = self.grammar.quick_first_production(production);

      // primer regla
      for terminal in first.to_owned() {
        if terminal == "' '" {
          continue;
        }

        let mut row_to_insert = self.table.get(non_terminal).unwrap().clone();
        row_to_insert.insert(terminal, index);
        self.table.insert(String::from(non_terminal), row_to_insert);
      }

      // segunda regla
      if !first.contains(&String::from("' '")) {
        continue;
      }

      for terminal in self.grammar.follows.get(non_terminal).unwrap() {
        let mut row_to_insert = self.table.get(non_terminal).unwrap().clone();
        row_to_insert.insert(String::from(terminal), index);
        self.table.insert(String::from(non_terminal), row_to_insert);
      }
    }

    println!("{:?}", self.table);
  }
}
