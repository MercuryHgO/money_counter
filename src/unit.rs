use eframe::egui::TextBuffer;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::pronounce::{NumberPronouce, Triplet};


#[derive(
    Debug,
    PartialEq, Eq, PartialOrd, Ord,
    Clone, Copy
)]
pub struct Unit(pub u128);

impl NumberPronouce for Unit {
    fn into_triplets(& self) -> Vec<crate::pronounce::Triplet> {
        let mut triplets: Vec<Triplet> = Vec::new();

        let mut units_str = self.0.to_string();

        match units_str.chars().count() % 3 {
            1 => units_str.insert_str(0,"00"),
            2 => units_str.insert_str(0,"0"),
            _ => ()
        }

        let units_str_triplets: Vec<[u8;3]> = units_str
            .as_bytes()
            .chunks(3)
            .map(
                |chunk| {
                    let mut triplet = [0u8;3];
                    for (i,byte) in chunk.iter().enumerate() {
                        triplet[i] = *byte
                    };
                    triplet
                }
            )
            .rev()
            .collect();

        for (i,triplet) in units_str_triplets.iter().enumerate() {
            triplets.push(
                Triplet::new(*triplet,i)
            )
        };

        if let Some(triplet) = triplets.last_mut() {
            *triplet = triplet.into_feminie()
        }
        
        triplets

    }
}

impl std::ops::Add for Unit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Unit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let triplets = self.into_triplets();
        let mut unit_pronounce: String = "".to_string();
        if let Some(pronounce) = triplets.last() {
            unit_pronounce.push_str(pronounce.triplet_pronounce(["единица","единицы","единиц"]));
        }
        write!(f,"{} {}",self.pronounce(),unit_pronounce)
    }
}

impl From<&Unit> for Decimal {
    fn from(value: &Unit) -> Self {
        Decimal::from_parts(
            (value.0 & 0xFFFFFFFF) as u32,
            ((value.0 >> 32) & 0xFFFFFFFF) as u32,
            ((value.0 >> 64) & 0xFFFFFFFF) as u32,
            false,
        0)
    }
}

impl TextBuffer for Unit {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        let formatted = format!("{}",self.0);
        Box::leak(formatted.into_boxed_str())
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let current_string = self.0.to_string();
        let mut new_string = String::new();

        new_string.push_str(&current_string[..char_index]);
        new_string.push_str(text);
        new_string.push_str(&current_string[char_index..]);

        match u128::from_str(&new_string) {
            Ok(v) => {
                self.0 = v;
                new_string.len() - current_string.len()
            },
            Err(_) => 0,
        }
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        let mut original_str = self.0.to_string();
        original_str.delete_char_range(char_range);

        match u128::from_str(&original_str) {
            Ok(v) => self.0 = v,
            Err(_) => (),
        }
    }
}
