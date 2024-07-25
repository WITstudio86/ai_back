use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum Figure {
    Chat,
    Teacher,
    All,
}

impl FromStr for Figure {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chat" => Ok(Figure::Chat),
            "teacher" => Ok(Figure::Teacher),
            "all" => Ok(Figure::All),
            _ => Err("Invalid figure"),
        }
    }
}

// impl ToString for Figure {
//     fn to_string(&self) -> String {
//         match self {
//             Figure::Chat => "chat".to_string(),
//         }
//     }
// }

impl Display for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Figure::Chat => write!(f, "chat"),
            Figure::Teacher => write!(f, "teacher"),
            Figure::All => write!(f, "all"),
        }
    }
}
