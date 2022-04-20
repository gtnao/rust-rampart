use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Interval<T: Ord> {
    pub lesser: T,
    pub greater: T,
}

impl<T: Ord> Interval<T> {
    pub fn new(lesser: T, greater: T) -> Self {
        Self { lesser, greater }
    }
    pub fn relate(&self, y: &Self) -> Relation {
        let lxly = self.lesser.cmp(&y.lesser);
        let lxgy = self.lesser.cmp(&y.greater);
        let gxly = self.greater.cmp(&y.lesser);
        let gxgy = self.greater.cmp(&y.greater);
        match (lxly, lxgy, gxly, gxgy) {
            (Ordering::Equal, _, _, Ordering::Equal) => Relation::Equal,
            (_, _, Ordering::Less, _) => Relation::Before,
            (Ordering::Less, _, Ordering::Equal, Ordering::Less) => Relation::Meets,
            (_, _, Ordering::Equal, _) => Relation::Overlaps,
            (Ordering::Greater, Ordering::Equal, _, Ordering::Greater) => Relation::MetBy,
            (_, Ordering::Equal, _, _) => Relation::OverlappedBy,
            (_, Ordering::Greater, _, _) => Relation::After,
            (Ordering::Less, _, _, Ordering::Less) => Relation::Overlaps,
            (Ordering::Less, _, _, Ordering::Equal) => Relation::FinishedBy,
            (Ordering::Less, _, _, Ordering::Greater) => Relation::Contains,
            (Ordering::Equal, _, _, Ordering::Less) => Relation::Starts,
            (Ordering::Equal, _, _, Ordering::Greater) => Relation::StartedBy,
            (Ordering::Greater, _, _, Ordering::Less) => Relation::During,
            (Ordering::Greater, _, _, Ordering::Equal) => Relation::Finishes,
            (Ordering::Greater, _, _, Ordering::Greater) => Relation::OverlappedBy,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lesser.cmp(&self.greater) == Ordering::Equal
    }
    pub fn is_non_empty(&self) -> bool {
        !self.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relation {
    Before,
    Meets,
    Overlaps,
    FinishedBy,
    Contains,
    Starts,
    Equal,
    StartedBy,
    During,
    Finishes,
    OverlappedBy,
    MetBy,
    After,
}

impl Relation {
    pub fn invert(&self) -> Self {
        match self {
            Self::After => Self::Before,
            Self::Before => Self::After,
            Self::Contains => Self::During,
            Self::During => Self::Contains,
            Self::Equal => Self::Equal,
            Self::FinishedBy => Self::Finishes,
            Self::Finishes => Self::FinishedBy,
            Self::Meets => Self::MetBy,
            Self::MetBy => Self::Meets,
            Self::OverlappedBy => Self::Overlaps,
            Self::Overlaps => Self::OverlappedBy,
            Self::StartedBy => Self::Starts,
            Self::Starts => Self::StartedBy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Interval, Relation};

    #[test]
    fn test_interval_new() {
        let interval = Interval::new(1, 2);
        assert_eq!(interval.lesser, 1);
        assert_eq!(interval.greater, 2);
    }

    #[test]
    fn test_relate() {
        let y = Interval::new(4, 8);
        assert_eq!(Relation::Before, Interval::new(1, 3).relate(&y));
        assert_eq!(Relation::Meets, Interval::new(2, 4).relate(&y));
        assert_eq!(Relation::Overlaps, Interval::new(3, 5).relate(&y));
        assert_eq!(Relation::FinishedBy, Interval::new(3, 8).relate(&y));
        assert_eq!(Relation::Contains, Interval::new(3, 9).relate(&y));
        assert_eq!(Relation::Starts, Interval::new(4, 6).relate(&y));
        assert_eq!(Relation::Equal, Interval::new(4, 8).relate(&y));
        assert_eq!(Relation::StartedBy, Interval::new(4, 9).relate(&y));
        assert_eq!(Relation::During, Interval::new(5, 7).relate(&y));
        assert_eq!(Relation::Finishes, Interval::new(6, 8).relate(&y));
        assert_eq!(Relation::OverlappedBy, Interval::new(7, 9).relate(&y));
        assert_eq!(Relation::MetBy, Interval::new(8, 10).relate(&y));
        assert_eq!(Relation::After, Interval::new(9, 11).relate(&y));
    }

    #[test]
    fn test_is_empty() {
        assert_eq!(Interval::new(1, 1).is_empty(), true);
        assert_eq!(Interval::new(1, 2).is_empty(), false);
    }

    #[test]
    fn test_is_non_empty() {
        assert_eq!(Interval::new(1, 1).is_non_empty(), false);
        assert_eq!(Interval::new(1, 2).is_non_empty(), true);
    }

    #[test]
    fn test_invert() {
        assert_eq!(Relation::Before, Relation::After.invert());
        assert_eq!(Relation::After, Relation::Before.invert());
        assert_eq!(Relation::During, Relation::Contains.invert());
        assert_eq!(Relation::Contains, Relation::During.invert());
        assert_eq!(Relation::Equal, Relation::Equal.invert());
        assert_eq!(Relation::Finishes, Relation::FinishedBy.invert());
        assert_eq!(Relation::FinishedBy, Relation::Finishes.invert());
        assert_eq!(Relation::MetBy, Relation::Meets.invert());
        assert_eq!(Relation::Meets, Relation::MetBy.invert());
        assert_eq!(Relation::Overlaps, Relation::OverlappedBy.invert());
        assert_eq!(Relation::OverlappedBy, Relation::Overlaps.invert());
        assert_eq!(Relation::Starts, Relation::StartedBy.invert());
        assert_eq!(Relation::StartedBy, Relation::Starts.invert());
    }
}
