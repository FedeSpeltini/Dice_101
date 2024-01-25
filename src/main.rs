use rand::Rng;
fn roll_dice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}


fn main() {
    println!("Hola, tira el dado!");
    loop {
        println!("Presiona una tecla para tirar el dado, Q para salir");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "Q" {
            let value = roll_dice();
            println!("Dado con nro {}", value);
        }
        else {
            break;
        }
    }
}


