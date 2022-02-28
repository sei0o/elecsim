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

#[derive(Debug, PartialEq, Eq, Hash)]
struct FptpCandidate {
    name: String,
    party: Party,
}

type Party = String;
type PartyVotes = HashMap<Party, usize>;
type FptpCandidateVotes = HashMap<FptpCandidate, usize>;

#[derive(Debug)]
struct FptpVotes {
    districts: HashMap<String, FptpCandidateVotes>,
}

impl FptpVotes {
    pub fn get_seats(&self) -> FptpSeats {
        let mut seats = FptpSeats::new();
        self.districts
            .iter()
            .map(|(_district, result)| {
                result
                    .iter()
                    .max_by_key(|(_cand, v)| *v)
                    .unwrap()
                    .0
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
                .map(|(party, votes)| (1..=district.seats()).map(move |i| (party, votes / i)))
                .flatten()
                .collect::<Vec<_>>();
            table.sort_by_key(|(_p, v)| *v);
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

fn main() {
    let fptp = FptpVotes {
        districts: [(
            "hokkaido_7".to_owned(),
            [
                (
                    FptpCandidate {
                        name: "a".to_owned(),
                        party: "cdp".to_owned(),
                    },
                    100000,
                ),
                (
                    FptpCandidate {
                        name: "b".to_owned(),
                        party: "ldp".to_owned(),
                    },
                    200000,
                ),
            ]
            .into(),
        )]
        .into(),
    };
    let pr = PrVotes {
        districts: [(
            PrDistrict::Hokkaido,
            [("cdp".to_owned(), 120000), ("ldp".to_owned(), 200000)].into(),
        )]
        .into(),
    };
    let result = Votes { fptp, pr };

    println!("{:?}", result);
    println!("{:?}", result.get_seats());
}
