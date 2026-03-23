use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct PlannerConfig {
    /// Max nodes to expand before aborting (Hard Memory Limit)
    pub max_nodes: usize,
    /// Max time allowed before aborting (Hard Real-Time Limit)
    pub max_time: Duration,
    /// Weight for the heuristic (1.0 = Accurate, 1.5 = Faster but sub-optimal)
    pub heuristic_weight: f32,

    pub heuristic_type: HeuristicType,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_time: Duration::from_millis(5000),
            heuristic_weight: 1.5,
            heuristic_type: HeuristicType::Hamming
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeuristicType {
    /// Simply counts missing bits (Fastest).
    Hamming,
    /// Counts missing bits but multiplies them by a weight (Good for speed).
    Weighted(u64),
    /// A custom mode that prioritizes safety over speed.
    SafetyFirst,
}