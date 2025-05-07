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

fn main() {
    arrays();
    matriz();

    println!("É fim de semana? {}\n ", eh_fim_de_semana(DiaDaSemana::Domingo));

    cores();
}
