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

fn main() {
    arrays();
    matriz();
}
