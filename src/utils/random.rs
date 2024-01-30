use rand::Rng;

pub fn random_string(n: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_chars: Vec<char> = (0..n)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    random_chars.into_iter().collect()
}
