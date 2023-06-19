#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Madhab {
    /// Jomhor (Shafii, Maliki & Hambali)
    Shafi = 1,
    /// Hanafi
    Hanafi = 2,
}

impl Madhab {
    pub const fn shadow(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shafi_shadow() {
        let shafi = Madhab::Shafi;

        assert_eq!(shafi.shadow(), 1);
    }

    #[test]
    fn hanafi_shadow() {
        let hanafi = Madhab::Hanafi;

        assert_eq!(hanafi.shadow(), 2);
    }
}
