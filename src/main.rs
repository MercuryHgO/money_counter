use std::env::args;

use rust_decimal::Decimal;

#[derive(Debug)]
struct Unit(u128);

impl Unit {
    fn pronounce(&self) -> String {
        let pronounce: &'static str = match self.0 % 10 {
            1 if self.0 % 100 != 11 => "единицу",
            2..=4 if !(self.0 % 100 >= 12 && self.0 % 100 <= 14) => "единицы",
            _ => "единиц"
        };
        format!("{} {}", self.0, pronounce)
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

#[derive(Debug)]
struct Money(Decimal);

impl Money {
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

impl TryFrom<String> for Money {
    type Error = rust_decimal::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let number = Decimal::from_str_exact(&value)?;
        Ok(Money(number))
    }
}

impl TryFrom<Decimal> for Money {
    type Error = Error;

    fn try_from(value: Decimal) -> Result<Self,Self::Error> {
        let unpacked = value.unpack();

        if unpacked.negative == true { return Err(Error::NegativeValue); };
        if unpacked.scale > 2 { return Err(Error::KopekDigitsTooBig); };
        
        Ok(Money(value))
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pronounce()?)
    }
}

#[derive(Debug)]
enum Error {
    NegativeValue,
    BudgetNotPresented,
    KopekDigitsTooBig,
    RustDecimalError(rust_decimal::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NegativeValue => write!(f,"Число не может быть отрицательным"),
            Error::BudgetNotPresented => write!(f,"Бюджет не предоставлен"),
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

fn main() -> anyhow::Result<()> {
    let budget: Money = match args().nth(1) {
        Some(val) => {
            let dec = Decimal::from_str_exact(&val)?;
            dec.try_into()?
        },
        None => {
            Err(Error::BudgetNotPresented)?
        },
    };

    const PRICE: Money = Money(Decimal::from_parts(8999,0 ,0 ,false , 2));

    let count = budget.0 / PRICE.0;
    let units = Unit(count.trunc().try_into()?);

    let summ: Money = Money(
        PRICE.0 * Decimal::from(&units)
    );

    let leftover = Money(
        budget.0 - summ.0
    );

    println!("Размер бюджета: {}",budget);
    println!("Цена за единицу товара: {}",PRICE);


    print!("За {} можно приобрести {} товара ",budget,units);
    println!("на сумму {}, имея в остатке {}",summ,leftover);


    Ok(())
    
}
