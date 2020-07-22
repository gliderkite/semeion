/// The lifespan of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    /// Constructs an Ephemeral Lifespan with the given span value.
    pub fn with_span(span: impl Into<Span>) -> Self {
        Self::Ephemeral(span.into())
    }

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
    pub fn shorten_by(&mut self, amount: impl Into<Span>) -> &Self {
        let amount = amount.into();
        if let Lifespan::Ephemeral(span) = self {
            span.shorten_by(amount.into());
        }
        self
    }

    /// Lengthen the lifespan by the given amount of span, it has no effect if
    /// immortal. Returns the Lifespan left.
    pub fn lengthen_by(&mut self, amount: impl Into<Span>) -> &Self {
        let amount = amount.into();
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

    /// Gets the Span of the Lifespan if self is Ephemeral, otherwise returns
    /// None.
    pub fn span(self) -> Option<Span> {
        if let Lifespan::Ephemeral(span) = self {
            Some(span)
        } else {
            None
        }
    }

    /// Gets the length of the Lifespan if self is Ephemeral, otherwise returns
    /// None.
    pub fn length(self) -> Option<u64> {
        self.span().map(|span| span.length())
    }
}

/// The window of time span as seen by an entity, represented as discrete number
/// of steps left before the entity dies.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
