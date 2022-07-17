use std::ops::{AddAssign, SubAssign};

use rand::{thread_rng, Rng};
use regex::Regex;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DiceValue {
    pub dice: u32,
    pub constant: i32,
}
impl DiceValue {
    pub fn roll(&self) -> u32 {
        let mut rng = thread_rng();
        (self.constant
            + vec![6; self.dice as usize]
                .into_iter()
                .map(|dice_size| rng.gen_range(1..=dice_size))
                .sum::<i32>()) as u32
    }
    pub fn drained_to_match(&self, target: u32) -> Option<DiceValue> {
        let mut rng = thread_rng();
        let mut target = target as i32 - self.constant;
        for dice_used in 0..=self.dice {
            if target <= 0 {
                return Some(DiceValue {
                    dice: self.dice - dice_used,
                    constant: self.constant,
                });
            }
            target -= rng.gen_range(1..=6);
        }
        None
    }
    pub fn theoretical_limit(&self) -> u32 {
        (6 * self.dice as i32 + self.constant) as u32
    }
    pub fn as_string(&self) -> String {
        format!("{}d6+{}", self.dice, self.constant)
    }
}
impl From<&str> for DiceValue {
    fn from(source: &str) -> Self {
        let re = Regex::new(r"(?:(\d+)d6)?([+-]?\d+)?").unwrap();

        if let Some(captures) = re.captures(source) {
            Self {
                dice: if let Some(dice_capture) = captures.get(1) {
                    dice_capture.as_str().parse().unwrap()
                } else {
                    0
                },
                constant: if let Some(const_capture) = captures.get(2) {
                    const_capture.as_str().parse().unwrap()
                } else {
                    0
                },
            }
        } else {
            Self {
                dice: 0,
                constant: 0,
            }
        }
    }
}
impl AddAssign for DiceValue {
    fn add_assign(&mut self, rhs: Self) {
        self.dice += rhs.dice;
        self.constant += rhs.constant;
    }
}
impl SubAssign for DiceValue {
    fn sub_assign(&mut self, rhs: Self) {
        self.dice -= rhs.dice;
        self.constant -= rhs.constant;
    }
}

#[cfg(test)]
mod test {
    use super::DiceValue;

    // It was faster to write these than try to figure out how to do regex

    #[test]
    fn full_parse() {
        let parsed: DiceValue = "2d6+4".into();
        let expected = DiceValue {
            dice: 2,
            constant: 4,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn dice_parse() {
        let parsed: DiceValue = "4d6".into();
        let expected = DiceValue {
            dice: 4,
            constant: 0,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn const_parse() {
        let parsed: DiceValue = "5".into();
        let expected = DiceValue {
            dice: 0,
            constant: 5,
        };
        assert!(parsed == expected);
    }
    #[test]
    fn empty_parse() {
        let parsed: DiceValue = "".into();
        let expected = DiceValue::default();
        assert!(parsed == expected);
    }

    #[test]
    fn full_parse_double_digits() {
        let parsed: DiceValue = "20d6+40".into();
        let expected = DiceValue {
            dice: 20,
            constant: 40,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn dice_parse_double_digits() {
        let parsed: DiceValue = "40d6".into();
        let expected = DiceValue {
            dice: 40,
            constant: 0,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn const_parse_double_digits() {
        let parsed: DiceValue = "50".into();
        let expected = DiceValue {
            dice: 0,
            constant: 50,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn full_parse_negative() {
        let parsed: DiceValue = "20d6-40".into();
        let expected = DiceValue {
            dice: 20,
            constant: -40,
        };
        assert!(parsed == expected);
    }

    #[test]
    fn const_parse_negative() {
        let parsed: DiceValue = "-50".into();
        let expected = DiceValue {
            dice: 0,
            constant: -50,
        };
        assert!(parsed == expected);
    }
}
