use std::str::FromStr;

use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString, IntoAsciiString};

use crate::rcon::{RconError, RconResult};

#[derive(Debug, Copy, Clone)]
pub enum Team {
    Neutral = 0,
    One = 1,
    Two = 2,
}

impl Team {
    pub fn to_rcon_format(self) -> AsciiString {
        (self as usize).to_string().into_ascii_string().unwrap()
    }

    pub fn from_rcon_format<'a>(ascii: &AsciiStr) -> RconResult<Team> {
        match ascii.as_str() {
            "0" => Ok(Team::Neutral),
            "1" => Ok(Team::One),
            "2" => Ok(Team::Two),
            _   => Err(RconError::protocol_msg(format!("Unknown team Id {}", ascii))),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum Squad {
    NoSquad = 0,
    Alpha = 1,
    Bravo = 2,
    Charlie = 3,
    Delta = 4,
    Echo = 5,
    Foxtrot = 6,
    Golf = 7,
    Hotel = 8,
    India = 9,
    Juliet = 10,
    Kilo = 11,
    Lima = 12,
}

impl Squad {
    /// Returns "2" for Bravo, 0 for "NoSquad", ...
    pub fn to_rcon_format(self) -> AsciiString {
        (self as usize).to_string().into_ascii_string().unwrap()
    }

    pub fn from_rcon_format(ascii: &AsciiStr) -> RconResult<Self> {
        match ascii.as_str() {
            "0" => Ok(Squad::NoSquad),
            "1" => Ok(Squad::Alpha),
            "2" => Ok(Squad::Bravo),
            "3" => Ok(Squad::Charlie),
            "4" => Ok(Squad::Delta),
            "5" => Ok(Squad::Echo),
            "6" => Ok(Squad::Foxtrot),
            "7" => Ok(Squad::Golf),
            "8" => Ok(Squad::Hotel),
            "9" => Ok(Squad::India),
            "10" => Ok(Squad::Juliet),
            "11" => Ok(Squad::Kilo),
            "12" => Ok(Squad::Lima),
            _   => Err(RconError::protocol_msg(format!("Unknown squad Id {}", ascii))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Visibility {
    All,
    Team(Team),
    Squad(Team, Squad),
    Player(AsciiString),
}

impl Visibility {
    pub fn to_rcon_format(&self) -> AsciiString {
        match self {
            Visibility::All => AsciiString::from_str("all").unwrap(),
            Visibility::Team(team) => {
                let mut ascii = AsciiString::from_str("team ").unwrap();
                ascii.extend(team.to_rcon_format().into_iter());
                ascii
            }
            Visibility::Squad(team, squad) => {
                let mut ascii = AsciiString::from_str("squad ").unwrap();
                ascii.push_str(team.to_rcon_format().as_ascii_str().unwrap());
                ascii.push(AsciiChar::Space);
                ascii.extend(squad.to_rcon_format().into_iter());
                ascii
            }
            Visibility::Player(player) => {
                let mut ascii = AsciiString::from_str("player ").unwrap();
                ascii.extend(player.into_iter());
                ascii
            }
        }
    }

    pub fn from_rcon_format(str: &AsciiStr) -> RconResult<Self> {
        let split : Vec<_> = str.split(AsciiChar::Space).collect::<Vec<_>>();
        if split.len() == 0 {
            return Err(RconError::protocol());
        }
        match split[0].as_str() {
            "all" => {
                if split.len() != 1 {
                    Err(RconError::protocol())
                } else {
                    Ok(Visibility::All)
                }
            },
            "team" => {
                if split.len() != 2 {
                    Err(RconError::protocol())
                } else {
                    Ok(Visibility::Team(Team::from_rcon_format(&split[1])?))
                }
            },
            "squad" => {
                if split.len() != 3 {
                    Err(RconError::protocol())
                } else {
                    Ok(Visibility::Squad(
                        Team::from_rcon_format(&split[1])?,
                        Squad::from_rcon_format(&split[2])?
                    ))
                }
            },
            "player" => {
                if split.len() != 2 {
                    Err(RconError::protocol())
                } else {
                    Ok(Visibility::Player(
                        split[1].into()
                    ))
                }
            }
            _ => Err(RconError::protocol()),
        }
    }
}
