enum Flag {
    AsDegrees,
    AsRadians,
    Hex,
    Dec,
    Bin,
    Oct
}

trait FlagFunc {
    fn degrees(self, flag : Flag) -> f64;
    fn convert(&self, flag : Flag) -> String;
}

impl FlagFunc for f64 {
    fn degrees(&self, flag : Flag) -> String {
        match flag {
            Flag::AsDegrees => self.to_degrees(),
            _ => self
        }
    }
    fn convert(&self, flag : Flag) -> String {
        match flag {
            Flag::Hex => format!("{:x}", self),
            Flag::Bin => format!("{:b}", self),
            Flag::Oct => format!("{:o}", self)
        }
    }
}