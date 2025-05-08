fn main() {
    arrays();
    matriz();

    println!("É fim de semana? {}\n ", eh_fim_de_semana(DiaDaSemana::Domingo));

    cores();
    conteudo_opcional();
    vectors();
    conta_corrente();
}

fn arrays(){
    let notas:[f32; 4] = [6.5; 4];
    let indice:usize = 0;

    print!("{}\n", notas[indice]);

    for indice in 0..notas.len() {
        print!("Nota {} = {}\n", indice + 1, notas[indice]);
    }
}

fn matriz(){
    let matriz: [[f32; 3]; 2] = [
        [1.2, 3.4, 5.6],
        [9.8, 7.6, 5.4]
    ];

    for linha in matriz {
        for item in linha {
            print!("{}  ", item);
        }
        print!("\n");
    }
}

// Remove warnings about unused pieces of code
#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sábado
}

fn eh_fim_de_semana(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Sábado | DiaDaSemana::Domingo => true,
        _ => false
    }
}

// Remove warnings about unused pieces of code
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    CYMK{cyan: u8, yellow: u8, magenta :u8, black: u8}
}

fn cores(){
    let color = Color::CYMK { cyan: 0, yellow: 1, magenta: 0, black: 255 };

    println!("Cor: {}", match color {
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::RGB(0, 0, 0) | Color::CYMK { cyan: 0, yellow: 0, magenta: 0, black: 255 } => "Preto",
        Color::RGB(_, _, _) => "Desconhecido",
        Color::CYMK { cyan: _, yellow: _, magenta: _, black: _ } => "CYMK Desconhecido"
    });
}

fn conteudo_opcional(){
    let file_content = read_file(String::from("./"));

    match &file_content{
        // MyOption::Some(value) => println!("{}", value),
        Some(value) => println!("{}", value),
        // MyOption::None => println!("File does not exists!")
        None => println!("File does not exists!")
    };

    println!("{:?}", file_content);

    //implementation if IF LET
    // if let MyOption::Some(value) = file_content {
    if let Some(value) = file_content {
        println!("Implementando if let: {}\n", value);
    }
}

//Generics
#[allow(dead_code)]
enum MyOption<T> {
    Some(T),
    None
}

#[allow(dead_code)]
enum MyResult<S, E> {
    Ok(S),
    Err(E)
}
// fn read_file(path_to_file: String) -> MyOption<String> {
fn read_file(path_to_file: String) -> Option<String> {
    if path_to_file.len() != 0 {
        // MyOption::Some(String::from("Conteúdo do arquivo"))
        Some(String::from("Conteúdo do arquivo"))
    } else {
        // MyOption::None
        None
    }
}

fn vectors() {
    // let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5];
    let mut notas: Vec<f32> = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);
    println!("Capacity: {}", notas.capacity());
    println!("{:?}", notas);

    notas.push(6.8);
    println!("Capacity: {}", notas.capacity());
    println!("{:?}", notas);

    println!("Nota 1: {}", notas[0]);

    println!("Nota 6: {}", match notas.get(5) {
        Some(n) => *n,
        None => 0.0
    });

    // popping values from the vector using WHILE LET
    // while let Some(nota)= notas.pop() {
    //     println!("Nota removida: {}", nota);
    //     println!("{:?}", notas);
    // }

    // Looping thru the vector with FOR LET
    for nota in &notas {
        println!("Nota: {}", nota);
    }
    println!("{:?}", notas);
}

// FINALLY SOME STRUCTS
struct Account {
    holder: String,
    balance: f64
}

fn conta_corrente() {
    let conta: Account = Account {
        holder: String::from("Herick Vinicius"),
        balance: 100.0
    };
    // let account_holder: String = String::from("Herick Vinicius");
    // let account_balance: f64 = 100.0;

    println!("Dados da conta: Titular = {} | Saldo = {}",
        conta.holder, conta.balance);
}