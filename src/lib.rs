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

#[derive(Clone, Debug)]
pub enum Relation {
    Unknown,
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
            _ => Self::Unknown,
        }
    }
}
