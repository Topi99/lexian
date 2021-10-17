use std::collections::{HashMap, HashSet};
use itertools::Itertools;

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
  productions: HashMap<usize, Vec<String>>,
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
      productions: HashMap::new(),
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

  /// Regresa FIRST de toda la producción.
  fn find_first_production(&mut self, elements: &Vec<String>) -> Vec<String> {
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

  /// Regresa FIRST de un no terminal.
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
      let production = self.get_production(index);
      let first_in_body = &production[0];

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

  /// Revisa si la gramática es LL(1) siguiendo las 3 condiciones.
  pub fn is_ll1(&mut self) -> bool {
    for non_terminal in self.non_terminals.to_owned() {
      let indexes = &mut self.get_indexes_in_non_terminals(&non_terminal);
      // Si el no terminal solo tiene una produccieon o menos (no se si pueda)
      // suceder, no se aplica ninguna regla. Solo aplica para terminales de
      // la forma A -> a | b
      if indexes.len() <= 1 {
        continue;
      }

      for pair_of_indexes in indexes.to_owned().into_iter().combinations(2) {
        // Obtenemos objetos y estructuras necesarias para las tres reglas.
        let side_1 = self.sides.right[pair_of_indexes[0]].to_owned();
        let side_2 = self.sides.right[pair_of_indexes[1]].to_owned();
        let production_1 = self.side_to_prod(&side_1);
        let production_2 = self.side_to_prod(&side_2);
        // Se obtienen first
        let first_1: HashSet<String> = self.find_first_production(
          &production_1,
        ).into_iter().collect();
        let first_2: HashSet<String> = self.find_first_production(
          &production_2,
        ).into_iter().collect();

        // primera regla: la intersección de FIRST(a) y FIRST(b) debe ser le
        // conjunto vacío: FIRST(a) ∩ FIRST(b) = ∅
        let intersection: Vec<&String> = first_1.intersection(
          &first_2,
        ).collect();

        // Aquí se validan tanto la primera como la segunda regla. Si los dos
        // derivan en epsilon, la intersección será diferente que cero.
        if intersection.len() != 0 {
          return false;
        }

        // Revisa la tercer regla.
        if first_1.contains("' '") {
          let follow_set: HashSet<String> = self.find_follow(
            &non_terminal,
          ).into_iter().collect();
          let third_rule_inter: Vec<&String> = first_2.intersection(
            &follow_set,
          ).collect();
          
          if third_rule_inter.len() != 0 {
            return false;
          }
        }

        if first_2.contains("' '") {
          let follow_set: HashSet<String> = self.find_follow(
            &non_terminal,
          ).into_iter().collect();
          let third_rule_inter: Vec<&String> = first_1.intersection(
            &follow_set,
          ).collect();

          if third_rule_inter.len() != 0 {
            return false;
          }
        }
      }
    }

    return true;
  }

  /// Regresa FOLLOW de un terminal.
  pub fn find_follow(&mut self, non_terminal: &String) -> Vec<String> {
    if self.follows.contains_key(non_terminal) {
      match self.follows.get(non_terminal) {
        Some(follow) => return follow.to_owned(),
        None => {},
      }
    }

    let mut follow = vec![];
    let mut _use_third_rule = false;
    let indexes = self.get_indexes_in_non_terminals(non_terminal);

    // primera regla
    if indexes.contains(&0) {
      follow.push(String::from("$"));
    }

    // segunda regla
    let locations = self.find_in_right_side(non_terminal);

    for (right_index, prod_index) in locations {
      _use_third_rule = false;

      let production = self.get_production(right_index);
      
      // Si es el último elemento de la producción, aplicar la tercer regla.
      if prod_index + 1 == production.len() {
        _use_third_rule = true;
      }
      
      // encontrar FIRST del lado derecho del no terminal.
      // A -> aBb, entonces FOLLOW(B) = FIRST(b) excepto ' '
      let right_side = &production[prod_index+1..production.len()];

      // aplicar la segunda regla solamente si no es el último elemento
      // de la producción.
      if right_side.len() > 0 {
        let right_first = self.find_first_production(&right_side.to_vec());
        for element in right_first {
          if element != "' '"  {
            if !follow.contains(&element) {
              follow.push(element);
            }
          } else {
            _use_third_rule = true;
          }
        }
      }

      // con esta condición se rompe la recursividad en caso de que la
      // producción a analizar sea la misma que la del no terminal.
      if non_terminal == &self.sides.left[right_index] {
        continue;
      }

      // se aplica la tercera regla si es necesario.
      if _use_third_rule {
        let non_terminal = self.sides.left[right_index].to_owned();
        let next_follow = self.find_follow(&non_terminal);
        for next in next_follow {
          if !follow.contains(&next) {
            follow.push(next);
          }
        }
      }
    }

    self.follows.insert(String::from(non_terminal), follow.to_owned());
    follow
  }

  /// Regresa los índices de las apariciones del no terminal en el lado
  /// izquierdo de la gramática.
  fn get_indexes_in_non_terminals(&self, non_terminal: &String) -> Vec<usize> {
      let mut indexes = vec![];

      for (index, value) in self.sides.left.iter().enumerate() {
          if non_terminal == value {
              indexes.push(index)
          }
      }
      indexes
  }

  /// Busca un no terminal del lado derecho de la gramática.
  fn find_in_right_side(&self, non_terminal: &String) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (right_index, prod) in self.sides.right.iter().enumerate() {
      for (prod_index, el) in self.side_to_prod(prod).iter().enumerate() {
        if non_terminal == el {
          result.push((right_index, prod_index));
        }
      }
    }

    result
  }

  /// Genera todas las producciones en la gramática y las guarda en un caché.
  pub fn find_all_productions(&mut self) {
    for (index, _) in self.sides.left.iter().enumerate() {
      let production = self.side_to_prod(&self.sides.right[index]);
      self.productions.insert(
        index, production.to_owned(),
      );
    }
  }

  /// Construye una sola producción de acuerdo a un índice del lado derecho.
  fn get_production(&mut self, index: usize) -> Vec<String> {
    if self.productions.contains_key(&index) {
      match self.productions.get(&index) {
        Some(production) => return production.to_owned(),
        None => {},
      }
    }
    let production = self.side_to_prod(&self.sides.right[index]);
    self.productions.insert(
      index, production.to_owned(),
    );
    
    production
  }

  /// Convierte un string a un vector de elementos (una producción).
  fn side_to_prod(&self, side: &String) -> Vec<String> {
    let elements_in_production = side.split(' ').collect::<Vec<_>>();
    let mut production = vec![];

    for element in elements_in_production {
      production.push(String::from(element));
    }

    production
  }
}
