use std::fmt;

#[derive(Debug)]
pub struct Percentage(u64);

#[derive(Debug, PartialEq)]
pub enum PercentageError {
    OutOfRange { min: u64, max: u64 },
}

impl Percentage {
    const VALUE_SCALE: u64 = 1000; //Allows for percentages to 1 decimal point i.e 12.5%
    const MIN: u64 = 0;
    const MAX: u64 = 10_000;

    fn try_new(percent: u64) -> Result<Self, PercentageError> {
        if (Self::MIN..=Self::MAX).contains(&percent) {
            Ok(Self(percent))
        } else {
            Err(PercentageError::OutOfRange {
                min: Self::MIN,
                max: Self::MAX,
            })
        }
    }

    fn apply_to(&self, base: u64) -> u64 {
        base * self.0 / Self::VALUE_SCALE
    }

    pub fn markup_scaled(&self) -> u64 {
        self.0 - Self::VALUE_SCALE
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scaled = self.markup_scaled();
        let whole = scaled / 10;
        let fraction = scaled % 10;

        if fraction == 0 {
            write!(f, "{whole}% markup")
        } else {
            write!(f, "{whole}.{fraction}% markup")
        }
    }
}

pub struct Fence {
    reputation: u8,
    pub avg_markup: Percentage,
    pub lowest_markup: Percentage,
    pub highest_markup: Percentage,
}

impl Default for Fence {
    fn default() -> Self {
        //Markup is 4 digits to allow for 1 decimal point. 1100 = 110%, 1205 = 120.5%
        Self {
            reputation: 0,
            avg_markup: Percentage(1100),
            lowest_markup: Percentage(1080),
            highest_markup: Percentage(1205),
        }
    }
}

impl Fence {
    pub fn new(
        reputation: u8,
        avg_markup: Percentage,
        lowest_markup: Percentage,
        highest_markup: Percentage,
    ) -> Self {
        Self {
            reputation,
            avg_markup,
            lowest_markup,
            highest_markup,
        }
    }

    pub fn avg_markup_price(&self, base_price: u64) -> u64 {
        self.avg_markup.apply_to(base_price)
    }

    pub fn lowest_markup_price(&self, base_price: u64) -> u64 {
        self.lowest_markup.apply_to(base_price)
    }
    pub fn highest_markup_price(&self, base_price: u64) -> u64 {
        self.highest_markup.apply_to(base_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_range_percentage_fails() {
        let err = Percentage::try_new(100000000).unwrap_err();
        assert_eq!(
            err,
            PercentageError::OutOfRange {
                min: Percentage::MIN,
                max: Percentage::MAX
            }
        );
    }

    #[test]
    fn default_fence_avg_markup_succeeds() {
        let fence = Fence::default();
        let base_price = 100;
        let result = fence.avg_markup_price(base_price);
        assert_eq!(110, result);
    }

    #[test]
    fn default_fence_lowest_markup_succeeds() {
        let fence = Fence::default();
        let base_price = 100;
        let result = fence.lowest_markup_price(base_price);
        assert_eq!(108, result);
    }

    #[test]
    fn default_fence_highest_markup_succeeds() {
        let fence = Fence::default();
        let base_price = 1000;
        let result = fence.highest_markup_price(base_price);
        assert_eq!(1205, result);
    }

    #[test]
    fn integar_trunc_works_as_intended() {
        let fence = Fence::default();
        let base_price = 100;
        let result = fence.highest_markup_price(base_price);
        assert_eq!(120, result);
    }
}
