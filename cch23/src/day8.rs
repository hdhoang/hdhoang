//! 🎄 Day 8: PokéPhysics

//! In the heart of the North Pole, Santa's workshop buzzes with a new
//! type of magic. A portal has opened, and Pokémon from another world
//! have tumbled into the snow-dusted realm of elves and
//! reindeer. Santa, ever the innovator, sees an opportunity: why not
//! enlist these charming creatures in his annual gift-giving
//! campaign?

//! But before the sleigh bells ring and the Pokémon can join the
//! flight, Santa needs to understand their unique properties and
//! figure out how many can fit into his sleigh, given their weight.

use axum::extract::Path;

/// ⭐ Task 1: IT'S PIKACHU!

#[derive(serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Pokemon {
    weight: u32,
}
/// given a pokédex number, responds with the corresponding Pokémon's weight in kilograms as a number in plain text.
pub async fn weight(Path(pokedex): Path<usize>) -> String {
    tokio::task::spawn_blocking(move || get_pokemon_by_pokedex(pokedex))
        .await
        .expect("http call")
        .map(|p| p.weight / 10)
        .expect("pokemon data")
        .to_string()
}

fn get_pokemon_by_pokedex(pokedex: usize) -> Result<Pokemon, reqwest::Error> {
    const POKE_API: &'static str = "https://pokeapi.co/api/v2/pokemon";
    let pokemon: Pokemon = reqwest::blocking::get(format!("{POKE_API}/{pokedex}"))?.json()?;
    Ok(pokemon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "hitting network"]
    fn check_442_weight() {
        assert_eq!(
            1080,
            get_pokemon_by_pokedex(442).expect("getting 442").weight
        )
    }

    #[test]
    fn drop_pikachu() {
        const PIKACHU_WEIGHT_KG: u32 = 6;
        assert!(84.10707 - dbg!(calculate_momentum(PIKACHU_WEIGHT_KG)) < 0.001)
    }
}

/// 🎁 Task 2: That's gonna leave a dent (160 bonus points)

/// Once the Pokémon's weight is retrieved, Santa needs you to
/// calculate the momentum it would have at the time of impact with
/// the floor if dropped from a 10-meter high chimney (so that he
/// knows if he needs to climb down or if he can just drop it).

/// Keep in mind that the gravity of Earth that Santa has in his
/// physics book was measured close to the North Pole. This could
/// explain why his calculations are a bit off sometimes, but he still
/// wants you to use it.

/// The momentum, measured in Newton-seconds, signifies the force the
/// Pokémon would exert upon meeting the floor beneath the 10-meter
/// high chimney.

/// The GET endpoint /8/drop/<pokedex_number> shall respond with a
/// plain text floating point number.

pub async fn drop(Path(pokedex): Path<usize>) -> String {
    let pokemon_weight_kg = tokio::task::spawn_blocking(move || get_pokemon_by_pokedex(pokedex))
        .await
        .expect("http call")
        .map(|p| p.weight / 10)
        .expect("pokemon data");
    dbg!(format!("{}", calculate_momentum(pokemon_weight_kg)))
}

/// Use gravitational acceleration g = 9.825 m/s². Ignore air
/// resistance.
const GRAV_ACCEL: f64 = 9.825;

/// Chimney height
const DROP_HEIGHT: f64 = 10.0;

fn calculate_momentum(weight_kg: u32) -> f64 {
    let time = ((2.0 * DROP_HEIGHT) / GRAV_ACCEL).sqrt();
    let terminal_velocity = time * GRAV_ACCEL;
    let momentum = terminal_velocity * weight_kg as f64;
    momentum
}
