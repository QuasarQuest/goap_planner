// src/core/a_star.rs

use rustc_hash::FxHashSet;
use std::time::Instant;

use crate::models::atomics::{WorldState, GoalState, Action};
use crate::config::planner::PlannerConfig;
use crate::utils::planner::{PlanResult, PlannerStats};
use crate::error::planner::PlannerError;
use crate::core::node::Node;
use crate::core::heuristic::Heuristic;

// ── Bucket priority queue ────────────────────────────────────────────────────
// O(1) push, amortised O(1) pop.  Works because f-scores are small integers.

struct BucketQueue {
    buckets: Vec<Vec<usize>>,
    min_idx: usize,
}

impl BucketQueue {
    fn with_capacity(num_buckets: usize) -> Self {
        Self {
            buckets: vec![Vec::new(); num_buckets],
            min_idx: usize::MAX,
        }
    }

    #[inline(always)]
    fn push(&mut self, priority: u32, value: usize) {
        let idx = priority as usize;
        if idx >= self.buckets.len() {
            self.buckets.resize_with(idx + 1, Vec::new);
        }
        self.buckets[idx].push(value);
        if idx < self.min_idx {
            self.min_idx = idx;
        }
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<usize> {
        let len = self.buckets.len();
        while self.min_idx < len {
            if let Some(val) = self.buckets[self.min_idx].pop() {
                return Some(val);
            }
            self.min_idx += 1;
        }
        None
    }
}

// ── Fixed-point heuristic weight ─────────────────────────────────────────────
// Convert f32 weight to numerator once, avoid f32 cast per node.
// Precision: 1/64 ≈ 0.016 granularity — plenty for GOAP weights.

const WEIGHT_DENOM: u32 = 64;

#[inline(always)]
fn weight_to_fixed(w: f32) -> u32 {
    (w * WEIGHT_DENOM as f32) as u32   // 1.5 → 96
}

#[inline(always)]
fn apply_weight(raw_h: u32, weight_num: u32) -> u32 {
    (raw_h * weight_num) / WEIGHT_DENOM
}

// ── Core algorithm ───────────────────────────────────────────────────────────

pub fn plan(
    start: WorldState,
    goal: GoalState,
    actions: &[Action],
    config: PlannerConfig
) -> Result<PlanResult, PlannerError> {

    if actions.is_empty() {
        return Err(PlannerError::EmptyActionList);
    }

    let start_time = Instant::now();
    let weight_num = weight_to_fixed(config.heuristic_weight);

    // 256 buckets covers f-scores 0..255; grows automatically if exceeded
    let mut open_set = BucketQueue::with_capacity(256);
    let mut arena: Vec<Node> = Vec::with_capacity(config.max_nodes);
    let mut closed_set: FxHashSet<u64> = FxHashSet::with_capacity_and_hasher(
        config.max_nodes,
        Default::default(),
    );

    // Seed
    let start_h = Heuristic::heuristic(start, goal);
    let start_wh = apply_weight(start_h, weight_num);

    arena.push(Node {
        state: start,
        cost_g: 0,
        cost_h: start_wh,
        action_index: 0,
        parent_index: 0,
    });

    open_set.push(start_wh, 0);
    closed_set.insert(start.0);

    let mut nodes_expanded: usize = 0;
    let mut nodes_generated: usize = 1;

    // ── Search loop ──────────────────────────────────────────────────────

    while let Some(current_idx) = open_set.pop() {

        // Budget: nodes
        if nodes_expanded >= config.max_nodes {
            return Err(PlannerError::NodeLimitExceeded);
        }

        // Budget: time — syscall every 64 expansions instead of 16
        if nodes_expanded & 63 == 0 {
            if start_time.elapsed() > config.max_time {
                return Err(PlannerError::TimeLimitExceeded);
            }
        }

        nodes_expanded += 1;

        let current_state = arena[current_idx].state;
        let current_g = arena[current_idx].cost_g;

        // Goal check
        if (current_state.0 & goal.0) == goal.0 {
            let steps = reconstruct_path(current_idx, &arena, actions);
            return Ok(PlanResult {
                steps,
                total_cost: current_g as f32,
                stats: PlannerStats {
                    nodes_expanded,
                    nodes_generated,
                    duration: start_time.elapsed(),
                }
            });
        }

        // Expand neighbours
        for (i, action) in actions.iter().enumerate() {

            if !action.is_valid(&current_state) {
                continue;
            }

            let next_state = action.apply(&current_state);

            // Single hash lookup: insert() returns false if already present
            if !closed_set.insert(next_state.0) {
                continue;
            }

            let next_g = current_g + action.cost;
            let raw_h = Heuristic::heuristic(next_state, goal);
            let weighted_h = apply_weight(raw_h, weight_num);
            let f_score = next_g + weighted_h;

            if arena.len() >= config.max_nodes {
                return Err(PlannerError::NodeLimitExceeded);
            }

            let next_idx = arena.len();
            arena.push(Node {
                state: next_state,
                cost_g: next_g,
                cost_h: weighted_h,
                action_index: i,
                parent_index: current_idx,
            });
            nodes_generated += 1;

            open_set.push(f_score, next_idx);
        }
    }

    Err(PlannerError::NoPathFound)
}

// ── Path reconstruction ──────────────────────────────────────────────────────

fn reconstruct_path(
    mut current_idx: usize,
    arena: &[Node],
    actions: &[Action]
) -> Vec<&'static str> {
    let mut path = Vec::with_capacity(32);
    while current_idx != 0 {
        let node = &arena[current_idx];
        path.push(actions[node.action_index].name);
        current_idx = node.parent_index;
    }
    path.reverse();
    path
}