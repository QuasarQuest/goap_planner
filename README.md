# 🧠 Goal-Oriented Action Planning (GOAP)

**A Domain-Agnostic, Type-Safe GOAP Library for Rust.**

## 1. Overview

This a high-performance **Goal-Oriented Action Planning (GOAP)** library written in Rust. It operates strictly on the **Tactical Layer** of an autonomous agent.

This crate solves the question of **"What to do?"** (Sequencing) while remaining completely agnostic to **"How to do it?"** (Execution). It relies on a symbolic, abstract representation of the world rather than raw physical data, ensuring that your planning logic remains decoupled from hardware specifics.

### The "Agnostic" Philosophy

The planner does not know about specific object IDs, coordinates, or physics. It operates purely on **abstract capabilities and boolean states**.

| Approach | Logic | Verdict |
| --- | --- | --- |
| **Instance Specific** | *"Move to Tree_ID_42 at (10, 20). If dist < 1.0m, Pick Apple_99."* | ❌ **Brittle.** Fails if IDs change or trees move. |
| **GOAP (Tactical)** | **Goal:** `Has(Apple)` <br>

<br> **State:** `at_tree: true`, `tree_has_apple: true` <br>

<br> **Plan:** `Climb` -> `Pick` | ✅ **Robust.** Works for any tree, anywhere. |

---

## 2. High-Level Architecture

The system maintains a strict separation between **Static Configuration** (Repositories), **Dynamic Planning** (Runtime), and **External Reality** (I/O).

```mermaid
graph TD
%% --- THEME & STYLING ---
%% Logic: Active processing nodes (Blue)
    classDef logic fill:#e3f2fd,stroke:#1e88e5,stroke-width:2px,color:#0d47a1
%% Storage: Databases/Registries (Amber)
    classDef storage fill:#fff8e1,stroke:#ffb300,stroke-width:2px,color:#000,shape:cylinder
%% Definition: Static configs (Grey)
    classDef defs fill:#f5f5f5,stroke:#9e9e9e,stroke-dasharray: 5 5,color:#616161
%% Interface: API boundaries (Purple)
    classDef interface fill:#f3e5f5,stroke:#8e24aa,stroke-width:2px,color:#4a148c
%% IO: External interactions (Green)
    classDef io fill:#e8f5e9,stroke:#43a047,stroke-width:2px,color:#1b5e20

%% --- 1. REPOSITORY LAYER (CONFIGURATION) ---
    subgraph Repositories ["1. Repository Layer (Configuration & State)"]
        direction LR
        
        subgraph Repo_Action ["Action System"]
            ActionDef["Action Definitions"]:::defs
            ActionManager["Action Manager"]:::logic
            ActionRegistry[("Action Registry<br/>(Static)")]:::storage
            ActionDef -.-> ActionManager <--> ActionRegistry
        end

        subgraph Repo_Trait ["Trait System"]
            TraitDef["Capability Definitions"]:::defs
            TraitManager["Trait Manager"]:::logic
            TraitRegistry[("Trait Registry<br/>(Static)")]:::storage
            TraitDef -.-> TraitManager <--> TraitRegistry
        end

        subgraph Repo_World ["WorldState System"]
            WSDef["WorldState Struct"]:::defs
            WSManager["WorldState Manager"]:::logic
            WSRegistry[("WorldState Registry<br/>(Dynamic)")]:::storage
            WSDef -.-> WSManager <--> WSRegistry
        end

        subgraph Repo_Goal ["Goal System"]
            GoalDef["Goal Definitions"]:::defs
            GoalManager["Goal Manager"]:::logic
            GoalRegistry[("Goal Registry<br/>(Static)")]:::storage
            GoalDef -.-> GoalManager <--> GoalRegistry
        end
    end

%% --- 2. RUNTIME LAYER (PLANNING) ---
    subgraph Runtime ["2. Runtime Layer <br/>(GOAP Planner)"]

    %% Interfaces act as the bridge between Repo and Logic
        subgraph Interfaces ["Data Interfaces"]
            GoalInt[["Goal Selector"]]:::interface
            ActionInt[["Action Interface"]]:::interface
            TraitInt[["Trait Interface"]]:::interface
            WSInt[["WorldState Interface"]]:::interface
        end

    %% The Filtering Logic (STATIC ONLY!)
        subgraph Filtering ["Affordance Filter"]
            TraitCheck{{"Trait Check<br/>(Hardware)"}}:::logic
        end

        Planner("GOAP Planner"):::logic
    end

%% --- 3. I/O LAYER ---
    subgraph IO ["3. External I/O"]

        ExtData[/"External Data"/]:::io
        MissionCmd[/"Mission Command"/]:::io
        GoalSelector[/"Goal Selector"/]:::io
        ActionExecutor[/"Action Executor"/]:::io

    %% WS Update Loop
        WSUpdateInt[["WorldState Update Interface"]]:::interface
    end

%% --- CONNECTIONS ---

%% 1. Input Flow
    GoalSelector == "Goal ENUM" ==> GoalInt
    MissionCmd == "Run Command" ==> Planner
    ExtData ==> WSUpdateInt
    WSUpdateInt -.-> WSManager

%% 2. Registry Access
    GoalRegistry -.-> GoalInt
    ActionRegistry -.-> ActionInt
    TraitRegistry -.-> TraitInt
    WSRegistry -.-> WSInt

%% 3. Planning Flow
    GoalInt -- "Goal" --> Planner

    ActionInt --> TraitCheck
    TraitInt --> TraitCheck

%% Filter (Static) -> Planner -> Context (Dynamic)
    TraitCheck -- "Feasible Actions" --> Planner
    WSInt -- "Dynamic Context" --> Planner

%% 4. Output
    Planner == "Action Plan" ==> ActionExecutor

```

