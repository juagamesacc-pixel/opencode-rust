// source: src/util/slug.ts — exports: Slug (create)
//
// `create()` indexes ADJECTIVES/NOUNS with `Math.floor(Math.random() * len)`
// and joins with `-`. The Math.random equivalent is util/hash's PRNG
// (PROVISIONAL entropy note there).

const ADJECTIVES: [&str; 29] = [
    "brave", "calm", "clever", "cosmic", "crisp", "curious", "eager", "gentle", "glowing", "happy",
    "hidden", "jolly", "kind", "lucky", "mighty", "misty", "neon", "nimble", "playful", "proud",
    "quick", "quiet", "shiny", "silent", "stellar", "sunny", "swift", "tidy", "witty",
];

const NOUNS: [&str; 31] = [
    "cabin", "cactus", "canyon", "circuit", "comet", "eagle", "engine", "falcon", "forest",
    "garden", "harbor", "island", "knight", "lagoon", "meadow", "moon", "mountain", "nebula",
    "orchid", "otter", "panda", "pixel", "planet", "river", "rocket", "sailor", "squid", "star",
    "tiger", "wizard", "wolf",
];

/// source: `Slug.create()` — `"<adjective>-<noun>"`. Verbatim lists/order.
pub fn create() -> String {
    let adjective =
        ADJECTIVES[(crate::util::hash::random01() * ADJECTIVES.len() as f64).floor() as usize];
    let noun = NOUNS[(crate::util::hash::random01() * NOUNS.len() as f64).floor() as usize];
    format!("{adjective}-{noun}")
}
