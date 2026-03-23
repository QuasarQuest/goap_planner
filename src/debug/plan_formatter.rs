// src/models/plan.rs
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PlannerStats {
    pub nodes_expanded: usize,
    pub nodes_generated: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct PlanResult {
    pub steps: Vec<&'static str>,
    pub total_cost: u64,
    pub stats: PlannerStats,
}

impl PlanResult {
    pub fn print_report(&self) {
        println!("\n");
        println!("╔════════════════════════════════════════╗");
        println!("║            GOAP MISSION PLAN           ║");
        println!("╠════════════════════════════════════════╣");

        for (i, step) in self.steps.iter().enumerate() {
            println!("║ {:02}. {:<34} ║", i + 1, step);
        }

        println!("╠════════════════════════════════════════╣");
        println!("║ Stats:                                 ║");
        println!("║  • Time:      {:<24?} ║", self.stats.duration);
        println!("║  • Expanded:  {:<24} ║", self.stats.nodes_expanded);
        println!("║  • Generated: {:<24} ║", self.stats.nodes_generated);
        println!("║  • Cost:      {:<24} ║", self.total_cost);
        println!("╚════════════════════════════════════════╝\n");
    }
}