### Key Layers

1. **Repositories:** Static definitions of what the agent *is* (Traits), what it can *do* (Actions), and what it *wants* (Goals).
2. **Runtime:** The active decision-making layer. It filters actions based on hardware traits (e.g., "Do I have a thermal camera?") before planning.
3. **I/O:** The bridge to the external application, handling sensor fusion and executing the final plan.

---

## 3. Internal Logic: The Orchestrator Pattern

Internally, GOAP uses an **Orchestrator Pattern** to ensure concurrency safety and clean state management. The `PlanningOrchestrator` wraps the raw A* Engine, handling lifecycle events (Idle -> Running -> Done) and interfacing with the outside world.

```mermaid
graph TD
%% --- THEME & STYLING ---
    classDef logic fill:#e3f2fd,stroke:#1e88e5,stroke-width:2px,color:#0d47a1
    classDef storage fill:#fff8e1,stroke:#ffb300,stroke-width:2px,color:#000,shape:cylinder
    classDef interface fill:#e1bee7,stroke:#4a148c,stroke-width:2px,color:#4a148c
    classDef io fill:#e8f5e9,stroke:#43a047,stroke-width:2px,color:#1b5e20
    classDef orchestrator fill:#ffcdd2,stroke:#c62828,stroke-width:2px,color:#b71c1c

    subgraph Wrapper [GOAP Planner]
        direction TB

    %% --- 1. INTERFACES ---
        subgraph Inputs [Inputs]
            GoalInt[["Goal Interface"]]:::interface
            ActionInt[["Action Interface"]]:::interface
            WSInt[["WorldState Interface"]]:::interface
        end

    %% --- 2. ORCHESTRATOR (The Logic Binder) ---
    %% This node manages the state machine (Idle -> Running -> Success)
        Orchestrator{{"Planner Orchestrator**<br/>State Machine"}}:::orchestrator

    %% --- 3. CORE ALGORITHM ---
        subgraph A_Star_Engine [A* Core Logic]
            direction TB

        %% STORAGE
            subgraph Registries [Read-Only Data]
                ActionReg[("Action Registry")]:::storage
                GoalReg[("Goal Registry")]:::storage
                WSReg[("Worldstate Registry")]:::storage
            end

            subgraph Memory [Search Memory]
                OpenSet[("OpenSet")]:::storage
                ClosedSet[("ClosedSet")]:::storage
            end

        %% LOGIC LOOP
            PopNode[Pop Lowest Cost]:::logic

            subgraph Checks [Termination Checks]
                GoalCheck{Goal Met?}:::logic
                OpenSetCheck{OpenSet Empty?}:::logic
            end

            subgraph Node_Gen [Expansion Logic]
                ActionFilter[Filter Actions]:::logic
                CloneWS[Clone State]:::logic
                Apply["Apply Effects"]:::logic
                Calc[Calculate Costs]:::logic
                NewNode[Create Node]:::logic
            end
        end

    %% --- 4. OUTPUTS ---
        subgraph Outputs [Outputs]
            ActionPlan[["Action Plan"]]:::interface
            PlannerStatus[["Planner Status<br/>(Idle / Running / Done)"]]:::interface
        end
    end

%% ================= CONNECTIONS =================

%% 1. Setup Phase
    ActionInt & GoalInt & WSInt ==> Orchestrator
    Orchestrator -.-> |"Updates"| PlannerStatus

    Orchestrator --> |"Init Data"| Registries
    Orchestrator --> |"Push Start Node"| OpenSet

%% 2. The Search Loop
    OpenSet --> OpenSetCheck
    OpenSetCheck -- "No (Continue)" --> PopNode
    PopNode --> GoalCheck

%% 3. Goal Logic (Read from Registry)
    GoalReg -.-> GoalCheck

%% 4. Failure/Success Paths
    OpenSetCheck -- "Yes (Empty)" --> Orchestrator
    GoalCheck -- "Yes (Success)" --> Orchestrator

%% 5. Expansion Path
    GoalCheck -- "No" --> ActionFilter
    ActionReg -.-> |"Read Actions"| ActionFilter

    ActionFilter --> CloneWS
    WSReg -.-> |"Read Constants"| CloneWS

    CloneWS --> Apply --> Calc --> NewNode
    NewNode --> OpenSet
    NewNode -.-> ClosedSet
    PopNode -.-> ClosedSet

%% 6. Final Output
    Orchestrator ==> |"Publish"| ActionPlan

```

