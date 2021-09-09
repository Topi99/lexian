use std::io::{self, BufRead};

struct Sides {
    left: Vec<String>,
    right: Vec<String>,
}

fn main() -> io::Result<()> {
    let productions = read_productions();

    let Sides { left, right } = split_sides(productions);

    let non_terminals = get_non_terminals(left);
    let terminals = get_terminals(&non_terminals, right);

    println!("Terminal: {}", terminals.join(", "));
    println!("Non terminal: {}", non_terminals.join(", "));

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

fn get_non_terminals(left_side: Vec<String>) -> Vec<String> {
    let mut non_terminals = vec![];

    for element in left_side {
        if !non_terminals.contains(&element) {
            non_terminals.push(element);
        }
    }

    non_terminals
}

fn get_terminals(non_terminals: &Vec<String>, right_side: Vec<String>) -> Vec<String> {
    let mut splited_right = vec![];
    let mut terminals = vec![];

    for line in right_side {
        for element in line.split(" ") {
            splited_right.push(String::from(element));
        }
    }

    for element in splited_right {
        if element != "'" && !non_terminals.contains(&element)  && !terminals.contains(&element) {
            terminals.push(element);
        }
    }

    terminals
}
