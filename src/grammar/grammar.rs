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

  pub fn find_first(&self, non_terminal: &String) -> Vec<String> {
    if self.terminals.contains(non_terminal) {
      return vec![String::from(non_terminal)]
    }
    
    let indexes = self.get_indexes(non_terminal);
    let mut first = vec![];

    for index in indexes {
      let first_in_body = &String::from(
        self.sides.right[index].split(' ').collect::<Vec<_>>()[0]
      );

      if first_in_body == non_terminal {
        continue;
      }

      if first_in_body.as_str() == "'" {
        first.push(String::from("' '"));
        continue;
      }

      if self.sides.left.contains(first_in_body) {
        for maybe_first in self.find_first(first_in_body) {
          if !first.contains(&maybe_first) {
            first.push(maybe_first);
          }
        }
      } else if !first.contains(first_in_body) {
        first.push(String::from(first_in_body));
      }
    }

    first
  }

  fn get_indexes(&self, non_terminal: &String) -> Vec<usize> {
      let mut indexes = vec![];

      for (index, value) in self.sides.left.iter().enumerate() {
          if non_terminal == value {
              indexes.push(index)
          }
      }
      indexes
  }
}
