pub mod goap_mission_small;
pub mod goap_mission_medium;
pub mod goap_mission_large;

use crate::models::atomics::{Action, WorldState, GoalState};

pub struct Mission {
    pub name:    &'static str,
    pub start:   WorldState,
    pub goal:    GoalState,
    pub actions: &'static [Action],
}

pub static ALL_MISSIONS: &[Mission] = &[
    Mission {
        name:    "ELS Small",
        start:   goap_mission_small::INITIAL_STATE,
        goal:    goap_mission_small::GOAL_STATE,
        actions: &goap_mission_small::ACTIONS,
    },
    Mission {
        name:    "ELS Medium",
        start:   goap_mission_medium::INITIAL_STATE,
        goal:    goap_mission_medium::GOAL_STATE,
        actions: &goap_mission_medium::ACTIONS,
    },
    Mission {
        name:    "ELS Large",
        start:   goap_mission_large::INITIAL_STATE,
        goal:    goap_mission_large::GOAL_STATE,
        actions: &goap_mission_large::ACTIONS,
    },
];