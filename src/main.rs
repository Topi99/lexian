use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let prods = read_productions();

    println!("{:?}", prods);

    Ok(())
}

fn read_productions() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut prods = Vec::new();

    while let Some(line) = lines.next() {
        let length: i32 = line.unwrap().trim().parse().unwrap();

        for _ in 0..length {
            let line = lines
                .next()
                .expect("No hubo una siguiente producción")
                .expect("¡Error al leer la producción!");

            prods.push(line);
        }
    }

    prods
}
