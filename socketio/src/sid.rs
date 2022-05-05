use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn generate_sid() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}
