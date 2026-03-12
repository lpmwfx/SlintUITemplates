/// Ratio-space min/max constraints for layout panels.
///
/// All values are in the range [0.0, 1.0] where 1.0 = full container size.
/// The solver enforces that no panel shrinks below `min` or grows above `max`.

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub min: f32,   // minimum ratio (default: 0.05 = 5% of container)
    pub max: f32,   // maximum ratio (default: 0.95 = 95% of container)
}

impl Constraint {
    pub fn new(min: f32, max: f32) -> Self {
        assert!(min >= 0.0, "min must be >= 0");
        assert!(max <= 1.0, "max must be <= 1");
        assert!(min < max,  "min must be < max");
        Self { min, max }
    }

    pub fn clamp(&self, ratio: f32) -> f32 {
        ratio.clamp(self.min, self.max)
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self { min: 0.05, max: 0.95 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_respects_bounds() {
        let c = Constraint::new(0.1, 0.8);
        assert_eq!(c.clamp(0.0), 0.1);
        assert_eq!(c.clamp(0.5), 0.5);
        assert_eq!(c.clamp(1.0), 0.8);
    }
}
