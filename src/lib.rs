extern crate rand;

use rand::{thread_rng, Rng};

pub const ADJECTIVES: &'static [&'static str] = &[
    "fart",
    "weed",
    "poop",
    "felch",
    "skullfuck",
    "blaze",
    "pussy",
    "meat",
    "slippery",
    "dumb",
    "heady",
    "messy",
    "drunk",
    "blood",
    "unintended",
    "uncool",
    "fartridden",
    "decrepit",
    "justbad",
    "impressively",
    "gross",
    "pathetic",
    "floppy",
    "chrisbrown",
    "hurt",
    "clitty",
    "fragile",
    "used",
    "abused",
    "shit",
    "fuck",
    "gushy",
    "mushy",
    "squishy",
    "soggy",
    "frothy",
    "awkward",
    "shredded",
    "piss",
    "jenkem",
    "ass",
    "terrible",
    "sadistic",
    "lackluster",
    "unfortunate",
    "ebola",
    "aids",
    "massive",
    "herpes",
    "gonorrhea",
    "diarrhea",
    "chlamidia",
    "aborted",
    "fecal",
    "generic",
    "heavyset",
    "anorexic",
    "bulimic",
    "unlucky",
    "lucky",
    "horrible",
    "voluptuous",
    "punctual",
    "outofshape",
    "porous",
    "poor",
    "divorced",
    "athletic",
    "unathletic",
    "mediocre",
    "failed",
    "precocious",
    "prodigious",
    "unremarkable",
    "spoiled",
    "rotten",
    "fresh",
    "overdue",
    "underaccomplished",
    "bionic",
    "spent",
    "destined",
    "underwhelming",
    "oozing",
    "festering",
    "pissing",
    "tight",
    "splitopen",
    "depressing",
    "depressed",
    "overrated",
    "meager",
    "homely",
    "shitty",
    "shit",
    "fucky",
    "fuck",
    "misused",
    "abused",
    "recycled",
    "bleeding",
    "bloody",
    "guttural",
    "blasted",
    "cute",
    "shameless",
    "slaughtered",
    "unapologetic",
    "awful",
    "stupid",
    "dumb",
    "smelly",
    "yummy",
    "delicious",
    "powerful",
    "mouthbreathing",
    "dumbass",
    "incredible",
    "vibrating",
];

pub const NOUNS: &'static [&'static str] = &[
    "fan",
    "dude",
    "man",
    "doctor",
    "expert",
    "thug",
    "hero",
    "king",
    "queen",
    "idiot",
    "queef",
    "muscles",
    "splatter",
    "satan",
    "worshipper",
    "virgin",
    "boy",
    "girl",
    "badbitch",
    "sack",
    "loser",
    "cultworshipper",
    "astronaut",
    "playboy",
    "pounder",
    "asstronaut",
    "blaster",
    "penis",
    "cock",
    "culo",
    "puta",
    "piss",
    "ass",
    "dopeboy",
    "buttfucker",
    "douche",
    "pissboy",
    "cumstain",
    "ebola",
    "aids",
    "diarrhea",
    "trump",
    "chlamidia",
    "abortion",
    "fetus",
    "athlete",
    "failure",
    "underwhelmer",
    "homie",
    "shit",
    "fuck",
    "fuckboy",
    "fuckgirl",
    "fuckperson",
    "fiend",
    "dominator",
    "cockboy",
    "frothball",
    "weiner",
    "thunderlips",
    "meatdrapes",
    "thunderlips",
    "beefcurtains",
    "kumquat",
    "obama",
    "republican",
    "filler",
    "democrat",
    "vibrator",
    "dildo",
    "babe",
    "bebe",
    "baby",
    "drapes",
    "schlong",
    "beershits",
    "pocket",
];

/// Holds settings that will be used when `generate` is called.
///
/// # Example
///
/// ```
/// use awful_idea_name::AwfulIdeaName;
///
/// let a = AwfulIdeaName {
///     adjectives: &["smelly", "yummy"],
///     nouns: &["weiner", "fiend"],
/// };
/// ```
#[derive(Debug)]
pub struct AwfulIdeaName<'a> {
    pub adjectives: &'a [&'a str],
    pub nouns: &'a [&'a str],
}

impl<'a> Default for AwfulIdeaName<'a> {
    /// Creates a new `AwfulIdeaName` with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use awful_idea_name::AwfulIdeaName;
    ///
    /// let a = AwfulIdeaName::default();
    /// ```
    fn default() -> Self {
        AwfulIdeaName {
            adjectives: ADJECTIVES,
            nouns: NOUNS,
        }
    }
}

impl<'a> AwfulIdeaName<'a> {
    /// Creates a new `AwfulIdeaName` with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use awful_idea_name::AwfulIdeaName;
    ///
    /// let a = AwfulIdeaName::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Generates a random awful heroku-like idea name.
    ///
    /// # Example
    ///
    /// ```
    /// use awful_idea_name::AwfulIdeaName;
    ///
    /// let a = AwfulIdeaName::new();
    /// println!("{:?}", a.generate());
    /// ```
    pub fn generate(&self) -> String {
        // Define some RNG.
        let mut rng = thread_rng();

        // Pick a random adjective.
        let adjective = self.adjectives[rng.gen_range(0, self.adjectives.len())];

        // Pick a random noun.
        let noun = self.nouns[rng.gen_range(0, self.nouns.len())];

        // Create an empty string to hold a random number.
        let mut number = String::with_capacity(4);

        // Generate a random number between 0 and 9. Do this four
        // times; each time pushing the result into `number`.
        for _ in 0..4 {
            number.push(std::char::from_digit(rng.gen_range(0, 10), 10).unwrap());
        }

        // Return a string of the whole random project name.
        vec![adjective, noun, &number].join("-")
    }
}
