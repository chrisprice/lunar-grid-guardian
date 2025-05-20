# **Game Specification: Lunar Grid Guardian**

## **1\. Core Gameplay Dynamics & Pacing**

This section outlines the fundamental mechanics and the intended rhythm of the game. The player's primary goal is to maintain grid stability and ensure colony survival for as long as possible by interacting with a series of detailed control and readout panels. Success hinges on balancing fluctuating power generation against dynamic colony demands, managing system integrity, and responding to unpredictable events. The core challenge is amplified by the lunar day/night cycle, which directly influences resource availability and operational strategies.

### **1.1. Lunar Day/Night Cycle and Game Pacing**

The game's challenge and rhythm are driven by the lunar day/night cycle, directly affecting solar power generation (Solar Power Sub-Panel) and influencing strategic decisions across all panels.

* **Day Cycle:** Solar power is generally available. This period allows for building energy reserves (Battery Storage Sub-Panel), potentially running the Operations Sub-Panel to collect **Boost Items** (Life Support, Battery, Coolant, Repair), and optimizing overall system health (SI Panel). The GO Panel will reflect a higher *potential contribution from solar power to the total grid supply*.  
* **Night Cycle:** Solar power diminishes or ceases. This increases pressure, forcing reliance on stored energy (Batteries), reactor power (Reactor Power Sub-Panel), and careful load balancing (DM Panel). The GO Panel will show a *reduced or absent contribution from solar power to the total grid supply*, increasing reliance on other sources.  
* **Rhythm of Play:** The day/night transitions, signaled by solar countdowns, create an ebb and flow in intensity. Players must adapt using information from the SI and GO Panels and controls on the GC and DM Panels. Surviving multiple cycles, tracked by the Mission Timer, demonstrates skill in resource management, crisis aversion, and strategic use of collected **Boost Items**.

### **1.2. General Note on Data Representation**

All system level values (e.g., damage levels, charge levels, coolant levels, colony damage) should be stored internally as a percentage (ranging from 0% to 100%). This allows for precise calculations and consistent representation. UI displays can still use different visual interpretations (e.g., segmented bars, gauges) but the underlying value will be 0-100%.

### **1.3. Grid Frequency Dynamics and Power Imbalance**

The stability of the colony's power grid, visually represented by the "Frequency Imbalance" metric on the GO Panel (see Section 2.1.1), is a critical aspect of gameplay. When there's a sudden change in total grid supply versus total grid demand (e.g., a power generation system trips offline, a high-demand system like Operations docking sequence initiates, or the player rapidly changes system states), the grid frequency will deviate from its nominal target. The rate and magnitude of this deviation are dynamically calculated based on the principles of power system physics, specifically the swing equation.

This equation models the rotational dynamics of the aggregated power generation system:

f0​2H​dtd(Δf)​=Pnom​ΔP​  
Or, focusing on the Rate of Change of Frequency (RoCoF), which dictates how quickly the "Frequency Imbalance" display will react:

dtd(Δf)​=2H⋅Pnom​f0​⋅ΔP​  
Where these terms apply to the game's simulation:

* H is the **System Inertia Constant** (in seconds), representing the inherent resistance to frequency changes within the colony's power grid. A higher inertia (e.g., more generators online and spinning) means the frequency changes more slowly in response to an imbalance. This is a key balancing parameter (see SystemInertiaH in Table 1).  
* f0​ is the **Nominal System Frequency**, which for this colony is **50 Hz** (as defined in Section 2.1.1).  
* Δf is the **Frequency Deviation** from f0​, the value displayed on the "Frequency Imbalance" gauge.  
* dtd(Δf)​ is the **Rate of Change of Frequency (RoCoF)** (in Hz/s), determining how rapidly Δf changes.  
* ΔP is the **Power Imbalance** (in Power units for game purposes), calculated as (Total Grid Supply \- Total Grid Demand). These values are displayed on the GO Panel.  
* Pnom​ is the **Nominal System Power Capacity** (in Power units for game purposes), representing the total rated power capacity of all currently operational and connected power generation sources. This value will dynamically change as generators (Solar, Reactor) are brought online/offline or their output capability changes (e.g., due to damage or solar intensity). This is a key balancing parameter (see SystemNominalPowerPnom in Table 1, which may represent a baseline or be dynamically calculated).

