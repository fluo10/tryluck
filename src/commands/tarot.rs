use rand::RngExt as _;
use rand::seq::SliceRandom as _;
use serde::Serialize;

/// Output case format for card names.
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Case {
    /// Proper display name, e.g. "The Fool"
    Proper,
    /// Snake case identifier, e.g. "the_fool"
    Snake,
}

struct Card {
    snake: &'static str,
    proper: &'static str,
}

impl Card {
    fn format(&self, case: Case) -> &'static str {
        match case {
            Case::Proper => self.proper,
            Case::Snake => self.snake,
        }
    }
}

const CARDS: &[Card] = &[
    Card { snake: "the_fool",           proper: "The Fool"           },
    Card { snake: "the_magician",       proper: "The Magician"       },
    Card { snake: "the_high_priestess", proper: "The High Priestess" },
    Card { snake: "the_empress",        proper: "The Empress"        },
    Card { snake: "the_emperor",        proper: "The Emperor"        },
    Card { snake: "the_hierophant",     proper: "The Hierophant"     },
    Card { snake: "the_lovers",         proper: "The Lovers"         },
    Card { snake: "the_chariot",        proper: "The Chariot"        },
    Card { snake: "strength",           proper: "Strength"           },
    Card { snake: "the_hermit",         proper: "The Hermit"         },
    Card { snake: "wheel_of_fortune",   proper: "Wheel of Fortune"   },
    Card { snake: "justice",            proper: "Justice"            },
    Card { snake: "the_hanged_man",     proper: "The Hanged Man"     },
    Card { snake: "death",              proper: "Death"              },
    Card { snake: "temperance",         proper: "Temperance"         },
    Card { snake: "the_devil",          proper: "The Devil"          },
    Card { snake: "the_tower",          proper: "The Tower"          },
    Card { snake: "the_star",           proper: "The Star"           },
    Card { snake: "the_moon",           proper: "The Moon"           },
    Card { snake: "the_sun",            proper: "The Sun"            },
    Card { snake: "judgement",          proper: "Judgement"          },
    Card { snake: "the_world",          proper: "The World"          },
];

#[derive(Debug, Serialize)]
pub struct TarotDraw {
    pub card: &'static str,
    pub orientation: &'static str,
}

/// Draw `count` cards without replacement (capped at deck size).
pub fn draw(count: u32, case: Case) -> Vec<TarotDraw> {
    let mut rng = rand::rng();
    let count = (count.max(1) as usize).min(CARDS.len());

    let mut indices: Vec<usize> = (0..CARDS.len()).collect();
    indices.shuffle(&mut rng);

    indices[..count]
        .iter()
        .map(|&i| TarotDraw {
            card: CARDS[i].format(case),
            orientation: if rng.random_bool(0.5) { "upright" } else { "reversed" },
        })
        .collect()
}

pub fn run(count: u32, json: bool, case: Case) {
    let draws = draw(count, case);
    if json {
        println!("{}", serde_json::to_string_pretty(&draws).unwrap());
    } else {
        for d in &draws {
            if d.orientation == "upright" {
                println!("{}", d.card);
            } else {
                // Reversed cards are displayed as reversed characters
                println!("{}", d.card.chars().rev().collect::<String>());
            }
        }
    }
}
