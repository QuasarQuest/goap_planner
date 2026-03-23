use std::cmp::Ordering;
use crate::models::atomics::WorldState;

// A Node tracks "How we got here"
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub state: WorldState,
    pub cost_g: u32,       // Cost from start to here
    pub cost_h: u32,       // Heuristic (estimated cost to goal)
    pub action_index: usize, // Which action created this state?
    pub parent_index: usize, // Index of the previous node in our 'arena' vector
}

impl Node {
    pub fn total_cost(&self) -> u32 {
        self.cost_g + self.cost_h
    }
}

// BOILERPLATE: Rust needs to know how to sort Nodes for the Priority Queue.
// We want the LOWEST 'f' score to be popped first.
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison because BinaryHeap is a Max-Heap by default
        other.total_cost().cmp(&self.total_cost())
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}