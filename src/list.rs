#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use std::io::Cursor;

use crate::pokemon::Region;
use bimap::BiHashMap;

/// A parsed representation of `names.csv`.
///
/// Used to derive filenames from Pokedex ID's, and to
/// format image filenames back into proper pokemon names.
pub struct List {
    /// The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,
}

impl List {
    /// Reads a new [`List`] from `data/names.csv`.
    pub fn read() -> Self {
        const FILE: &'static str = include_str!("../data/names.csv");

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(FILE));

        const CAPACITY: usize = 1000;

        let mut ids = BiHashMap::with_capacity(CAPACITY);
        let mut names = Vec::with_capacity(CAPACITY);

        for (i, entry) in reader.deserialize().enumerate() {
            let record: (String, String) = entry.unwrap();

            ids.insert(i, record.1);
            names.push(record.0);
        }

        Self { ids, names }
    }

    /// Takes a filename and looks up the proper display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use pokeget::list::List;
    /// let list = List::read();
    /// assert_eq!(list.format_name("mr-mime"), "Mr. Mime")
    /// ```
    pub fn format_name(&self, filename: &str) -> String {
        let Some(id) = self.ids.get_by_right(filename) else {
            return filename.to_owned();
        };

        let Some(name) = self.names.get(*id) else {
            return filename.to_owned();
        };

        name.clone()
    }

    /// Gets a pokemon filename by a Dex ID.
    pub fn get_by_id(&self, id: usize) -> Option<&String> {
        self.ids.get_by_left(&id)
    }

    /// Gets a random pokemon & returns it's filename.
    pub fn random(&self) -> String {
        let idx = rand::random_range(0..self.ids.len());
        self.ids.get_by_left(&idx).unwrap().clone()
    }

    /// Gets a random pokemon by region
    pub fn get_by_region(&self, region: Region) -> String {
        let region = match region {
            Region::Kanto => 0..=151,
            Region::Johto => 152..=251,
            Region::Hoenn => 252..=386,
            Region::Sinnoh => 387..=493,
            Region::Unova => 494..=649,
            Region::Kalos => 650..=721,
            Region::Alola => 722..=809,
            Region::Galar => 810..=905,
        };

        let idx = rand::random_range(region);
        self.ids.get_by_left(&idx).unwrap().clone()
    }

    pub fn random_mega(&self) -> String {
        const MEGA_POKEMON: &[&str] = &[
            "venusaur-mega", "charizard-mega-x", "charizard-mega-y",
            "blastoise-mega", "alakazam-mega", "gengar-mega",
            "kangaskhan-mega", "pinsir-mega", "gyarados-mega",
            "aerodactyl-mega", "mewtwo-mega-x", "mewtwo-mega-y",
            "ampharos-mega", "scizor-mega", "heracross-mega",
            "houndoom-mega", "tyranitar-mega", "blaziken-mega",
            "gardevoir-mega", "mawile-mega", "aggron-mega",
            "medicham-mega", "manectric-mega", "banette-mega",
            "absol-mega", "garchomp-mega", "lucario-mega",
            "abomasnow-mega", "beedrill-mega", "pidgeot-mega",
            "slowbro-mega", "steelix-mega", "sceptile-mega",
            "swampert-mega", "sableye-mega", "sharpedo-mega",
            "camerupt-mega", "altaria-mega", "glalie-mega",
            "salamence-mega", "metagross-mega", "latias-mega",
            "latios-mega", "rayquaza-mega", "lopunny-mega",
            "gallade-mega", "audino-mega", "diancie-mega",
        ];
        let idx = rand::random_range(0..MEGA_POKEMON.len());
        MEGA_POKEMON[idx].to_string()
    }
}