This model means that large, sudden differences between power supply and demand will cause rapid changes in grid frequency. The player's ability to manage ΔP by adjusting generation and load, and the inherent H and Pnom​ of the system, will determine if the frequency can be kept within the safe operational limit of ±0.5Hz (see Section 2.1.1).

## **2\. Panel Specifications**

### **2.1. Readout Panels**

Readout panels are primarily designed to display critical game information to the player. Their displays should be large and clear enough to be easily viewable by people observing the player, allowing them to follow the game's unfolding events and system statuses.

#### **2.1.1. Grid Overview (GO) Panel**

This panel serves as the main display for the player, showing key metrics related to grid status, colony health, and game progression.

**Key Displays & Metrics:**

* **Total Grid Demand:** Displays the current total power being demanded by all active colony systems (in Power units).  
  * *Visual Style:* This could be, for example, an analog-style gauge.  
* **Total Grid Supply:** Displays the current total power being supplied by all active generation sources (in Power units).  
  * *Visual Style:* This could also be, for example, an analog-style gauge, perhaps visually paired with the Demand gauge.  
* **Frequency Imbalance (Primary Health Metric 1):**  
  * Represents the stability of the power grid's frequency. The nominal target frequency is 50Hz.  
  * Displayed as a health bar or a gauge showing deviation from the target frequency.  
  * **Game Over Condition:** If the frequency deviates by more than **±0.5Hz** from the target, it results in a game over.  
  * *Visual Style:* A central, prominent gauge that clearly shows the acceptable range and the current frequency. It could pulse or change color as it nears critical limits.  
* **Colony Damage (Primary Damage Metric 2):**  
  * Represents the overall well-being or operational status of the colony.  
  * Displayed as a percentage (0-100%, where 0% is no damage and 100% is critical).  
  * **Game Over Condition:** If Colony Damage reaches 100%, it results in a game over.  
* **Mission Timer:**  
  * Displays the elapsed time the player has successfully managed the grid, represented as "scaled in-game time."  
  * Time Format: Recorded and displayed as Lunar Days, Hours, Minutes, and Seconds.  
  * Scaling Factor: A scaling factor will be applied to convert real elapsed time to this in-game lunar time (refer to MissionTimeScaleFactor in Table 1: Game Variables for Balancing).  
  * **Primary Score Metric:** This in-game time serves as the primary score for the player.  
  * *Visual Style:* For example, a Nixie-tube style digital display could be used to provide a retro feel, showing the formatted lunar time.

#### **2.1.2. System Integrity (SI) Panel**

This panel provides the player with critical information about ongoing events, upcoming threats (if communications are operational), the status of key colony systems, and recently applied boosts.

**Key Displays & Metrics:**

* **Event Alerts Display:**  
  * Shows alerts for currently active/in-progress events.  
  * Examples of events: Micrometeorite impacts, Lunar Quakes, Solar Flares.  
  * *Visual Style:* A dedicated section with, for instance, illuminated text warnings or icons representing the event type.  
* **Upcoming Event Timers:**  
  * Displays countdown timers (in **real-world seconds**) to the next anticipated events.  
  * **Condition:** These timers are only visible/active if the "Comms" system is online.  
  * *Visual Style:* For example, digital readouts next to an icon or text indicating the type of upcoming event.  
