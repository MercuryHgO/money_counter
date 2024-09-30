use core::panic;
use std::ops::IndexMut;

pub struct Triplet {
    number: [u8;3],
    ///Maximum supported size: 12 (0-12)
    pos: usize,
    feminie: bool
}

impl Triplet {
    pub fn new(number: [u8;3], pos: usize) -> Triplet {
        Triplet { number, pos, feminie: false}
    }

    pub fn into_feminie(&self) -> Self {
        Triplet { feminie: true, ..*self  }
    }

    fn pronounce_hundreeds(&self) -> &'static str {
        match self.number[0] {
            b'1' => "сто",
            b'2' => "двести",
            b'3' => "триста",
            b'4' => "четыреста",
            b'5' => "пятьсот",
            b'6' => "шестьсот",
            b'7' => "семьсот",
            b'8' => "восемьсот",
            b'9' => "девятьсот",
            _ => ""
        }
    }

    /// Returns pronounced Triplet and Bool variable, that tells,
    /// whether the unit has been processed
    fn pronounce_tens(&self) -> (&'static str, bool) {
        match self.number[1] {
            b'1' => match self.number[2] {
                b'1' => ("одиннадцать",true),
                b'2' => ("двенадцать",true),
                b'3' => ("тринадцать",true),
                b'4' => ("четырнадцать",true),
                b'5' => ("пятнадцать",true),
                b'6' => ("шестнадцать",true),
                b'7' => ("семнадцать",true),
                b'8' => ("восемнадцать",true),
                b'9' => ("девятнадцать",true),
                b'0' => ("десять",false),
                _ => ("",false)
            },
            b'2' => ("двадцать",false),
            b'3' => ("тридцать",false),
            b'4' => ("сорок",false),
            b'5' => ("пятьдесят",false),
            b'6' => ("шестдесят",false),
            b'7' => ("семьдесят",false),
            b'8' => ("восемьдесят",false),
            b'9' => ("девяноста",false),
            _ => ("",false)
        }
    }

    /// feminitive - specifies feminitive for 1 and 2: 
    /// "один/одна", "два/две"
    fn pronounce_units(&self) -> &'static str {
        match self.number[2] {
            b'1' => if self.feminie { "одна" } else { "один" },
            b'2' => if self.feminie { "две" } else { "два" },
            b'3' => "три",
            b'4' => "четыре",
            b'5' => "пять",
            b'6' => "шесть",
            b'7' => "семь",
            b'8' => "восемь",
            b'9' => "девять",
            _ => ""
        }
    }

    fn number_pronounce(&self) -> String {
        let mut pronounced_number = String::new();

        let hundreeds = self.pronounce_hundreeds();
        pronounced_number.push_str(&hundreeds);
        if !hundreeds.is_empty() { pronounced_number.push(' ') }

        let (tens, units_processed) = self.pronounce_tens();
        pronounced_number.push_str(tens);
        if !tens.is_empty() { pronounced_number.push(' ') }
        
        if !units_processed { 
            if self.pos == 1 {
                pronounced_number.push_str(self.into_feminie().pronounce_units())
            } else {
                pronounced_number.push_str(self.pronounce_units())
            }
        };

        pronounced_number.trim().to_string()
    }

    /// Generates triplets pronounce, based on passed pronounces
    pub fn triplet_pronounce<'a>(&self,pronounces: [&'a str;3]) -> &'a str {
        if self.number[1] == b'1' {
            return pronounces[2];
        }

        match self.number[2] {
            b'1' => pronounces[0],
            b'2'..b'5' => pronounces[1],
            _ => pronounces[2]
        }

    }

    pub fn pronounce(&self) -> String {
        let mut pronounced_triplet = String::new();

        let number_pronounce = &self.number_pronounce();
        pronounced_triplet.push_str(number_pronounce);

        if !number_pronounce.is_empty() {
            pronounced_triplet.push(' ');

            match self.pos {
                0 => (),
                1 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["тысяча","тысячи","тысяч"]
                        )
                    )
                },
                2 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["миллион","миллиона","миллионов"]
                        )
                    )
                },
                3 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["миллиард","миллиарда","миллиардов"]
                        )
                    )
                },
                4 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["триллион","триллиона","триллионов"]
                        )
                    )
                },
                5 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["квадрилион","квадриллиона","квадриллионов"]
                        )
                    )
                },
                6 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["квинтиллион","квинтиллиона","квинтиллионов"]
                        )
                    )
                },
                7 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["секситиллион","секстиллиона","секстиллионов"]
                        )
                    )
                },
                8 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["септиллион","септиллиона","септиллионов"]
                        )
                    )
                },
                9 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["октиллион","октиллиона","октиллионов"]
                        )
                    )
                },
                10 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["нониллион","нониллиона","нониллионов"]
                        )
                    )
                },
                11 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["дециллион","дециллиона","дециллионов"]
                        )
                    )
                },
                12 => {
                    pronounced_triplet.push_str(
                        self.triplet_pronounce(
                            ["ундециллион","ундециллиона","ундециллионов"]
                        )
                    )
                },
                _ => panic!("Triplet size overflow")
            };
        }

        pronounced_triplet
    }
}


pub trait NumberPronouce {
    fn into_triplets(& self) -> Vec<Triplet>;
    fn pronounce(&self) -> String {
        let mut result = String::new();

        self
            .into_triplets()
            .into_iter()
            .rev()
            .for_each( |triplet| {
                result.push_str(&triplet.pronounce());
                result.push(' ');
            });

        result
    }
}
