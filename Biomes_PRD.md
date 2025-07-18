# PRD: Biomes and Planting in Cities on a World Map

## 1. Overview

This PRD proposes the introduction of biomes and a world map with multiple cities, which will fundamentally change where and how players can plant their crops. This will add a new layer of strategy to the game, requiring players to consider the climate and environment of a region when deciding what to grow.

## 2. Current System

Currently, the game takes place on a single, uniform grid. There is no concept of different environments or climates. The soil is procedurally generated but doesn't vary in a structured way.

## 3. Proposed System: A World of Diverse Biomes

### 3.1. The World Map

The game will feature a world map divided into distinct regions. Each region will be characterized by a specific biome.

### 3.2. Biomes

Each biome will have a unique set of environmental characteristics:

*   **Climate:**
    *   **Temperature Range:** Affects which plants can grow. Some plants will be adapted to cold climates, while others will thrive in the heat.
    *   **Rainfall:** Determines the natural moisture level of the soil.
    *   **Sunlight Intensity:** Affects plant growth rates.
*   **Soil Composition:** The default soil type will vary by biome (e.g., sandy soil in deserts, rich loam in temperate forests).
*   **Native Plants and Pests:** Each biome will have its own unique set of native plants, weeds, and pests.

**Example Biomes:**

*   **Temperate Forest:** Moderate temperature and rainfall, loamy soil. Good for a wide variety of crops.
*   **Desert:** High temperature, low rainfall, sandy soil. Only specially adapted plants can grow here.
*   **Tundra:** Low temperature, low rainfall, poor soil. Very few plants can survive.
*   **Tropical Rainforest:** High temperature, high rainfall, rich but easily depleted soil. Home to exotic fruits and plants.

### 3.3. Planting in Cities

Players will be able to establish gardens in different cities around the world map. The success of their garden will depend on how well their chosen plants are adapted to the local biome.

*   **Plant Adaptation:** Each plant species will have a set of preferred environmental conditions. Planting a crop in a biome that doesn't match its preferences will result in reduced yield, slower growth, and increased susceptibility to disease.
*   **Greenhouses and Terraforming:** To grow non-native plants, players will be able to build special structures:
    *   **Greenhouses:** Allow players to control the temperature and humidity for a small plot of land, enabling them to grow any plant anywhere, but at a high cost.
    *   **Soil Amendment:** Players can amend the soil to change its properties (e.g., adding clay to sandy soil to improve water retention), but this will be an ongoing and expensive process.

## 4. Gameplay Implications

*   **Location, Location, Location:** The choice of where to plant will become a key strategic decision.
*   **Specialization:** Players may choose to specialize in growing crops that are well-suited to a particular biome.
*   **Trade and Exploration:** Players will be motivated to explore the world map to find new biomes with unique plants and resources.
*   **New Challenges:** Players will have to contend with the challenges of growing crops in hostile environments.

## 5. Technical Implementation

*   **Biome Data Structure:** A new data structure will be created to define the properties of each biome.
*   **World Map Generation:** The world map will be procedurally generated, with different biomes distributed across it.
*   **Plant Adaptation Logic:** The `Plant` struct will be updated to include data on its environmental preferences. The game engine will then use this data to calculate the effects of the environment on plant growth.
*   **Greenhouse and Terraforming Mechanics:** New game mechanics will be implemented for greenhouses and soil amendment.
*   **UI Updates:** The UI will be updated to show information about biomes and plant preferences.

## 6. Success Metrics

*   Players establishing gardens in a variety of different biomes.
*   The development of regional agricultural specialties.
*   Positive player feedback on the new strategic depth added by the biome system.
