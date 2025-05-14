# Lunar Grid Guardian - TUI Development Plan

This plan prioritizes getting a playable slice of the game up and running as quickly as possible, then iteratively adding features.

**Phase 1: Core Loop & Minimal Viable Gameplay (Focus: See something working)**

1.  **Project Setup (Rust & TUI):**
    *   Initialize Rust project: `cargo new lunar_grid_guardian --bin`.
    *   Add `ratatui` (or `tui-rs`) and a basic time crate (e.g., `chrono`) to `Cargo.toml`.
    *   Create a basic TUI layout with a placeholder for the game area.
    *   Implement the main game loop that ticks (e.g., once per second for now) and redraws the TUI.
2.  **Time Management (Simplified):**
    *   Implement a simple `Mission Timer` that just counts real-world seconds elapsed. Display this in the TUI.
3.  **Core Data Structures (Minimal):**
    *   Define `ColonyHealth` (0-100%).
    *   Define `TotalGridSupply` and `TotalGridDemand` (as simple `f32` or `i32` for now).
4.  **Simplest Power System - Reactor (Always On):**
    *   **Logic:**
        *   Implement a `ReactorPowerOutput` variable. For now, make it a fixed value.
        *   Set `TotalGridSupply` to be equal to `ReactorPowerOutput`.
    *   **UI (GO Panel - Minimal):**
        *   Display `TotalGridSupply` (simple text).
        *   Display `ColonyHealth` (simple text or a basic bar).
5.  **Simplest Demand System - Life Support (Basic Drain):**
    *   **Logic:**
        *   Implement a `LifeSupportPowerDemand` variable (fixed value).
        *   Set `TotalGridDemand` to this value.
        *   If `TotalGridSupply < TotalGridDemand`, start slowly decreasing `ColonyHealth`.
        *   If `ColonyHealth` reaches 0, print "Game Over" and stop the loop (basic game over).
    *   **UI (GO Panel - Minimal):**
        *   Display `TotalGridDemand` (simple text).
        *   Update `ColonyHealth` display.
6.  **Basic Input - Quit:**
    *   Allow the player to quit the game (e.g., pressing 'q').

*(At this point, you should have a very rudimentary "game": a timer runs, health might decrease if supply < demand, and you can quit. It's not much, but it's a running loop with some state and UI.)*

**Phase 2: Introducing Core Mechanics & Interaction**

7.  **Grid Frequency Dynamics (Simplified):**
    *   **Logic:**
        *   Calculate `PowerImbalance = TotalGridSupply - TotalGridDemand`.
        *   Implement a simplified `FrequencyDeviation`. If `PowerImbalance` is not zero, make `FrequencyDeviation` change (e.g., `FrequencyDeviation += PowerImbalance * 0.01`).
        *   If `FrequencyDeviation` goes beyond a simple threshold (e.g., +/- 5.0 from a target of 50Hz), rapidly decrease `ColonyHealth`.
    *   **UI (GO Panel):**
        *   Display `Frequency Imbalance` (e.g., as a number, target 50Hz).
8.  **Reactor Control (Basic):**
    *   **Logic:**
        *   Allow player to increase/decrease `ReactorPowerOutput` via key presses (e.g., '+' and '-').
        *   `TotalGridSupply` now dynamically updates based on this.
    *   **UI (Reactor Sub-Panel - Minimal):**
        *   Display current `ReactorPowerLevelOutput`.
        *   Add text indicating keys to control it.
9.  **Life Support Control (Emergency Restrictions):**
    *   **Logic:**
        *   Implement `Emergency Restrictions Toggle` (e.g., press 'e').
        *   If active, significantly reduce `LifeSupportPowerDemand` but also apply `ColonyStatusDrainRateEmergency` to `ColonyHealth`.
        *   If inactive, `LifeSupportPowerDemand` is normal, and `ColonyHealth` might slowly rebuild if supply is sufficient (`ColonyStatusRebuildRate`).
    *   **UI (Life Support Sub-Panel - Minimal):**
        *   Display `Life Support Power Level`.
        *   Display `Emergency Restrictions Status Indicator` (e.g., "ON" / "OFF").
        *   Display `Colony Health Level (Local Reference)`.

*(Now, the player can interact with the reactor to balance supply and make a choice with Life Support that affects demand and health. The frequency mechanic adds another layer of challenge.)*

**Phase 3: Expanding Power Generation & Storage**

10. **Lunar Day/Night Cycle & Solar Power (Basic):**
    *   **Logic:**
        *   Implement the `current_lunar_time` and `lunar_day_duration` (from spec).
        *   Implement the solar power generation formula: `abs(sin(2 * PI * current_lunar_time / lunar_day_duration)) * MaxSolarOutput`.
        *   Add `SolarPowerOutput` to `TotalGridSupply`.
    *   **UI (Solar Sub-Panel - Minimal):**
        *   Display `Solar Power Level Output`.
        *   Display `Lunar Sunset/Sunrise Countdowns` (real-world seconds for now).
