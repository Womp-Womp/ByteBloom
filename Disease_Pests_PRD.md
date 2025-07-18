# PRD: Advanced Disease and Pests System

## 1. Overview

This PRD outlines a more sophisticated disease and pest system for ByteBloom Gardens. The current system is very basic, with pests appearing randomly and having a simple, direct impact on plants. The new system will introduce a more complex and dynamic simulation of diseases and pests, with life cycles, spread mechanics, and more interesting interactions with the environment.

## 2. Current System

*   A `Pest` struct with a `PestType` enum and an `infestation_level`.
*   Pests appear on tiles, but there is no mechanism for them to spread.
*   The impact on plants is not clearly defined in the current code.
*   There is no concept of disease.

## 3. Proposed System: A Dynamic Ecosystem of Pests and Diseases

### 3.1. Disease

A new `Disease` system will be introduced, separate from pests.

*   **Disease Types:** There will be different types of diseases (e.g., fungal, bacterial, viral), each with its own characteristics.
*   **Symptoms:** Diseases will have visible symptoms on plants, which will be described in the UI (e.g., "The leaves are yellowing and have brown spots.").
*   **Transmission:** Diseases will spread from plant to plant through various vectors:
    *   **Airborne:** Spreads to adjacent plants.
    *   **Waterborne:** Spreads through the irrigation system.
    *   **Soilborne:** Lives in the soil and infects plants through their roots.
*   **Environmental Factors:** The likelihood of a disease outbreak will be influenced by environmental factors. For example, fungal diseases will be more common in damp, humid conditions.

### 3.2. Advanced Pests

The pest system will be expanded to be more dynamic and interactive.

*   **Pest Life Cycles:** Pests will have a life cycle (e.g., egg, larva, adult). Different stages of the life cycle may have different effects on plants and require different treatments.
*   **Pest Behavior:** Pests will have more complex behaviors:
    *   **Movement:** Pests will be able to move from tile to tile.
    *   **Feeding Preferences:** Some pests may prefer certain types of plants.
    *   **Predators and Prey:** A simple food web will be introduced. Some "pests" will actually be beneficial predators that eat other pests (e.g., ladybugs eating aphids).
*   **Infestations:** Infestations will grow and spread over time if left untreated. The severity of the infestation will determine the amount of damage to plants.

### 3.3. Treatments and Preventions

Players will have a wider range of tools to deal with pests and diseases.

*   **Pesticides and Fungicides:** Different chemicals will be effective against different pests and diseases. Overuse of a single chemical can lead to resistance.
*   **Biological Controls:** Players can introduce beneficial insects to control pest populations.
*   **Cultural Practices:** Good gardening practices can help prevent outbreaks:
    *   **Crop Rotation:** Planting the same crop in the same place year after year can lead to a buildup of soilborne diseases.
    *   **Sanitation:** Removing and destroying infected plants can prevent the spread of disease.
    *   **Resistant Varieties:** Players can breed plants with higher resistance to specific pests and diseases.

## 4. Gameplay Implications

*   **Proactive Management:** Players will need to be proactive in monitoring their gardens for signs of pests and diseases.
*   **Integrated Pest Management (IPM):** Players will be encouraged to use a combination of different strategies to manage pests and diseases, rather than relying on a single solution.
*   **New Challenges:** Pests and diseases will be a constant threat that players need to manage.
*   **Deeper Simulation:** The garden will feel more like a living ecosystem.

## 5. Technical Implementation

*   **Disease and Pest Data Structures:** New structs and enums will be created to represent diseases and the new, more complex pests.
*   **Spread and Life Cycle Logic:** New systems will be implemented in the game engine to handle the spread of diseases and the life cycles of pests.
*   **Treatment and Prevention Mechanics:** New game mechanics will be added for the various treatments and preventative measures.
*   **UI Updates:** The UI will be updated to provide players with detailed information about pests and diseases, and to allow them to use the new treatment options.

## 6. Success Metrics

*   Players using a variety of different pest and disease management strategies.
*   The emergence of challenging and interesting pest and disease outbreaks.
*   Positive player feedback on the new depth and realism of the system.