* **System Status Indicators (RAG Status):**  
  * A series of indicators showing the status of individual key colony systems.  
  * **Systems to Monitor:**  
    * Solar  
    * Batteries  
    * Reactor  
    * Life Support  
    * Comms  
    * Operations (e.g., Exterior Ops/Research)  
  * **Status Meanings & Visuals (General):**  
    * **Green:** System online, no damage. (Steady light)  
    * **Amber:** System online, but damaged. (Steady light)  
    * **Red:** System offline due to critical damage or other critical fault (this does not include systems manually taken offline for repair, which are Throbbing Amber). (Steady light)  
    * **Throbbing Amber:** System is currently undergoing repairs (e.g., manually taken offline by the player for repair, or an automated repair process is active). (Pulsing light)  
  * **Specific Note for Life Support RAG Status:**  
    * If Emergency Restrictions for Life Support are **ACTIVE**, its RAG status on this SI panel will be **Red**, overriding other damage/operational states for this indicator. Otherwise, it is Green (as Life Support itself doesn't take damage or go offline in other RAG-defined ways). The specific "Emergency Restrictions Status Indicator" on the Life Support sub-panel will also be active.  
  * *Visual Style:* For example, a matrix of labeled indicator lights, each corresponding to a system and capable of displaying the specified colors and throbbing effect.  
* **Boost Applied Indicators (x4):**  
  * Four distinct indicators, one for each type of **Boost Item** from the Operations Sub-Panel (Life Support Boost, Battery Boost, Coolant Boost, Repair Boost).  
  * When a **Boost Item** is used, the corresponding indicator pulses for **3 real-world seconds** to provide visual feedback that the (instantaneous) boost has been applied.  
  * *Visual Style:* Clearly labeled indicator lights (e.g., using icons or text for each boost type) that can pulse with a distinct color (e.g., bright blue or white) when a boost is activated.

### **2.2. Control Panels**

Control panels contain the primary interactive elements (switches, dials, buttons) that allow the player to manage the colony's power systems.

#### **2.2.1. Generation Control (GC) Panel**

This panel houses the sub-panels dedicated to managing the colony's primary power generation and storage systems.

##### **2.2.1.1. Solar Power Sub-Panel**

This sub-panel allows the player to monitor and manage the solar array network.

**Functionality:**

* **Power Generation:** Solar power generation is dependent on light intensity, which varies based on the lunar day/night cycle.  
  * Light Intensity Model: Follows a function like abs(sin(2 \* PI \* current\_lunar\_time / lunar\_day\_duration)). current\_lunar\_time is a continuous variable representing elapsed in-game time. lunar\_day\_duration refers to the duration of one complete lunar day/night cycle.  
  * *Note on current formula:* The function abs(sin(2 \* PI \* current\_lunar\_time / lunar\_day\_duration)) will result in solar power availability peaking twice within one full lunar\_day\_duration. If a single peak during one continuous 'daylight' phase per lunar\_day\_duration is intended, the formula may need adjustment (e.g., max(0, sin(PI \* current\_lunar\_time\_in\_day\_phase / daylight\_duration)) or (cos(2 \* PI \* current\_lunar\_time\_in\_cycle / lunar\_day\_duration \- PI) \+ 1\) / 2).  
  * Output: Power generated is directly proportional to the calculated light intensity.  
* **Vulnerabilities:**  
  * **Solar Flares:** Exposure can damage the solar arrays. This can be mitigated.  
  * **Micrometeorite Impacts:** Can cause damage to Solar arrays if the Solar Shields are not active (damage amount defined by MicrometeoriteDamageSolar in Table 1). Unlike Solar Flares, Micrometeorite impacts do not cause power surges that damage other connected systems like Batteries. Mitigation is achieved by activating the Solar Shields.  
* **Solar Shields:**  
  * Activation prevents solar flare damage to solar arrays (and consequently to batteries from power surges) and also prevents damage to solar arrays from micrometeorite impacts.  
  * When active, solar shields stop all solar power generation.  
* **Repair:**  
  * Solar arrays can be taken offline to repair damage.  
  * Repair initiation is prevented if a solar flare or micrometeorite event's direct impact phase (lasting **3 real-world seconds**) is currently active.  
  * The RAG status for "Solar" on the SI Panel will reflect its condition (e.g., Amber for damaged, Throbbing Amber for repairing, Red for offline/destroyed).

**Controls:**

* **Solar Shield Toggle:** A switch or button to activate/deactivate the solar shields.  
* **Initiate Solar Repair Button:** A button to take the solar arrays offline and begin the repair process.  
  * *Visual Style:* Could, for instance, illuminate when repairs are possible/needed and become inactive (unlit or physically locked out) during unrepairable conditions.

**Displays:**

* **Solar Power Level Output:** Shows the current power being generated by the solar arrays (e.g., in Power units).  
  * *Visual Style:* Could be an analog-style gauge.  
* **Solar Array Damage Level:** Indicates the current damage percentage or status of the solar arrays.  
  * *Visual Style:* Could be a bar graph or a segmented light display.  
* **Lunar Sunset Countdown:** Displays a countdown (in **real-world seconds**) to the next lunar sunset, signaling the end of solar power availability for the current cycle.  
  * *Visual Style:* For example, a digital display, perhaps integrated near the power level output.  
* **Lunar Sunrise Countdown:** Displays a countdown (in **real-world seconds**) to the next lunar sunrise, signaling the beginning of solar power availability for the next cycle.  
  * *Visual Style:* For example, a digital display, perhaps integrated near the power level output or paired with the sunset countdown.  
* **Solar Shield Status Indicator:** A light indicating if solar shields are active or inactive.  
  * *Visual Style:* Could be integrated with the Solar Shield Toggle or a separate indicator light.

##### **2.2.1.2. Battery Storage Sub-Panel**

This sub-panel allows the player to manage the colony's battery storage system.

**Functionality:**

* **Charge Level:** Batteries maintain a charge level (0-100%), representing stored energy.  
* **Operating Modes:**  
  * **Auto Mode:** Automatically selects "Charge" or "Discharge" based on the balance between grid supply and demand. If supply exceeds demand, batteries will charge. If demand exceeds supply, batteries will discharge.  
  * **Charge Mode:** Prioritizes sending surplus grid power to charge the batteries.  
  * **Discharge Mode:** Forces batteries to supply power to the grid, even if other generation sources are available.  
* **Charging/Discharging:**  
  * When charging, batteries draw power from the grid.  
  * When discharging, batteries supply power to the grid.  
  * Batteries cannot overcharge; they will stop drawing power once full (100%).  
  * If depleted (charge level at 0%), batteries cannot supply power.  
* **Damage & Repair:**  
  * Damage (e.g., from Lunar Quakes, or from power spikes caused by Solar Flares if the Solar arrays are unshielded) can disable "Auto Mode" and reduce the maximum storage capacity of the batteries. The "Battery Damage Level" display reflects the extent of these effects. Shielding the Solar arrays (see Solar Power Sub-Panel 2.2.1.1) mitigates battery damage from Solar Flares.  
  * Repairs take the batteries offline for a specified duration (defined by BatteryRepairTimePerDamageUnit in Table 1), during which they cannot charge or discharge.  
  * The RAG status for "Batteries" on the SI Panel will reflect its condition.

**Controls:**

* **Mode Toggle:** A three-way switch or set of buttons to select "Auto", "Charge", or "Discharge" mode.  
  * *Visual Style:* For example, a rotary switch with clear labels for each mode.  
* **Initiate Battery Repair Button:** A button to take the batteries offline and begin the repair process.  
  * *Visual Style:* Similar to the Solar Repair button, it could, for instance, illuminate when repairs are possible/needed.

**Displays:**

* **Battery Charge Level:** Shows the current amount of energy stored in the batteries (percentage, 0-100%).  
  * *Visual Style:* For example, a large, vertical bar-graph display.  
* **Battery Damage Level:** Indicates the current damage percentage or status of the battery system.  
  * *Visual Style:* Could be a bar graph or a segmented light display.  
* **Mode Status Indicator:** Clearly indicates the currently selected operating mode (Auto, Charge, Discharge).  
  * *Visual Style:* For example, illuminated indicators next to the mode toggle positions.

##### **2.2.1.3. Reactor Power Sub-Panel**

This sub-panel allows the player to monitor and manage the colony's reactor.

**Functionality:**

* **Power Generation:** The reactor provides a consistent power output, adjustable by the player.  
* **Heat Management:**  
  * Increasing the reactor's power level output generates more heat.  
  * The reactor automatically increases coolant usage to mitigate temperature rises.  
* **Coolant System:**  
  * Coolant is drawn from a limited supply (maximum capacity defined by ReactorMaxCoolantPercentage in Table 1).  
  * This coolant supply refills at a constant rate (defined by ReactorCoolantRefillRate in Table 1), up to its maximum capacity.  
  * The effectiveness of the coolant in reducing reactor temperature is diminished if the reactor is damaged (reduction rate defined by CoolantEffectivenessReductionRate in Table 1, applied as a percentage reduction in cooling power).  
* **Overheating & Shutdown:**  
  * If the reactor runs out of coolant, it will overheat.  
  * Overheating causes **100% damage** to the reactor and triggers an emergency shutdown (meltdown averted), taking the reactor offline.  
* **Damage & Repair:**  
  * The reactor can be damaged by events (e.g., Lunar Quakes, overheating).  
  * Repairing the reactor takes it offline for a specified duration (defined by ReactorRepairTimePerDamageUnit in Table 1).  
  * The RAG status for "Reactor" on the SI Panel will reflect its condition.

**Controls:**

* **Reactor Power Level Dial:** A dial or slider to adjust the reactor's power output level.  
  * *Visual Style:* For example, a prominent, central, circular dial with satisfying detents.  
* **Initiate Reactor Repair Button:** A button to take the reactor offline and begin the repair process.  
  * *Visual Style:* Could, for instance, illuminate when repairs are possible/needed.

**Displays:**

* **Reactor Power Level Output:** Shows the current power being generated by the reactor (e.g., in Power units or as a percentage of max output).  
  * *Visual Style:* Could be an analog-style gauge.  
* **Reactor Temperature Level:** Indicates the current operating temperature of the reactor.  
  * *Visual Style:* For example, a thermometer-style illuminated gauge that changes color (e.g., blue to red) as temperature increases.  
* **Coolant Level:** Shows the current amount of coolant available in the reactor's limited supply (percentage, 0-100%).  
  * *Visual Style:* Could be a vertical bar-graph display or an illuminated gauge.  
* **Reactor Damage Level:** Indicates the current damage percentage or status of the reactor.  
  * *Visual Style:* Could be a bar graph or a segmented light display.

#### **2.2.2. Demand Management (DM) Panel**

This panel provides the player with controls to manage the power consumption of various critical and non-critical colony systems via its sub-panels.

##### **2.2.2.1. Life Support Sub-Panel**

This sub-panel is critical for maintaining colony viability and directly impacts the "Colony Damage" metric.

**Functionality:**

* **Colony Damage Impact:**  
  * When Life Support is operational (i.e., not in Emergency Restrictions mode), it slowly decreases the "Colony Damage" score (0-100%) (rate defined by ColonyDamageRepairRate in Table 1).  
  * Enabling "Emergency Restrictions" increases the "Colony Damage" score at an accelerated rate (defined by ColonyDamageIncreaseRateEmergency in Table 1).  
* **Power Demand:**  
  * Life Support has a base power demand (defined by LifeSupportBasePowerDemand in Table 1).  
  * Over time, the base power demand for Life Support will gradually increase (rate defined by LifeSupportPowerDemandIncrease in Table 1), adding to the game's difficulty scaling.  
* **Integrity:** Life Support is considered to be always drawing its required power when not in Emergency Restrictions. Its operational status is reflected on the SI Panel: Green for normal (drawing power, no restrictions), Red if 'Emergency Restrictions' are active. Failure of the grid to supply Life Support's power demand directly contributes to overall grid instability (e.g., Frequency Imbalance on GO Panel), which is a primary path to game over, rather than Life Support having an 'unpowered' RAG state separate from Emergency Restrictions.

**Controls:**

* **Emergency Restrictions Toggle:** A switch or button to activate/deactivate emergency power restrictions for Life Support.  
  * *Visual Style:* For example, a guarded toggle switch to signify its critical and potentially negative impact.

**Displays:**

* **Life Support Power Level:** Shows the current power being consumed by the Life Support system (in Power units).  
  * *Visual Style:* Could be an analog-style gauge.  
* **Colony Damage Level (Local Reference):** Displays the current "Colony Damage" score (0-100%), mirroring the display on the GO Panel for immediate reference when making decisions on this sub-panel.  
  * *Visual Style:* For example, a numerical display or segmented bar matching the GO Panel's display.  
* **Emergency Restrictions Status Indicator:** A light indicating if emergency restrictions are active.  
  * *Visual Style:* For example, a prominent warning light (e.g., flashing red or amber) when active.

##### **2.2.2.2. Comms Sub-Panel**

This sub-panel allows the player to manage the colony's communication systems, which are vital for early event warnings.

**Functionality:**

* **Power Demand:** Comms system draws a constant power load from the grid when online (defined by CommsPowerDemand in Table 1, in Power units). This load contributes to the "Total Grid Demand" on the GO Panel.  
* **Event Warnings:**  
  * When online, the Comms system provides advanced notice of upcoming events. This enables the "Upcoming Event Timers" on the SI Panel.  
  * **Event Types & Impacts:**  
    * **Micrometeorites:** Cause damage to unshielded Solar arrays (amount defined by MicrometeoriteDamageSolar in Table 1).  
    * **Lunar Quakes:** Cause damage to Reactor (amount defined by LunarQuakeDamageReactor in Table 1\) and Batteries (amount defined by LunarQuakeDamageBattery in Table 1).  
    * **Solar Flares:** Cause damage to Solar arrays (if unshielded) (amount defined by SolarFlareDamageSolarArray in Table 1). Unshielded Solar arrays experiencing a Solar Flare can also cause power spikes that damage Batteries (amount defined by SolarFlareSpikeDamageBattery in Table 1). Mitigation for both Solar arrays and Batteries against Solar Flare effects is achieved by activating the Solar Shields.  
  * If Comms are offline, no advanced notice is given for events, and the "Upcoming Event Timers" on the SI Panel will be inactive. Events will strike with **no warning**.  
* **Integrity:** The Comms system does not take damage and cannot be repaired. Its operational status is determined by the Online/Offline toggle. The RAG status for "Comms" on the SI Panel will be Green when online and Red when offline.

**Controls:**

* **Online/Offline Toggle:** A switch or button to turn the Comms system on or off.  
  * *Visual Style:* For example, a robust toggle switch.  
* **Event Alert Acknowledgement Buttons (x3):** Three separate buttons, one for each major event type (Micrometeorites, Lunar Quakes, Solar Flares).  
  * When an event alert is first received (and displayed on SI Panel and potentially here), the corresponding alert indicator on this sub-panel strobes red.  
  * Pressing the acknowledgement button for that event type changes its alert indicator from strobing red to constant amber. This action also silences an associated audio alarm for that specific event alert.  
  * Acknowledging an alert has no other gameplay effect beyond changing its visual indicator on this sub-panel and silencing the associated audio alarm.  
  * *Visual Style:* For example, illuminated push-buttons that can show different states (off, strobing red, constant amber).

**Displays:**

* **Event Alert Indicators (x3):** Three distinct indicators, one for each event type (Micrometeorites, Lunar Quakes, Solar Flares).  
  * Shows the status of an incoming/active alert (e.g., Off, Strobing Red for new unacknowledged alert, Constant Amber for acknowledged alert).  
  * *Visual Style:* For example, clearly labeled lights next to the acknowledgement buttons.  
* **Online/Offline Status Indicator:** A light indicating if the Comms system is currently active.  
  * *Visual Style:* For example, a simple green (online) / red (offline) indicator light.

##### **2.2.2.3. Operations Sub-Panel**

This sub-panel allows the player to manage non-critical colony operations that can provide strategic benefits.

**Functionality:**

* **Power Demand:** Operations draw a constant power load from the grid when online (defined by OperationsBasePowerDemand in Table 1, in Power units). This load spikes significantly (magnitude OperationsDockingSpikePower, duration OperationsDockingSpikeDuration in Table 1\) during the docking process for a **Boost Item** supply drop.  
* **Supply Drops (Boost Item Supply Drops):**  
  * When the Operations sub-panel is online, **Boost Item** supply drops occur on a periodic timer (interval defined by SupplyDropInterval in Table 1).  
  * The timer for the next supply drop is paused if a current supply drop is pending docking or is in the process of docking (i.e., from "Authorize Docking" press until docking sequence completes \- duration SupplyDropDockingDuration in Table 1).  
  * Docking can only be authorized if the Operations sub-panel is online.  
* **Boost Items (One is randomly awarded per successful docking):**  
  * **Life Support Boost:** Increases "Colony Damage" (0-100%) by a set amount (defined by BoostLifeSupportAmount in Table 1). This should decrease the damage value.  
  * **Battery Boost:** Increases charge level (0-100%) of all batteries by a set amount (defined by BoostBatteryAmount in Table 1). This boost cannot increase the charge level of batteries beyond their normal maximum capacity (100%).  
  * **Coolant Boost:** Increases the reactor's coolant level (0-100%) by a set amount (defined by BoostCoolantAmount in Table 1). This boost cannot increase the reactor's coolant level beyond its normal maximum capacity (100%).  
  * **Repair Boost:** Reduces all current system damage levels by a set percentage amount for all damageable systems (i.e., Solar, Batteries, and Reactor) (defined by BoostRepairAmount in Table 1).  
* **Integrity:** The Operations system does not take damage and cannot be repaired. Its RAG status on the SI Panel will be Green when online and Red when offline.

**Controls:**

* **Online/Offline Toggle:** A switch or button to turn the Operations system on or off.  
  * *Visual Style:* For example, a robust toggle switch.  
* **Authorize Docking Button:** A button to authorize an incoming supply drop to dock.  
  * *Visual Style:* Could, for instance, illuminate when a supply drop is pending and awaiting authorization.  
* **Use Item Buttons (x4):** Four separate buttons, one for each type of collected **Boost Item** (Life Support Boost, Battery Boost, Coolant Boost, Repair Boost).  
  * Pressing a button consumes one stored item of that type and applies its effect.  
  * *Visual Style:* For example, illuminated push-buttons, which may appear dimmed or inactive if the count for the respective item is zero or the item is otherwise not usable in the current context.

**Displays:**

* **Operations Power Level:** Shows the current power being consumed by the Operations system (in Power units). This will show a base level when online and a spike during docking.  
  * *Visual Style:* Could be an analog-style gauge.  
* **Item Counts (x4):** Four distinct numerical displays or segmented lights showing the current quantity of each stored **Boost Item**.  
  * *Visual Style:* For example, small digital readouts or a series of small lights for each item type, next to their respective "Use Item" button.  
* **Online/Offline Status Indicator:** A light indicating if the Operations system is currently active.  
  * *Visual Style:* For example, a simple green (online) / red (offline) indicator light.  
* **Pending Docking Indicator:** A light or display that signals a supply drop is approaching and awaiting authorization.  
  * *Visual Style:* For example, a distinct flashing light or a message on a small integrated screen.  
* **Next Supply Drop Timer:** Displays a countdown (in **real-world seconds**) to the next available **Boost Item** supply drop (paused during active docking).  
  * *Visual Style:* For example, a digital display.

## **3\. Game Variables for Balancing**

The following table lists parameters that require specific values to be assigned for game balancing and tuning. These are referenced throughout the document.

**Table 1: Game Variables for Balancing**

| Section Reference | Item Requiring Definition | Variable Name (Placeholder) | Units / Type of Value | Notes |
| :---- | :---- | :---- | :---- | :---- |
| 1.3 | System Inertia Constant | SystemInertiaH | Seconds | Higher value means frequency changes slower. |
| 1.3 | Nominal System Power Capacity | SystemNominalPowerPnom | Power units | Baseline total rated power of generation; may be dynamic. |
| 2.1.1 | Mission Timer \- Scaling Factor | MissionTimeScaleFactor | Scaled time factor (real seconds to game time unit) | e.g., 60 real seconds \= 1 game day/hour |
| 2.2.1.2 | Battery Repair Duration | BatteryRepairTimePerDamageUnit | Seconds per % damage | e.g., X seconds to repair 1% damage |
| 2.2.1.3 | Reactor Max Coolant Capacity | ReactorMaxCoolantPercentage | Percentage (0-100%) | Represents the maximum coolant storage. |
| 2.2.1.3 | Reactor Coolant Refill Rate | ReactorCoolantRefillRate | Percentage points per second | Rate at which coolant refills towards 100%. |
| 2.2.1.3 | Reactor Coolant Effectiveness Reduction Rate | CoolantEffectivenessReductionRate | % effectiveness loss per % reactor damage | How much cooling power is lost as reactor damage increases. |
| 2.2.1.3 | Reactor Repair Duration | ReactorRepairTimePerDamageUnit | Seconds per % damage | e.g., X seconds to repair 1% damage |
| 2.2.2.1 | Life Support \- Colony Damage Repair Rate | ColonyDamageRepairRate | Percentage points per second (for 0-100% scale) | Rate at which Colony Damage (0-100%) decreases when LS is normal. |
| 2.2.2.1 | Life Support \- Colony Damage Increase Rate (Emergency) | ColonyDamageIncreaseRateEmergency | Percentage points per second (for 0-100% scale) | Rate at which Colony Damage (0-100%) increases during LS Emergency Restrictions. |
| 2.2.2.1 | Life Support \- Base Power Demand | LifeSupportBasePowerDemand | Power units | Initial power demand. |
| 2.2.2.1 | Life Support \- Power Demand Increase Rate | LifeSupportPowerDemandIncrease | Power units per in-game day | How much power demand increases each game day. |
| 2.2.2.2 | Comms \- Power Demand | CommsPowerDemand | Power units | Constant power load when Comms are online. |
| 2.2.2.2 | Micrometeorite Damage (to unshielded Solar) | MicrometeoriteDamageSolar | Fixed value (damage %) | Percentage damage inflicted on unshielded Solar arrays. |
| 2.2.2.2 | Lunar Quake Damage (to Reactor) | LunarQuakeDamageReactor | Fixed value (damage %) | Percentage damage inflicted on the Reactor. |
| 2.2.2.2 | Lunar Quake Damage (to Batteries) | LunarQuakeDamageBattery | Fixed value (damage %) | Percentage damage inflicted on Batteries. |
| 2.2.2.2 | Solar Flare Damage (to unshielded Solar Arrays) | SolarFlareDamageSolarArray | Fixed value (damage %) | Percentage damage inflicted on unshielded Solar arrays by the flare itself. |
| 2.2.2.2 | Solar Flare Power Spike Damage (to Batteries via Solar Arrays) | SolarFlareSpikeDamageBattery | Fixed value (damage %) | Percentage damage to Batteries from power spike if Solar Arrays unshielded. |
| 2.2.2.3 | Operations \- Base Power Demand | OperationsBasePowerDemand | Power units | Power load when Operations online (not docking). |
| 2.2.2.3 | Operations \- Docking Power Spike Magnitude | OperationsDockingSpikePower | Power units (absolute increase) | Additional Power units drawn during docking. |
| 2.2.2.3 | Operations \- Docking Power Spike Duration | OperationsDockingSpikeDuration | Seconds | Duration of the increased power draw for docking. |
| 2.2.2.3 | Supply Drop \- Timer Interval | SupplyDropInterval | Seconds | Time between availability of new Boost Item supply drops. |
| 2.2.2.3 | Supply Drop \- Docking Sequence Duration | SupplyDropDockingDuration | Seconds | Time taken for the docking sequence to complete after authorization. |
| 2.2.2.3 | Boost Item \- Life Support Effect | BoostLifeSupportAmount | Flat amount (percentage points for 0-100% scale) | Percentage points to decrease Colony Damage (0-100%). |
| 2.2.2.3 | Boost Item \- Battery Effect | BoostBatteryAmount | Flat amount (percentage points) | Percentage points added to battery charge (0-100%). |
| 2.2.2.3 | Boost Item \- Coolant Effect | BoostCoolantAmount | Flat amount (percentage points) | Percentage points added to reactor coolant (0-100%). |
| 2.2.2.3 | Boost Item \- Repair Effect | BoostRepairAmount | Flat amount (damage % points) | Reduces system damage percentage by this many points (e.g., 50% to 40%). |

