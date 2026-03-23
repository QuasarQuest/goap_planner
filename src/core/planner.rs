
use crate::models::atomics::{WorldState, GoalState, Action};
use crate::config::planner::PlannerConfig;
use crate::utils::planner::PlanResult;
use crate::error::planner::PlannerError;
use crate::core::a_star;

pub struct Planner {
    // These are stored here so you don't have to pass them every time
    actions: &'static [Action],
    config: PlannerConfig,
}

impl Planner {
    /// Create a new planner with specific capabilities and budget
    pub fn new(actions: &'static [Action], config: PlannerConfig) -> Self {
        Self { actions, config }
    }
    pub fn plan(&self, start: WorldState, goal: GoalState) -> Result<PlanResult, PlannerError> {
        // Delegate to the core engine, passing our stored config/actions
        a_star::plan(
            start,
            goal,
            self.actions,
            self.config
        )
    }
}