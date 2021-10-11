use std::io::{self, BufRead};

/// Estructura para representar ambos lados de la gramática.
struct Sides {
    /// Lado izquierdo de el símbolo `->`.
    /// Contiene todos los items no terminales (se pueden repetir).
    left: Vec<String>,
    /// Lado derecho de el símbolo `->`. Contiene terminales y no terminales.
    right: Vec<String>,
}

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
/// 5. Se imprimen elementos terminales y no terminales en la consola.
fn main() -> io::Result<()> {
    let productions = read_productions();

    let Sides { left, right } = split_sides(productions);

    let non_terminals = get_non_terminals(&left);
    let terminals = get_terminals(&non_terminals, &right);

    println!("{:?}", get_first(
        &String::from("F"), &left, &right
    ));

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

/// Transforma un vector de producciones en dos vectores, uno para el lado
/// derecho y otro para el izquierdo.
/// 
/// `split_sides()` regresa una estructura `Sides` con los diferentes lados
///  de las producciones.
fn split_sides(productions: Vec<String>) -> Sides {
    let mut left = vec![];
    let mut right = vec![];

    for production in productions {
        let splited = production.split(" -> ").collect::<Vec<_>>();
        left.push(String::from(splited[0]));
        right.push(String::from(splited[1]));
    }

    Sides { left, right }
}

/// Filtra el lado izquierdo de la gramática y regresa un vector con todos los
/// elementos no terminales.
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

/// Filtra los elementos terminales desde el lado derecho con ayuda
/// de los no terminales.
fn get_terminals(
    non_terminals: &Vec<String>, right_side: &Vec<String>
) -> Vec<String> {
    let mut splited_right = vec![];
    // Se usa un Vector<String> en lugar de HasSet<String> porque el
    // HashSet<String> ordena de diferente manera sus elementos.
    // Se quieren obtener los elementos en orden de aparición.
    let mut terminals = vec![];

    // Ya que algunos elementos del lado derecho pueden venir algo parecido
    // a esto: `( A )`, debemos volver a separar por espacios
    // todos los elementos. 
    for line in right_side {
        for element in line.split(" ") {
            splited_right.push(String::from(element));
        }
    }

    // Al final se filtran elementos que sean `'` (ya que representan el
    // símbolo Epsilon) o cualquier no terminal.
    // Solo se mantienen elementos únivos en el vector.
    for element in splited_right {
        if element != "'" && !non_terminals.contains(&element) && !terminals.contains(&element) {
            terminals.push(element);
        }
    }

    terminals
}

fn get_first(
    non_terminal: &String, left_side: &Vec<String>, right_side: &Vec<String>,
) -> Vec<String> {
    let indexes = get_indexes(non_terminal, left_side);

    let mut first = vec![];

    for index in indexes {
        let first_in_body = &right_side[index]
            .chars().next().unwrap().to_string();

        match first_in_body.as_str() {
            "'" => first.push(String::from("' '")),
            _ => {
                if left_side.contains(first_in_body) {
                    first.extend(
                        get_first(first_in_body, left_side, right_side)
                    );
                } else {
                    first.push(String::from(first_in_body));
                }
            }
        }
    }

    first
}

fn get_indexes(terminal: &String, terminals: &Vec<String>) -> Vec<usize> {
    let mut indexes = vec![];

    for (index, value) in terminals.iter().enumerate() {
        if terminal == value {
            indexes.push(index)
        }
    }

    indexes
}
