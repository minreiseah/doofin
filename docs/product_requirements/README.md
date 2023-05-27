# [PRD] High Level Overview

## Introduction

### Features

- traders can execute trading strategies
- data platform to allow for data ingestion and querying


### Rationale

> Most undergraduate research on asset trading strategies are rarely deployed to live production because most open source trading system software have too much vendor lock-in. For instance, quantconnect requires users to use their proprietary platform and language to deploy trades. Furthermore, users may be uncomfortable with sharing their strategies since the backend logic runs on foreign servers. Lastly, most open source systems are not extensible enough. Therefore, our project aims to build an extensible, easy-to-use trading system to deploy these strategies.


### How is success defined?

The platform *must be* performant (low latency) and reliable (high availability).

The platform *should be* flexible (suppport as many flows as possible).

The platform *does not need to be* scalable (handle high traffic).


## Requirements

> Preamble: 

### Flows

#### 1. Execution Flow

user, strategy, data component, execution component, portfolio component

**Initialising strategy component**
- [P0] Integration with data component
- [P0] Integration with execution component
- [P1] Integration with portfolio component

**Initialising data component**
- [P0] Integration with data clients (external)
- [P1] Integration with data clients (internal)

**Fetch data**
- [P0] Passed as streams (event-driven design)
- [P0] Real-time price data
- [P1] Historical price data

**Initialising execution component**
- [P0] Integration with execution clients (external)

**Execute order**
- [P0] Send an order to an execution client

**Initialising portfolio**
- [P1] Integrate with other strategies

**Fetch portfolio**
- [P0] Open positions
- [P0] Open orders
- [P1] Closed orders
- [P1] Historical positions
- [P1] Historical orders
- [P2] Aggregated statistics


#### 2. Backtest Flow
