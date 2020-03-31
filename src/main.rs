#[macro_use]
extern crate clap;

mod indivisual;
mod simulator;

use simulator::Simulator;

fn main() {
    let matches = clap_app!(onemax =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg num_indivisuals: -n --("num-indivisuals") "Number of indivisuals (20)")
        (@arg gene_length: -l --("gene-length") "Length of gene (10)")
        (@arg mutation_rate: -m --("mutation-rate") "Probability that mutation occurs (0.3)")
        (@arg iterations: -i --iteration "Number of generations (20)")
    )
    .get_matches();

    let num_indivisuals = value_t!(matches, "num_indivisuals", usize).unwrap_or(20);
    let gene_length = value_t!(matches, "gene_length", usize).unwrap_or(10);
    let mutation_rate = value_t!(matches, "mutation_rate", f64).unwrap_or(0.3);
    let iteration_count = value_t!(matches, "iterations", usize).unwrap_or(20);

    let mut simulator = Simulator::new(num_indivisuals, gene_length, mutation_rate);
    simulator.run(iteration_count);
}
