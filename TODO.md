# Project Roadmap

## Phase 1: Core Logic (Current)
- [x] Define Traits (`WorldState`, `Action`)
- [x] Implement A* Node (`Node`)
- [x] Implement Core Planner (`Planner::plan`)
- [ ] Add Unit Tests (`tests/simple_mission.rs`)

## Phase 2: Runtime Management
- [ ] Implement `Orchestrator` (Async wrapper for the Planner)
- [ ] Add `Status` Enum (Idle, Running, Success, Failure)
- [ ] Implement cancellation/timeout logic

## Phase 3: Advanced Features (Future)
- [ ] Add "Procedural Preconditions" (Dynamic constraints)
- [ ] Add "Action Parameters" (e.g., Move(x,y) instead of just MoveToTarget)