pub mod f64_flag_magic {
    pub enum Flag {
        AsDegrees,
        AsRadians,
        Hex,
        Dec,
        Bin,
        Oct,
    }

    pub trait FlagFunc {
        fn degrees(&self, flag: Flag) -> f64;
        fn convert(self, flag: Flag) -> String;
    }

    impl FlagFunc for f64 {
        fn degrees(&self, flag: Flag) -> f64 {
            match flag {
                Flag::AsDegrees => self.to_degrees(),
                _ => self.clone(),
            }
        }
        fn convert(self, flag: Flag) -> String {
            match flag {
                Flag::Hex => format!("0x{:x}", self as i64),
                Flag::Bin => format!("0b{:b}", self as i64),
                Flag::Oct => format!("0o{:o}", self as i64),
                _ => self.to_string(),
            }
        }
    }
}
