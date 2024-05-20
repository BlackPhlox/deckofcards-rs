use std::fmt;

use crate::{
    suit::{self, Color},
    DisplayCard,
};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Joker {
    pub color: Color,
}

impl fmt::Display for Joker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "pretty")]
        let str = self.to_pretty();
        #[cfg(not(feature = "pretty"))]
        let str = self.to_str();
        write!(f, "{}", str)
    }
}

impl DisplayCard for Joker {
    fn to_str(&self) -> String {
        "JK".to_string()
    }

    fn from_str(s: &str) -> std::prelude::v1::Result<Joker, &'static str> {
        if s.len() < 1 || s.len() > 3 {
            return Err("String is wrong length");
        }

        let s = s.to_string();
        let mut i = s.chars();
        let c1 = i.next().unwrap();
        let c2 = i.next().unwrap();
        let c3 = i.next().unwrap();

        let mut c2_3 = c2.to_string();
        c2_3.push(c3);

        if c1.eq_ignore_ascii_case(&'B') {
            if c2_3.eq_ignore_ascii_case("JK") {
                return Ok(Joker {
                    color: Color::Black,
                });
            }
        }

        if c1.eq_ignore_ascii_case(&'R') {
            if c2_3.eq_ignore_ascii_case("JK") {
                return Ok(Joker { color: Color::Red });
            }
        }

        if c1.to_string().eq_ignore_ascii_case("JK") {
            return Ok(Joker::default());
        }

        // Test rank / suit

        Err("Invalid string")
    }

    #[cfg(feature = "pretty")]
    fn to_pretty(&self) -> String {
        use colored::Colorize;

        use crate::GRAY;

        format!(
            "{}",
            match self.color {
                suit::Color::Black => "JK".to_string().as_str().custom_color(GRAY),
                suit::Color::Red => "JK".to_string().as_str().red(),
            }
        )
    }

    fn name(&self) -> String {
        "Joker".to_string()
    }
}
