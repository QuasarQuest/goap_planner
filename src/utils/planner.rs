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
    pub total_cost: f32,
    pub stats: PlannerStats,
}

impl PlanResult {
    pub fn print_report(&self) {
        println!("\n");
        println!("╔════════════════════════════════════════╗");
        println!("║            GOAP MISSION PLAN           ║");
        println!("╠════════════════════════════════════════╣");

        if self.steps.is_empty() {
            println!("║ (No steps required - Goal satisfied)   ║");
        } else {
            for (i, step) in self.steps.iter().enumerate() {
                // Formatting: {:02} = 2 digits, {:<34} = left align, 34 width
                println!("║ {:02}. {:<34} ║", i + 1, step);
            }
        }

        println!("╠════════════════════════════════════════╣");
        println!("║ Stats:                                 ║");
        // Print time in microseconds if small, or milliseconds if large
        if self.stats.duration.as_millis() > 0 {
            println!("║  • Time:      {:<24?} ║", self.stats.duration);
        } else {
            println!("║  • Time:      {:<24?} ║", self.stats.duration);
        }

        println!("║  • Expanded:  {:<24} ║", self.stats.nodes_expanded);
        println!("║  • Generated: {:<24} ║", self.stats.nodes_generated);
        println!("║  • Total Cost:{:<24.2} ║", self.total_cost);
        println!("╚════════════════════════════════════════╝\n");
    }
}