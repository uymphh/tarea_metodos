use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn take_string_as_vector() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}

fn imc() {
    clear_screen();
    println!("Dime tu peso: ");
    let peso: f64 = take_int() as f64;
    println!("Dime tu altura: ");
    let altura: f64 = take_int() as f64;
    let imc: f64 = peso / ((altura / 100.0) * (altura / 100.0));
    println!("{imc}")
}

fn palindromo() {
    clear_screen();
    println!("Dime tu palabra para saber si es palindromo: ");
    let palabra = take_string_as_vector();
    let mut palindromo = true;
    let mut l: usize = 0;
    let mut r = palabra.len() - 1;
    while l < r {
        if palabra[r] != palabra[l] {
            palindromo = false;
        }
        l += 1;
        r -= 1;
    }
    if palindromo {
        println!("Si es un palindromo");
    } else {
        println!("No es un palindromo");
    }
}
fn escalera_no_recursivo() {
    clear_screen();
    println!("Dime que escalon calcular");
    let escalon = take_int();
    let mut swap;
    let mut a = 1;
    let mut b = 1;
    for _ in 2..=escalon {
        swap = a + b;
        a = b;
        b = swap;
    }
    println!("Hay {} formas de subir {} escalones", b, escalon)
}
fn escalera_recursivo(a: usize) -> usize {
    match a {
        1 | 0 => 1,
        2 => 2,
        _ => escalera_recursivo(a - 1) + escalera_recursivo(a - 2),
    }
}

fn busqueda_de_telefono() {
    clear_screen();
    println!("Que nombre busco?: ");
    let nombre = take_string_as_vector();
    let lista: Vec<&str> = vec![
        "Adrian",
        "Alejandro",
        "Ana",
        "Andrea",
        "Andres",
        "Angel",
        "Antonella",
        "Beatriz",
        "Camila",
        "Carolina",
        "Carlos",
        "Clara",
        "Daniela",
        "David",
        "Diana",
        "Diego",
        "Elena",
        "Emiliano",
        "Felipe",
        "Fernanda",
        "Gabriel",
        "Gabriela",
        "Isabella",
        "Javier",
        "Jose",
        "Juan",
        "Julieta",
        "Leonardo",
        "Lucia",
        "Luis",
        "Manuel",
        "Mariana",
        "Martin",
        "Mateo",
        "Mia",
        "Natalia",
        "Nicolas",
        "Paula",
        "Rafael",
        "Regina",
        "Renata",
        "Rodrigo",
        "Samuel",
        "Santiago",
        "Sebastian",
        "Sofia",
        "Tomas",
        "Valentina",
        "Valeria",
        "Victoria",
        "Ximena",
    ];
    let mut l = 0;
    let mut r = lista.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        let mut diff = false;
        let compara: Vec<char> = lista[m].chars().collect();
        let min = if nombre.len() < compara.len() {
            nombre.len()
        } else {
            compara.len()
        };
        let mut i = 0;
        while i < min {
            if compara[i] < nombre[i] {
                l = m + 1;
                diff = true;
                break;
            } else if compara[i] > nombre[i] {
                r = m - 1;
                diff = true;
                break;
            }
            i += 1;
        }
        if diff {
            continue;
        }
        if compara.len() == nombre.len() {
            println!("Esta!!");
            return;
        } else if compara.len() < nombre.len() {
            l = m + 1;
            break;
        } else if compara.len() > nombre.len() {
            r = m - 1;
            break;
        }
    }
    println!("No esta :c");
}
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn clear_screen_enter() {
    println!("Presione enter para continuar...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    print!("\x1B[2J\x1B[H");
}
fn main() {
    loop {
        clear_screen_enter();
        println!("==Menu de Codigos==");
        println!("==1...........IMC==");
        println!("==2....PALINDROMO==");
        println!("==3...ESCALERA_IT==");
        println!("==4...ESCALERA_RE==");
        println!("==5....DIRECTORIO==");
        println!("==6.........SALIR==");
        println!("\nElige uno: ");
        let num = take_int();
        match num {
            1 => imc(),
            2 => palindromo(),
            3 => escalera_no_recursivo(),
            4 => {
                clear_screen();
                println!("Dime que escalon calcular");
                let escalon = take_int();
                let formas = escalera_recursivo(escalon);
                println!("Hay {} formas de subir {} escalones", formas, escalon)
            }
            5 => busqueda_de_telefono(),
            6 => return,
            _ => println!("error"),
        }
    }
}
