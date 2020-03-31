use rand::Rng;

#[derive(Clone, Debug)]
pub struct Indivisual {
    pub gene: Vec<u8>,
    pub evaluation: f64,
}

impl Indivisual {
    pub fn new(gene_length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let gene: Vec<u8> = (0..gene_length).map(|_| {
            rng.gen_range(0, 2)
        }).collect();
        Self {
            gene,
            evaluation: 0_f64,
        }
    }
}
