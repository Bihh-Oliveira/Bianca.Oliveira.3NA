use rand::Rng;

fn main() {
    let mut vec = Vec::new();
    
    let mut rng = rand::thread_rng();
    

    for _ in 0..20 {
        vec.push(rng.gen_range(1..=100));
    }

    // vetor original
    println!("Vetor original: {:?}", vec);

    // Ordena o vetor
    vec.sort();

    // vetor ordenado
    println!("Vetor ordenado: {:?}", vec);
}


