echo 'mutation_rate,iteraion,population_size,fitness' > result.csv
# for i in `seq 1 100`
# do
# done
cargo run -- -n 50 -l 100 --am 0 1 0.05 >> result.csv
