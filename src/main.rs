mod grammar;

use std::io::{self, BufRead};
use grammar::grammar::{Grammar};

/// La función principal que será llamada al ejecutar el programa.
/// 
/// Este es el algoritmo en su mas alto nivel:
/// 
/// 1. Se leen las producciones línea por línea de un archivo.
/// 2. Se se extraen los diferentes lados de las producciones: derecho e
/// izquierdo.
/// 3. Del lado izquierdo se obtienen los elementos no terminales.
/// 4. Del lado derecho se eliminan elementos no terminales y se obtienen
/// terminales.
/// 5. Por cada no terminal, se obtienen los conjuntos FIRST y FOLLOW de cada
/// non terminal y se imprimen en la consola.
/// 5. Se imprime si la gramática es LL(1).
fn main() -> io::Result<()> {
  let productions = read_productions();

  let mut grammar = Grammar::new(productions);
  grammar.find_non_terminals();
  grammar.find_terminals();
  grammar.find_all_productions();

  for non_terminal in grammar.non_terminals.to_owned() {
    println!(
      "{} => FIRST = {{{}}}, FOLLOW = {{{}}}",
      non_terminal,
      grammar.find_single_first(&non_terminal).join(", "),
      grammar.find_follow(&non_terminal).join(", "),
    );
  }

  println!("LL(1)? {}", if grammar.is_ll1() { "Yes" } else { "No" });

  Ok(())
}

/// Lee de `stdin` las producciones de la gramática libre de contexto.
/// La entrada debe empezar con la cantidad de producciones a leer.
/// Las producciones deben estar en la 
/// [forma normal de Chomsky](https://en.wikipedia.org/wiki/Chomsky_normal_form)
/// 
/// # Ejemplo
/// 
/// ```txt
/// 5
/// goal -> A
/// A -> ( A )
/// A -> two
/// two -> a
/// two -> b
/// ```
pub fn read_productions() -> Vec<String> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut prods = Vec::new();

  while let Some(line) = lines.next() {
    // Se obtiene la cantidad de producciones a leer
    let length: i32 = line.unwrap().trim().parse().unwrap();

    // Se leen las producciones línea a línea
    for _ in 0..length {
      let line = lines
          .next()
          .expect("No hubo una siguiente producción")
          .expect("¡Error al leer la producción!");

      // Se guarda la producción leída
      prods.push(line);
    }
  }

  prods
}
