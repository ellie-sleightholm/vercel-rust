use rand::seq::SliceRandom;

/// Randomly selects a planet from the list of known planets in the solar system.
///
/// # Returns
///
/// Returns a string representing the name of the chosen planet.
///
/// # Examples
///
/// ```
/// use crate::choose_planet;
///
/// let planet = choose_planet();
/// println!("Selected planet: {}", planet);
/// ```
pub fn choose_planet() -> String {
    let planets = vec!["Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"];
    let starter = planets.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}
