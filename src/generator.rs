use rand::Rng;
pub fn gen_ran() -> f32 {
    let mut rng = rand::thread_rng();
    let n: f32 = rng.gen();
    n
}
