/// Estructura para representar ambos lados de la gramática.
pub struct Sides {
  /// Lado izquierdo de el símbolo `->`.
  /// Contiene todos los items no terminales (se pueden repetir).
  pub left: Vec<String>,
  /// Lado derecho de el símbolo `->`. Contiene terminales y no terminales.
  pub right: Vec<String>,
}

pub struct Grammar {
  terminals: Vec<String>,
  non_terminals: Vec<String>,
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
  fn get_non_terminals(left_side: &Vec<String>) -> Vec<String> {
    // Se usa un Vector<String> en lugar de HasSet<String> porque el
    // HashSet<String> ordena de diferente manera sus elementos.
    // Se quieren obtener los elementos en orden de aparición.
    let mut non_terminals = vec![];

    for element in left_side {
      // La condición es para obtener solamente elementos únicos.
      if !non_terminals.contains(element) {
        non_terminals.push(String::from(element));
      }
    }

    non_terminals
  }
}
