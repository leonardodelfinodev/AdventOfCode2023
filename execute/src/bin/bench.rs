use took::{Timer, Took};

const RUNS: usize = 1000;

fn main() {
    println!("Benchmarking with {} runs", RUNS);

    let times: Vec<_> = execute::jobs()
        .iter()
        .map(|j| {
            (
                j.1,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j.0();
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();
    
    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("Total");
}
