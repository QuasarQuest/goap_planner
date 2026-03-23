#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WorldState(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GoalState(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub name: &'static str,

    /// PRECONDITION: Which bits do we care about before running this?
    pub pre_mask: u64,

    /// PRECONDITION: What must those bits equal?
    pub pre_value: u64,

    /// EFFECT: Which bits does this action change?
    pub effect_mask: u64,

    /// EFFECT: What do we change those bits to?
    pub effect_value: u64,

    /// The "Weight" of this action for A* pathfinding (e.g., fuel cost, time).
    pub cost: u32,
}

impl Action {
    /// Checks if this action can be executed in the current state.
    /// This compiles down to 2-3 CPU cycles.
    #[inline(always)]
    pub fn is_valid(&self, state: &WorldState) -> bool {
        (state.0 & self.pre_mask) == self.pre_value
    }

    /// Returns the new state after applying this action.
    #[inline(always)]
    pub fn apply(&self, state: &WorldState) -> WorldState {
        // 1. Clear the bits we are about to change (state & ~mask)
        // 2. Set the new values (| value)
        WorldState((state.0 & !self.effect_mask) | self.effect_value)
    }
}