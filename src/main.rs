mod grammar;
mod analyzer;

use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use grammar::{Grammar};
use analyzer::{LL1Analyzer};

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
/// 6. Si la gramática es LL(1), continúa con la evaluación de cadenas de 
/// texto, si no, se detiene la ejecución y se notifica al usuario.
/// 7. Se construye la tabla de parseo.
/// 8. Las entradas son evaluadas una por una.
/// 9. Se construye el archivo de salida.
fn main() -> io::Result<()> {
  let (productions, inputs) = read_productions();

  let mut grammar = Grammar::new(productions);
  grammar.find_non_terminals();
  grammar.find_terminals();
  grammar.find_all_productions();

  for non_terminal in grammar.non_terminals.to_owned() {
    grammar.find_single_first(&non_terminal);
    grammar.find_follow(&non_terminal);
  }

  let is_ll1 = grammar.is_ll1();

  if !is_ll1 {
    println!("No se puede analizar con LL(1)");
    return Ok(())
  }

  let mut analyzer = LL1Analyzer::new(&mut grammar);
  analyzer.build_table();

  // Crea archivo a escribir
  let timestamp = get_timestamp();
  let mut file = OpenOptions::new()
    .append(true).create(true).open(format!("{:?}.html", timestamp)).unwrap();

  // Escribe tabla a archivo
  let table_html = analyzer.get_table_as_html();
  if let Err(e) = writeln!(file, "{}", table_html) {
    eprintln!("No se pudo escribir al archivo: {}", e);
  }

  // Evalúa cada entrada e imprime resultados en el archivo HTML.
  for (index, input) in inputs.iter().enumerate() {
    let result = analyzer.eval(&input);

    if let Err(e) = writeln!(
      file,
      "<br><b>Input #{}:</b> {}", index + 1, if result {"Yes"} else {"No"},
    ) {
      eprintln!("No se pudo escribir la archivo: {}", e);
    }
  }

  println!("El resultado está en el archivo {:?}.html", timestamp);

  Ok(())
}

/// Lee de `stdin` las producciones de la gramática libre de contexto.
/// La entrada debe empezar con la cantidad de producciones y entradas a leer.
/// Las producciones deben estar en la 
/// [forma normal de Chomsky](https://en.wikipedia.org/wiki/Chomsky_normal_form)
/// 
/// # Ejemplo
/// 
/// ```txt
/// 5 4
// goal -> A
// A -> ( A )
// A -> two
// two -> a
// two -> b
// ( ( a ) )
// ( a ) )
// ( ( ( ( ( b ) ) ) ) )
// ( ( ( ( ( a b ) ) ) ) )
/// ```
pub fn read_productions() -> (Vec<String>, Vec<String>) {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut prods = Vec::new();
  let mut inputs = Vec::new();

  while let Some(line) = lines.next() {
    // Se obtiene la cantidad de producciones y cadenas a leer
    let line_result = line.unwrap();
    let splitted: Vec<&str> = line_result.trim().split(" ").collect();

    let prod_len: i32 = splitted[0].parse().unwrap();
    let inputs_len: i32 = splitted[1].parse().unwrap();

    // Se leen las producciones línea a línea
    for _ in 0..prod_len {
      let line = lines
          .next()
          .expect("No hubo una siguiente producción")
          .expect("¡Error al leer la producción!");

      // Se guarda la producción leída
      prods.push(line);
    }

    // Se leen las entradas línea a línea
    for _ in 0..inputs_len {
      let line = lines
          .next()
          .expect("No hubo una siguiente producción")
          .expect("¡Error al leer la producción!");

      // Se guarda la cadena leída
      inputs.push(line);
    }
  }

  (prods, inputs)
}

fn get_timestamp() -> Duration {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  since_the_epoch
}
