use iced::wgpu::wgt::error::ErrorType;

struct Percentage(u64);

pub enum PercentageError {
    OutOfRange { min: u64, max: u64 },
}

impl Percentage {
    const VALUE_SCALE: u64 = 1000; //three decimal places
    const MIN: u64 = 0;
    const MAX: u64 = 10_000;

    fn try_new(percent: u64) -> Result<Self, PercentageError> {
        if (Self::MIN..=Self::MAX).contains(&percent) {
            Ok(Self(percent))
        } else {
            Err(PercentageError::OutOfRange { min: Self::MIN, max: Self::MAX,})
        }
    }

    fn apply_to(self, base: u64) -> u64 {
        base * self.0 /Self::VALUE_SCALE
    }
}

pub struct Fence {
    reputation: u8,
    avg_markup: u64,
    lowest_markup: u64,
    highest_markup: u64,
}

impl Default for Fence {
    fn default() -> Self {
        Self { reputation: 0, avg_markup: 110, lowest_markup: 108, highest_markup: 120 }
    }
}

impl Fence {
    pub fn new(reputation: u8, avg_markup: u64, lowest_markup: u64, highest_markup: u64) -> Self {
        Self {
            reputation,
            avg_markup,
            lowest_markup,
            highest_markup,
        }
    }

    pub fn avg_markup_price(&self, base_price: u64) -> Result<u64, PercentageError>{
        let percentage = Percentage::try_new(self.avg_markup)?; 
        Ok(percentage.apply_to(base_price))
    }
    
    pub fn lowest_markup_price(&self, base_price: u64) -> Result<u64, PercentageError> {
        let percentage = Percentage::try_new(self.lowest_markup)?;
        Ok(percentage.apply_to(base_price))
    }
    pub fn highest_markup_price(&self, base_price: u64) -> Result<u64, PercentageError> {
        let percentage = Percentage::try_new(self.highest_markup)?;
        Ok(percentage.apply_to(base_price))
    }
}
