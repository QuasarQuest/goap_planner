use crate::models::atomics::{GoalState, WorldState};

#[derive(Clone, Copy, Debug)]
pub struct Heuristic {
    pub state: WorldState,

}
impl Heuristic {
    pub fn heuristic(current: WorldState, goal: GoalState) -> u32 {
        let diff = current.0 ^ goal.0;
        let missing = diff & goal.0;
        missing.count_ones()
    }
}