use Rnd::Rng;
use Rnd::distributions::AlphaNumeric;

fn password_generation(length:usize) -> String{
    let mut random = rand::thread_rng();
    (0..length)
        .map(|_| rng.Sample(AlphaNumeric))
        .collect::<String>()
}