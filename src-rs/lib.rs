use rand::seq::SliceRandom;

pub fn choose_planet() -> String {
    let planets = vec!["Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"];
    let starter = planets.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}
