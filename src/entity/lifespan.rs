/// The lifespan of an entity.
#[derive(Debug, Clone, Copy)]
pub enum Lifespan {
    /// The Entity ages as the time goes on, and its lifespan decreases generation
    /// after generation.
    Ephemeral(Span),
    /// The Entity is not affected by the passing of time, and its lifespan does
    /// not decrease, but it can still be killed by other entities since its
    /// lifespan is defined and can be altered.
    Immortal,
}

impl Lifespan {
    /// Returns true only if there is lifespan left. It will always return true
    /// if immortal.
    pub fn is_alive(&self) -> bool {
        match self {
            Lifespan::Ephemeral(span) => span.length() > 0,
            Lifespan::Immortal => true,
        }
    }

    /// Shorten the lifespan by a single unit of span, it has no effect if
    /// immortal. Returns the Lifespan left.
    pub fn shorten(&mut self) -> &Self {
        self.shorten_by(Span::with_length(1))
    }

    /// Lengthen the lifespan by a single unit of span, it has no effect if
    /// immortal. Returns the Lifespan left.
    pub fn lengthen(&mut self) -> &Self {
        self.lengthen_by(Span::with_length(1))
    }

    /// Shorten the lifespan by the given amount of span, it has no effect if
    /// immortal. Returns the Lifespan left.
    pub fn shorten_by(&mut self, amount: Span) -> &Self {
        if let Lifespan::Ephemeral(span) = self {
            span.shorten_by(amount.into());
        }
        self
    }

    /// Lengthen the lifespan by the given amount of span, it has no effect if
    /// immortal. Returns the Lifespan left.
    pub fn lengthen_by(&mut self, amount: Span) -> &Self {
        if let Lifespan::Ephemeral(span) = self {
            span.lengthen_by(amount.into());
        }
        self
    }

    /// Replaces the lifespan with a new empty one, by effectively representing
    /// the death of the entity. This method will have an effect also on an
    /// immortal lifespan.
    pub fn clear(&mut self) {
        *self = Lifespan::Ephemeral(Span::empty())
    }
}

/// The window of time span as seen by an entity, represented as discrete number
/// of steps left before the entity dies.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    length: u64,
}

impl From<u64> for Span {
    fn from(length: u64) -> Self {
        Self { length }
    }
}

impl From<Span> for u64 {
    fn from(span: Span) -> Self {
        span.length
    }
}

impl Span {
    /// Constructs a new Span of the given length.
    pub fn with_length(length: u64) -> Self {
        Self { length }
    }

    /// Constructs an empty lifespan.
    pub fn empty() -> Self {
        Self { length: 0 }
    }

    /// Gets the length of this Span.
    pub fn length(self) -> u64 {
        self.length
    }

    /// Shorten the span by the given length of span.
    pub fn shorten_by(&mut self, length: u64) {
        self.length = self.length.saturating_sub(length);
    }

    /// Lengthen the span by the given length of span.
    pub fn lengthen_by(&mut self, length: u64) {
        self.length = self.length.saturating_add(length);
    }

    /// Shorten the span by a single unit of span.
    pub fn shorten(&mut self) {
        self.shorten_by(1);
    }

    /// Lengthen the span by a single unit of span.
    pub fn lengthen(&mut self) {
        self.lengthen_by(1);
    }

    /// Reset the span to 0.
    pub fn clear(&mut self) {
        self.length = 0;
    }
}
