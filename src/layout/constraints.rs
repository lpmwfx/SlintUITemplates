/// Ratio-space min/max constraints for layout panels.
///
/// All values are in the range [0.0, 1.0] where 1.0 = full container size.
/// The solver enforces that no panel shrinks below `min` or grows above `max`.

/// Minimum ratio — 5% of container.
const RATIO_MIN: f32 = 0.05;
/// Maximum ratio — 95% of container.
const RATIO_MAX: f32 = 0.95;

/// Ratio-space min/max constraint that bounds a panel's resizable range.
#[derive(Debug, Clone, PartialEq)]
/// C on st ra in t struct.
pub struct Constraint {
    /// M in.
    pub min: f32,
    /// M ax.
    pub max: f32,
}

impl Constraint {
    /// Create a constraint with the given min and max ratio bounds.
    pub fn new(min: f32, max: f32) -> Self {
        assert!(min >= 0.0, "min must be >= 0");
        assert!(max <= 1.0, "max must be <= 1");
        assert!(min < max,  "min must be < max");
        Self { min, max }
    }

    /// Clamp a ratio value to the `[min, max]` range.
    pub fn clamp(&self, ratio: f32) -> f32 {
        ratio.clamp(self.min, self.max)
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self { min: RATIO_MIN, max: RATIO_MAX }
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
