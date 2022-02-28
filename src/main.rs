use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

const PR_SEATS: usize = 176;
const FPTP_SEATS: usize = 289;

#[derive(Debug)]
enum FptpDistrict {
    Hokkaido1,
    Hokkaido2,
    Hokkaido3,
    Hokkaido4,
    Hokkaido5,
    Hokkaido6,
    Hokkaido7,
    Hokkaido8,
    Hokkaido9,
    Hokkaido10,
    Hokkaido11,
    Hokkaido12,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PrDistrict {
    Hokkaido,
    Tohoku,
    KitaKanto,
    Tokyo,
    MinamiKanto,
    HokurikuShinetsu,
    Tokai,
    Kinki,
    Chugoku,
    Shikoku,
    Kyushu,
}

impl PrDistrict {
    // last modified 2017
    pub fn seats(&self) -> usize {
        match *self {
            PrDistrict::Hokkaido => 8,
            PrDistrict::Tohoku => 13,
            PrDistrict::KitaKanto => 19,
            PrDistrict::Tokyo => 17,
            PrDistrict::MinamiKanto => 22,
            PrDistrict::HokurikuShinetsu => 11,
            PrDistrict::Tokai => 21,
            PrDistrict::Kinki => 28,
            PrDistrict::Chugoku => 11,
            PrDistrict::Shikoku => 6,
            PrDistrict::Kyushu => 20,
        }
    }
}

type Party = String;
type PartyVotes = HashMap<Party, f32>;

#[derive(Debug)]
struct FptpVotes {
    // 同一政党から複数人出ることがままある
    districts: HashMap<String, Vec<FptpCandidateVotes>>,
}

impl FptpVotes {
    pub fn get_seats(&self) -> FptpSeats {
        let mut seats = FptpSeats::new();
        self.districts
            .iter()
            .map(|(_district, result)| {
                result
                    .iter()
                    .max_by(
                        |FptpCandidateVotes { votes: a, .. },
                         FptpCandidateVotes { votes: b, .. }| {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        },
                    )
                    .unwrap()
                    .party
                    .clone()
            })
            .for_each(|party| seats.add_seat(party));
        seats
    }
}

#[derive(Debug)]
struct PrVotes {
    districts: HashMap<PrDistrict, PartyVotes>,
}

impl PrVotes {
    pub fn get_seats(&self) -> PrSeats {
        let mut seats = PrSeats::new();
        for (district, dv) in &self.districts {
            let mut table = dv
                .iter()
                .map(|(party, votes)| {
                    (1..=district.seats()).map(move |i| (party, votes / i as f32))
                })
                .flatten()
                .collect::<Vec<_>>();
            table.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            table.reverse();
            table
                .into_iter()
                .take(district.seats())
                .for_each(|(p, _v)| seats.add_seat(p.to_owned()));
        }
        seats
    }
}

#[derive(Debug)]
struct Votes {
    pub fptp: FptpVotes,
    pub pr: PrVotes,
}

impl Votes {
    pub fn get_seats(&self) -> Seats {
        let fptp = self.fptp.get_seats();
        let pr = self.pr.get_seats();
        Seats { fptp, pr }
    }
}

impl Display for Votes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
struct FptpSeats {
    seats: HashMap<Party, usize>,
}

impl FptpSeats {
    pub fn new() -> Self {
        Self {
            seats: HashMap::new(),
        }
    }

    pub fn add_seat(&mut self, party: String) {
        let new = match self.seats.get(&party) {
            Some(old) => old + 1,
            None => 1,
        };
        self.seats.insert(party, new);
    }
}

#[derive(Debug)]
struct PrSeats {
    seats: HashMap<Party, usize>,
}

impl PrSeats {
    pub fn new() -> Self {
        Self {
            seats: HashMap::new(),
        }
    }

    pub fn add_seat(&mut self, party: String) {
        let new = match self.seats.get(&party) {
            Some(old) => old + 1,
            None => 1,
        };
        self.seats.insert(party, new);
    }
}

#[derive(Debug)]
struct Seats {
    fptp: FptpSeats,
    pr: PrSeats,
}

#[derive(Debug, Deserialize)]
struct VotesData {
    pr: HashMap<String, HashMap<String, f32>>,
    fptp: HashMap<String, Vec<FptpCandidateVotes>>,
}

#[derive(Debug, Deserialize)]
struct FptpCandidateVotes {
    party: String,
    votes: f32,
}

impl From<VotesData> for Votes {
    fn from(data: VotesData) -> Self {
        let pr_districts = data
            .pr
            .iter()
            .map(|(district, dv)| {
                let d = match district.as_ref() {
                    "北海道" => PrDistrict::Hokkaido,
                    "東北" => PrDistrict::Tohoku,
                    "北陸信越" => PrDistrict::HokurikuShinetsu,
                    "東海" => PrDistrict::Tokai,
                    "東京都" => PrDistrict::Tokyo,
                    "北関東" => PrDistrict::KitaKanto,
                    "南関東" => PrDistrict::MinamiKanto,
                    "近畿" => PrDistrict::Kinki,
                    "中国" => PrDistrict::Chugoku,
                    "四国" => PrDistrict::Shikoku,
                    "九州" => PrDistrict::Kyushu,
                    _ => unreachable!(),
                };
                (d, dv.clone())
            })
            .collect();

        Self {
            pr: PrVotes {
                districts: pr_districts,
            },
            fptp: FptpVotes {
                districts: data.fptp,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("./data/2021_rep.json")?;
    let data: VotesData = serde_json::from_str(&content)?;
    let votes: Votes = data.into();
    println!("{:?}", votes);
    println!("{:?}", votes.get_seats());

    Ok(())
}
