use std::{char, str::FromStr};

use eframe::egui;
use egui::{FontData, TextBuffer};
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(
    Debug,
    PartialEq, Eq, PartialOrd, Ord,
    Clone, Copy
)]
pub struct Unit(pub u128);

impl Unit {
    pub fn pronounce(&self) -> String {
        let pronounce: &'static str = match self.0 % 10 {
            1 if self.0 % 100 != 11 => "единицу",
            2..=4 if !(self.0 % 100 >= 12 && self.0 % 100 <= 14) => "единицы",
            _ => "единиц"
        };
        format!("{} {}", self.0, pronounce)
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
        write!(f,"{}",self.pronounce())
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

#[derive(Debug,
    PartialEq, Eq, PartialOrd, Ord,
    Clone, Copy)]
pub struct Money(pub Decimal);

impl Money {
    pub fn abs(&self) -> Self {
        Money(self.0.abs())
    }

    fn pronounce(&self) -> core::result::Result<String, std::fmt::Error> {
        let integer_amount: u128 = self.0.trunc()
            .try_into()
            .map_err(|_| std::fmt::Error)?;

        let rubles_pronounce: &'static str = match integer_amount % 10 {
            1 if integer_amount % 100 != 11 => "рубль",
            2..=4 if !(integer_amount % 100 >= 12 && integer_amount % 100 <= 14) => "рубля",
            _ => "рублей"
        };

        if self.0.is_integer() {
            Ok(format!("{} {}",integer_amount, rubles_pronounce))
        } else {
            let unwrapped_fraction = self.0.fract().unpack();
            let fraction_amount: u8 = Decimal::from_parts(
                unwrapped_fraction.lo, 0, 0, false, 0)
                .try_into()
                .map_err(|_| std::fmt::Error)?;

            let kopek_pronounce: &'static str = match fraction_amount % 10 {
                1 if fraction_amount % 100 != 11 => "копейка",
                2..=4 if !(fraction_amount % 100 >= 12 && fraction_amount % 100 <= 14) => "копейки",
                _ => "копеек"
            };


            Ok(format!("{} {} {} {}",integer_amount, rubles_pronounce, fraction_amount, kopek_pronounce))
        }
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
        write!(f, "{}", self.pronounce()?)
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