11. **Battery Storage (Basic Charge/Discharge):**
    *   **Logic:**
        *   Implement `BatteryChargeLevel` (0-100%).
        *   Implement manual "Charge" and "Discharge" modes (toggle with a key).
        *   If "Charge": Battery draws power from grid (adds to `TotalGridDemand`), increasing `BatteryChargeLevel`.
        *   If "Discharge": Battery adds power to grid (adds to `TotalGridSupply`), decreasing `BatteryChargeLevel`.
        *   Ensure charge doesn't go below 0 or above 100.
    *   **UI (Battery Sub-Panel - Minimal):**
        *   Display `Battery Charge Level`.
        *   Display `Mode Status Indicator` (Charge/Discharge).

*(The game now has multiple power sources and a storage system, making resource management more complex, especially with the day/night cycle.)*

**Phase 4: Introducing System Integrity, Events & Comms**

12. **System Status Indicators (RAG - Basic):**
    *   **Logic:** For now, assume all systems are "Green".
    *   **UI (SI Panel - Minimal):**
        *   Display static "Green" indicators for Reactor, Solar, Batteries, Life Support.
13. **Comms System (Basic Online/Offline):**
    *   **Logic:**
        *   Implement `CommsPowerDemand`. Add to `TotalGridDemand` if Comms are "Online".
        *   Implement `Online/Offline Toggle` for Comms.
    *   **UI (Comms Sub-Panel - Minimal):**
        *   Display `Online/Offline Status Indicator`.
        *   Update RAG status for "Comms" on SI Panel.
14. **First Event - Micrometeorites (Simplified):**
    *   **Logic:**
        *   On a timer, trigger a "Micrometeorite" event.
        *   If it occurs, and Solar Shields are NOT active (implement a simple toggle for shields first, no power effect yet), set Solar RAG status to "Red" (simulating damage) and reduce `SolarPowerOutput` to 0.
        *   If Comms are "Offline", the event happens with no warning. If "Online", display a warning message briefly before it hits.
    *   **UI (SI Panel):**
        *   Display `Event Alerts` when an event is active or warned.
        *   Update Solar RAG status.
    *   **UI (Solar Sub-Panel):**
        *   Add `Solar Shield Toggle` and `Status Indicator`.

*(Events start adding unpredictability. Comms provide a benefit. Damage is introduced simply for now.)*

**Phase 5: Refining Systems & Adding Depth**

15. **Damage & Repair (Simplified for Solar):**
    *   **Logic:**
        *   When Solar is "Red" (damaged), allow player to "Initiate Solar Repair" (key press).
        *   Repair takes a fixed amount of time (real-world seconds). During repair, Solar RAG is "Throbbing Amber" (or just "Amber").
        *   After repair time, Solar RAG becomes "Green" and output is restored.
    *   **UI (Solar Sub-Panel):**
        *   Add `Initiate Solar Repair Button` (text indicating key).
        *   Update `Solar Array Damage Level` display (can be just text "OK"/"Damaged"/"Repairing").
16. **Full Grid Frequency Dynamics (Section 1.3):**
    *   Implement the proper RoCoF calculation using `SystemInertiaH` and dynamic `SystemNominalPowerPnom`.
17. **Detailed System Mechanics & UI:**
    *   Gradually implement the full details for each panel as per the spec:
        *   **Solar:** Full damage model, shield power effect.
        *   **Battery:** Auto mode, damage effects, repair.
        *   **Reactor:** Heat, coolant, overheating, damage, repair.
        *   **Life Support:** Power demand increase over time.
        *   **Operations:** Power demand, docking, supply drops (initially just award one type of boost).
18. **Boost Items (One type first - e.g., Repair Boost):**
    *   **Logic:**
        *   When Operations successfully "docks", award a "Repair Boost" item.
        *   Implement "Use Repair Boost" button. When used, it sets all damaged systems (e.g., Solar if damaged) back to "Green" / full health.
    *   **UI (Operations Sub-Panel):**
        *   Display `Item Counts` (for Repair Boost).
        *   Add `Authorize Docking Button` and `Use Item Button` (for Repair Boost).
        *   Display `Next Supply Drop Timer`.
    *   **UI (SI Panel):**
        *   Add `Boost Applied Indicator` for Repair Boost.

**Phase 6: Completing Features & Polishing**

19. **Implement All Event Types & Effects:**
    *   Lunar Quakes (damage Reactor, Batteries).
    *   Solar Flares (damage Solar, Batteries via spike).
    *   Implement event acknowledgement on Comms panel.
20. **Implement All Boost Items & Effects.**
21. **Full UI Implementation:**
    *   Implement all displays and controls as per the visual styles suggested in the spec, using appropriate `ratatui` widgets (Gauges, BarCharts, Tables, etc.).
    *   Refine layout and styling.
22. **Balancing & Tuning:**
    *   Load all variables from Table 1 from a config file.
    *   Extensive playtesting and adjustment of these variables.
23. **Code Organization, Refactoring, Testing, Documentation.**
