use std::hint::black_box;
use std::time::Instant;

use goap_planner::missions::{ALL_MISSIONS, Mission};
use goap_planner::config::planner::PlannerConfig;
use goap_planner::core::planner::Planner;
use goap_planner::debug::byte_formatter::{print_state, DebugFormat};

fn main() {
    // ── Pick which mission to run ────────────────────────────────────────────
    let mission = &ALL_MISSIONS[2];
    run_single(mission);
    benchmark(mission, 100);
}

// ── Single run ───────────────────────────────────────────────────────────────

fn run_single(mission: &Mission) {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║  MISSION : {:<29} ║", mission.name);
    println!("╚══════════════════════════════════════════╝");
    print_state("Start", mission.start.0, DebugFormat::Hex);
    print_state("Goal ", mission.goal.0,  DebugFormat::Hex);
    println!("Actions : {}", mission.actions.len());

    let planner = Planner::new(mission.actions, PlannerConfig::default());

    match planner.plan(mission.start, mission.goal) {
        Ok(plan) => { println!("\n✅ PLAN FOUND!"); plan.print_report(); }
        Err(e)   => { println!("\n❌ ABORTED: {}", e); }
    }
}

// ── Benchmark ────────────────────────────────────────────────────────────────

fn benchmark(mission: &Mission, iterations: u32) {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║  BENCHMARK : {:<27} ║", mission.name);
    println!("║  Iterations: {:<27} ║", iterations);
    println!("╚══════════════════════════════════════════╝");

    let planner = Planner::new(mission.actions, PlannerConfig::default());

    let _ = planner.plan(mission.start, mission.goal);

    let t = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(planner.plan(mission.start, mission.goal));
    }
    let total = t.elapsed();
    let avg   = total / iterations;

    println!("  Total   : {:?}", total);
    println!("  Average : {:?}", avg);
    println!("  ops/sec : {:.0}", 1.0 / avg.as_secs_f64());
}