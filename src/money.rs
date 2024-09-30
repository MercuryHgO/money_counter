use std::{str::FromStr, u128};


use eframe::egui;
use egui::TextBuffer;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{pronounce::{NumberPronouce, Triplet}, unit::Unit};

#[derive(Debug,
    PartialEq, Eq, PartialOrd, Ord,
    Clone, Copy)]
pub struct Money(pub Decimal);

type MoneyDecomposed = (u128,u8);

impl Money {
    pub fn abs(&self) -> Self {
        Money(self.0.abs())
    }

    fn decompose(&self) -> MoneyDecomposed {
        let unpacked = self.0.fract().unpack();
        let unpacked_trunc = self.0.trunc().unpack();

        let rubles: u128 = ((unpacked_trunc.hi as u128) << 64) |
            ((unpacked_trunc.mid as u128) << 32) |
            ((unpacked_trunc.lo) as u128);
        let mut kopek: u8 = unpacked.lo as u8;
        if unpacked.scale == 1 { kopek *= 10; }

        (rubles,kopek)
    }
}

impl NumberPronouce for MoneyDecomposed {
    fn into_triplets(&self) -> Vec<crate::pronounce::Triplet> {
        let mut triplets: Vec<Triplet> = Vec::new();

        let mut rubles_str = self.0.to_string();

        match rubles_str.chars().count() % 3 {
            1 => rubles_str.insert_str(0,"00"),
            2 => rubles_str.insert_str(0,"0"),
            _ => ()
        }

        let mut kopek_slice: [u8;3] = [0u8;3];
        let mut kopek_string = "0".to_string();

        match self.1 {
            1..10 => { kopek_string.push('0'); kopek_string.push_str(&self.1.to_string())},
            10..100 => { kopek_string.push_str(&self.1.to_string())},
            _ => ()
        }

        for (i,char) in kopek_string.into_bytes().iter().enumerate() {
            kopek_slice[i] = *char;
        }
        

        let rubles_str_triplets: Vec<[u8;3]> = rubles_str
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

        for (i,triplet) in rubles_str_triplets.iter().enumerate() {
            triplets.push(
                Triplet::new(*triplet,i)
            )
        };

              
        triplets.push(
            Triplet::new(
                kopek_slice, 0 )
        );

        triplets
    }

    fn pronounce(&self) -> String {
        let mut result = String::new();
        let mut kopek_result = String::new();

        let mut triplets = self.into_triplets();
        if self.1 != 0 {
            if let Some(kopek) = &triplets.pop() {
                kopek_result.push_str(&kopek.into_feminie().pronounce());
                kopek_result.push_str(
                    &kopek.triplet_pronounce(
                        ["копейка", "копейки", "копеек"]
                    )
                );
            };
        }

        triplets
            .iter()
            .rev()
            .for_each( |triplet| {
                result.push_str(&triplet.pronounce());
                result.push(' ');
            });

        if let Some(triplet) = triplets.first() {
            result.push_str(
                triplet.triplet_pronounce(
                    ["рубль", "рубля", "рублей"]
                )
            );
        }

        result.push(' ');

        result.push_str(&kopek_result);


        result
    }

    
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Money {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Div for Money {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Add<Unit> for Money {
    type Output = Self;

    fn add(self, rhs: Unit) -> Self::Output {
        self + self * rhs
    }
}

impl std::ops::Sub<Unit> for Money {
    type Output = Self;

    fn sub(self, rhs: Unit) -> Self::Output {
        self - self * rhs
    }
}

impl std::ops::Mul<Unit> for Money {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self::Output {
        Self(self.0 * Decimal::from_u128(rhs.0).unwrap())
    }
}

impl std::ops::Div<Unit> for Money {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self::Output {
        Self(self.0 / Decimal::from_u128(rhs.0).unwrap())
    }
}

impl TryFrom<&str> for Money {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let number = Decimal::from_str_exact(&value)?;
        Ok(number.try_into()?)
    }
}

impl TryFrom<Decimal> for Money {
    type Error = Error;

    fn try_from(value: Decimal) -> Result<Self,Self::Error> {
        let unpacked = value.unpack();

        if unpacked.scale > 2 { return Err(Error::KopekDigitsTooBig); };
        
        Ok(Money(value))
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.decompose().pronounce())
    }
}

impl TextBuffer for Money {
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

        if char_index > current_string.len() {
            return 0;
        }

        if char_index == current_string.len() && current_string.len() > 3 && &current_string[current_string.len() - 2..] == ".0" {
            new_string.push_str(&current_string[..char_index-2]);
            new_string.push_str(".");
            new_string.push_str(text);
        } else {
            new_string.push_str(&current_string[..char_index]);
            new_string.push_str(text);
            new_string.push_str(&current_string[char_index..]);
        }
        if text == "." && char_index == current_string.len() {
            new_string.push_str("0");
        }

        match Money::try_from(new_string.as_str()) {
            Ok(n) => {
                self.0 = n.0;
                new_string.len() - current_string.len()
            },
            Err(_) => 0,
        }

    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        let mut current_string = self.0.to_string();
        current_string.delete_char_range(char_range);

        match Decimal::from_str(&current_string) {
            Ok(n) => self.0 = n,
            Err(_) => (),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    KopekDigitsTooBig,
    RustDecimalError(rust_decimal::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RustDecimalError(c) => std::fmt::Display::fmt(c,f),
            Error::KopekDigitsTooBig => write!(f,"Неверно указано количество копеек, правильное значение: [рубли].(1-99)"),
        }
    }
}

impl From<rust_decimal::Error> for Error {
    fn from(value: rust_decimal::Error) -> Self {
        Error::RustDecimalError(value)
    }
}

impl std::error::Error for Error { }

