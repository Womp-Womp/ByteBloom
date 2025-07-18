# PRD: Full-Fledged Economy and Market

## 1. Overview

This PRD describes a more complex and realistic economy for ByteBloom Gardens. The current economy is very basic, with static prices and simple supply/demand mechanics. This proposal will introduce a dynamic, multi-faceted market with regional variations, a stock market, and more sophisticated economic drivers.

## 2. Current System

The current economy consists of:

*   A simple market with a `HashMap` of item names to prices.
*   A basic supply and demand model that slightly adjusts prices based on player sales.
*   Random price fluctuations.

## 3. Proposed System: A Global and Local Economy

### 3.1. World Map and Regions

The game world will be expanded to a world map with multiple cities or regions. Each region will have its own local market with unique supply and demand characteristics.

**Example:**

*   A coastal city might have a high demand for exotic fruits but a low demand for staple crops like wheat.
*   A rural, agricultural region might have a high supply of wheat, making it cheap locally but expensive in the coastal city.

### 3.2. Local Markets

Each city/region will have its own `Market` instance. Prices for the same item will vary between these markets.

*   **Local Supply and Demand:** Player actions (buying/selling) will only affect the local market in that region.
*   **Arbitrage Opportunities:** Players can buy goods in a region where they are cheap and transport them to another region to sell for a profit.

### 3.3. Transportation and Logistics

To move goods between regions, players will need to manage logistics:

*   **Transportation Costs:** Moving goods will have a cost based on distance and volume.
*   **Transportation Time:** Goods will take time to arrive at their destination.
*   **Infrastructure:** Players can invest in upgrading transportation infrastructure (e.g., faster trucks, larger warehouses) to reduce costs and time.

### 3.4. Dynamic Events and News

The "Terminal Ticker" will play a much more significant role in the economy. It will report on events that have a real impact on supply and demand.

**Examples:**

*   **Drought in a region:** The price of water-intensive crops in that region will skyrocket.
*   **New trade agreement:** Transportation costs between two regions are reduced.
*   **Pest outbreak:** The supply of a specific crop is reduced, increasing its price.

### 3.5. The ByteBloom AgroCorp (BBA) Stock Market

The BBA stock will be more than just a single value. It will be a fully-fledged stock market with multiple companies.

*   **Regional Companies:** Each region might have its own agricultural corporation whose stock players can buy and sell.
*   **Company Performance:** The value of a company's stock will be tied to the economic health of its region and its primary products.
*   **Player Impact:** Large-scale player actions can influence the stock market. For example, a player who corners the market on a particular crop could see the stock of a company that relies on that crop fall.
*   **Insider Trading (a feature, not a bug):** Players might get news tips before they become public, allowing them to make strategic stock trades.

## 4. Gameplay Implications

*   **New Gameplay Loop:** The game will expand from just gardening to a full-fledged economic simulation, including logistics and trading.
*   **Greater Depth:** The economy will be a complex system that players need to learn and master.
*   **More Strategic Choices:** Players will have to decide where to establish their farms, what to grow, and where to sell their produce.
*   **New UI/Commands:** The game will need new commands for:
    *   Viewing different regional markets.
    *   Managing transportation.
    *   Interacting with the stock market.

## 5. Technical Implementation

*   **World Map Representation:** A data structure will be needed to represent the world map and the connections between regions.
*   **Regional Market Logic:** The `Market` struct will be instantiated for each region, and logic will be added to simulate regional price differences.
*   **Transportation System:** A new module will be created to handle the logic of moving goods between regions.
*   **Stock Market Simulation:** A new module will be created to simulate the stock market.
*   **Event System Integration:** The event system will be tightly integrated with the economy to create dynamic price shocks.

## 6. Success Metrics

*   Player engagement with the new economic features.
*   The emergence of interesting economic strategies (e.g., arbitrage, stock market manipulation).
*   Positive player feedback on the depth and realism of the economy.
