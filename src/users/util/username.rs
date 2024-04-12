use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn new() -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    random_string
}
