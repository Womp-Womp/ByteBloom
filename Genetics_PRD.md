# PRD: Advanced Genetics System

## 1. Overview

This PRD outlines an advanced genetics system for ByteBloom Gardens, moving beyond the current simple genetic traits to a more robust and interactive system based on dominant and recessive genes. This will create deeper, more engaging gameplay around plant breeding and cultivation.

## 2. Current System

The current system defines plant genetics as a set of fixed values in the `PlantGenetics` struct. These values are procedurally generated from a seed for each plant species, but they do not allow for inheritance or mutation in a granular way.

## 3. Proposed System: Gene-Based Genetics

### 3.1. Gene Pairs

Instead of a single value for each trait (e.g., `growth_time`), each trait will be determined by a pair of genes. Each gene will have a specific value and a dominance attribute (dominant or recessive).

**Example:**

A plant's `growth_time` will be determined by a gene pair, like `(G, g)`, where:

*   `G` is a dominant gene for fast growth (e.g., 5 ticks).
*   `g` is a recessive gene for slow growth (e.g., 10 ticks).

The expressed trait will be determined by the combination of these genes:

*   `GG`: Fast growth (5 ticks)
*   `Gg`: Fast growth (5 ticks)
*   `gg`: Slow growth (10 ticks)

### 3.2. Traits as Gene Collections

The `PlantGenetics` struct will be refactored to hold collections of these gene pairs for each trait.

```rust
// Proposed structure
pub struct PlantGenetics {
    pub growth_time: (Gene, Gene),
    pub yield_range: (Gene, Gene),
    pub ideal_moisture_range: (Gene, Gene),
    // ... and so on for other traits
}

pub struct Gene {
    pub value: f32, // Or appropriate type for the trait
    pub dominance: Dominance,
}

pub enum Dominance {
    Dominant,
    Recessive,
}
```

### 3.3. Inheritance

When a player harvests seeds from a plant, the new seeds will inherit a mix of the parent's genes. For each trait, the new seed will receive one gene from the parent's pair, chosen at random.

**Example:**

A parent plant with `Gg` for `growth_time` can produce seeds with either a `G` or a `g` gene.

To create a new plant, two parent plants are needed. The new plant will inherit one gene from each parent for each trait.

**Example:**

*   Parent 1: `Gg`
*   Parent 2: `gg`

Possible offspring genotypes: `Gg`, `gg`.

### 3.4. Mutations

There will be a small chance for a gene to mutate when a new seed is created. A mutation can:

*   Change the value of the gene (e.g., a `growth_time` gene mutates to have a slightly faster or slower value).
*   Change the dominance of the gene (e.g., a dominant gene becomes recessive).

The `genetic_stability` trait will influence the probability of mutations.

### 3.5. Cross-Breeding

Players will be able to cross-breed two different plants to create a new hybrid. The hybrid will inherit a mix of genes from both parents. This will be the primary way for players to create new and improved plant varieties.

## 4. Gameplay Implications

*   **Deeper Strategy:** Players will need to think strategically about which plants to breed to achieve desired traits.
*   **Emergent Variety:** The number of possible gene combinations is vast, leading to a huge variety of unique plants.
*   **New Tools:** New tools and UI features will be needed for:
    *   Viewing a plant's genetic makeup.
    *   A dedicated cross-breeding interface.
    *   A way to track the lineage of plants.
*   **End-Game Content:** Advanced players can focus on creating "perfect" plants with optimal genes for every trait.

## 5. Technical Implementation

*   **Refactor `PlantGenetics`:** The existing `PlantGenetics` struct will be replaced with the new gene-based system.
*   **Implement Inheritance Logic:** New functions will be needed to handle the logic of gene inheritance and cross-breeding.
*   **Implement Mutation Logic:** The mutation system will be implemented, tied to the `genetic_stability` trait.
*   **Update UI:** The command-line interface will be updated to display genetic information and provide cross-breeding commands.

## 6. Success Metrics

*   Player engagement with the breeding system.
*   The variety of player-created plant strains.
*   Positive feedback from the community on the depth of the new system.