### Component Breakdown

* **Planner Orchestrator:** The public API surface. It manages the `PlannerStatus` and triggers the search.
* **Registries:** Read-only storage for the `WorldState` layout and available `Actions`.
* **A* Engine:** The core algorithm that expands nodes.
* **CloneWS:** Efficiently clones the state for simulation.
* **Apply Effects:** Mutates the cloned state based on Action definitions.
* **Calc:** Computes G-Cost (Path) and H-Cost (Heuristic).



---

## 4. Data Contracts (Traits)

To use this library, the external system must implement the core traits defined in `src/interface/traits.rs`.

### 1. The WorldState Contract

The external system is responsible for Sensor Fusion. It must distill complex physics into a `WorldState` struct that implements the library's trait.

| Physical Reality (External) | Abstraction (Internal WorldState) |
| --- | --- |
| `UserPos: (10, 10), TreePos: (10, 11)` | `is_at_tree: True` |
| `Battery Voltage: 10.2V` | `battery_state: Critical` |

### 2. Logic vs. Cost Philosophy

The planner uses **Extreme Cost Abstraction** to handle soft constraints without requiring complex conditional logic inside the planner.

* **Logic (Strict):** Uses only **Boolean** or **Enum**.
* *Rule:* "I can `Fly` if `is_flying` is True."


* **Cost (Soft):** Uses **Floats** to discourage behavior.
* *Rule:* If `battery_state` is `Critical`, the `Fly` action remains valid, but its Cost increases to `10,000`.
* *Result:* The planner naturally avoids flying unless absolutely necessary.



---

## 5. Integration Workflow

To integrate GOAP into your Rust application:

1. **Define WorldState:** Create a struct representing your domain (e.g., `RobotState`) and implement the `WorldState` trait.
2. **Define Actions:** Create structs for your capabilities (e.g., `Move`, `Scan`) implementing the `Action` trait.
3. **Instantiate Manager:** Initialize the `PlanningOrchestrator`.
4. **Run:** Pass the Start State, Goal State, and Action List to the manager.