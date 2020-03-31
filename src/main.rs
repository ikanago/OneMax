mod indivisual;
use indivisual::Indivisual;

fn main() {
    let simulator = Simulator::new(10, 10, 0_f64);
    println!("{:?}", simulator);
}

#[derive(Clone, Debug)]
struct Simulator {
    current_generation: usize,
    mutate_rate: f64,
    indivisuals: Vec<Indivisual>,
}

impl Simulator {
    fn new(num_indivisuals: usize, gene_length: usize, mutate_rate: f64,) -> Self {
        let indivisuals: Vec<Indivisual> = (0..num_indivisuals).map(|_| {
            Indivisual::new(gene_length)
        }).collect();
        Self {
            current_generation: 1,
            mutate_rate,
            indivisuals
        }
    }
}
