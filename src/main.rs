use rand::Rng;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..alphabet.len());

    let random_letter = alphabet.chars().nth(random_index).unwrap();

    println!("A letra sorteada é: {}", random_letter);
}
