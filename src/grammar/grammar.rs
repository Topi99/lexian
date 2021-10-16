use std::collections::HashMap;

/// Estructura para representar ambos lados de la gramática.
pub struct Sides {
  /// Lado izquierdo de el símbolo `->`.
  /// Contiene todos los items no terminales (se pueden repetir).
  pub left: Vec<String>,
  /// Lado derecho de el símbolo `->`. Contiene terminales y no terminales.
  pub right: Vec<String>,
}

pub struct Grammar {
  pub terminals: Vec<String>,
  pub non_terminals: Vec<String>,
  pub sides: Sides,
  pub firsts: HashMap<String, Vec<String>>,
  pub follows: HashMap<String, Vec<String>>,
}

impl Grammar {
  /// Transforma un vector de producciones en dos vectores, uno para el lado
  /// derecho y otro para el izquierdo.
  /// 
  /// Regresa una nueva gramática (Grammar) con los lados divididos.
  pub fn new(productions: Vec<String>) -> Self {
    let mut left = vec![];
    let mut right = vec![];

    for production in productions {
        let splited = production.split(" -> ").collect::<Vec<_>>();
        left.push(String::from(splited[0]));
        right.push(String::from(splited[1]));
    }

    Grammar {
      terminals: vec![],
      non_terminals: vec![],
      sides: Sides { left, right },
      firsts: HashMap::new(),
      follows: HashMap::new(),
    }
  }

  /// Filtra el lado izquierdo de la gramática y regresa un vector con todos
  /// los elementos no terminales.
  pub fn find_non_terminals(&mut self) {
    // Se usa un Vector<String> en lugar de HasSet<String> porque el
    // HashSet<String> ordena de diferente manera sus elementos.
    // Se quieren obtener los elementos en orden de aparición.
    let mut non_terminals = vec![];

    for element in &self.sides.left {
      // La condición es para obtener solamente elementos únicos.
      if !non_terminals.contains(element) {
        non_terminals.push(String::from(element));
      }
    }
    
    self.non_terminals = non_terminals;
  }

  /// Filtra los elementos terminales desde el lado derecho con ayuda
  /// de los no terminales.
  pub fn find_terminals(&mut self) {
    let mut splited_right = vec![];
    // Se usa un Vector<String> en lugar de HasSet<String> porque el
    // HashSet<String> ordena de diferente manera sus elementos.
    // Se quieren obtener los elementos en orden de aparición.
    let mut terminals = vec![];

    // Ya que algunos elementos del lado derecho pueden venir algo parecido
    // a esto: `( A )`, debemos volver a separar por espacios
    // todos los elementos. 
    for line in &self.sides.right {
        for element in line.split(" ") {
            splited_right.push(String::from(element));
        }
    }

    // Al final se filtran elementos que sean `'` (ya que representan el
    // símbolo Epsilon) o cualquier no terminal.
    // Solo se mantienen elementos únivos en el vector.
    for element in splited_right {
        if element != "'" && !self.non_terminals.contains(&element) && !terminals.contains(&element) {
            terminals.push(element);
        }
    }

    self.terminals = terminals;
  }

  pub fn find_first_production(&mut self, elements: &Vec<String>) -> Vec<String> {
    let mut first = vec![];

    for element in elements {
      let next_first = self.find_single_first(element);

      for maybe_next_first in next_first.to_owned() {
        if !first.contains(&maybe_next_first) {
          first.push(maybe_next_first);
        }
      }

      if !next_first.contains(&String::from("' '")) {
        break;
      }
    }

    first
  }

  pub fn find_single_first(&mut self, non_terminal: &String) -> Vec<String> {
    // Revisa si el elemento es un terminal.
    if self.terminals.contains(non_terminal) {
      return vec![String::from(non_terminal)]
    }

    let indexes = self.get_indexes_in_non_terminals(non_terminal);

    // Revisa si el FIRST del no terminal ya fue encontrado anteriormente
    if self.firsts.contains_key(non_terminal) {
      match self.firsts.get(non_terminal) {
        Some(first) => return first.to_owned(),
        None => {},
      }
    }

    let mut first = vec![];

    for index in indexes {
      let elements_in_production = self.sides.right[index]
        .split(' ').collect::<Vec<_>>();
      let first_in_body = &String::from(elements_in_production[0]);

      if first_in_body == non_terminal {
        continue;
      }

      if first_in_body.as_str() == "'" {
        first.push(String::from("' '"));
        continue;
      }

      if self.non_terminals.contains(first_in_body) {
        // Si el primer elemento de la producción es un no terminal
        // realizar búsqueda de FIRST(production).
        let mut production = vec![];
        for element in elements_in_production {
          production.push(String::from(element));
        }
        for maybe_first in self.find_first_production(&production) {
          if !first.contains(&maybe_first) {
            first.push(maybe_first);
          }
        }

        continue;
      } 

      if !first.contains(first_in_body) {
        first.push(String::from(first_in_body));
      }
    }

    // Guardar en el "caché" el FIRST del no terminal
    self.firsts.insert(String::from(non_terminal), first.to_owned());
    first
  }

  pub fn find_follow(&mut self, non_terminal: &String) -> Vec<String> {
    let mut follow = vec![];
    let indexes = self.get_indexes_in_non_terminals(non_terminal);

    // primera regla
    if indexes.contains(&0) {
      follow.push(String::from("$"));
    }

    // segunda regla

    follow
  }

  fn get_indexes_in_non_terminals(&self, non_terminal: &String) -> Vec<usize> {
      let mut indexes = vec![];

      for (index, value) in self.sides.left.iter().enumerate() {
          if non_terminal == value {
              indexes.push(index)
          }
      }
      indexes
  }

  // fn find_in_right_side(
  //   &self, non_terminal: &String,
  // ) -> Vec<(usize, usize)> {
  //   let result = vec![];

  //   for (prod_index, prod) in self.sides.right.iter().enumerate() {
  //     for 
  //   }

  //   result
  // }
}
