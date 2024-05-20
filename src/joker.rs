use crate::{suit::Color, Cardy};

#[derive(Clone, Debug)]
pub struct Joker {
    pub color: Color,
}

impl Cardy for Joker {
    fn to_str(&self) -> String {
        "JK".to_string()
    }

    fn from_str(s: &str) -> std::prelude::v1::Result<Joker, &'static str> {
        return Ok(Joker {
            color: Color::Black,
        });
    }

    fn to_pretty(&self) -> String {
        "JK".to_string()
    }

    fn name(&self) -> String {
        "Joker".to_string()
    }
}
