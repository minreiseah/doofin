# [PRD] Strategy Component

## Glossary

## Introduction

### What are we building

A component that provides the following functionalities:
- Implement rule-based generation of actions (e.g. placing/cancelling diff types of orders)
- Allows dynamic configuration for parameters of execution
- Seamless integration with environment
- State of execution
- Integrated with all other components (data, portfolio, execution)

### Why are we building it

This main component serves as the "engine" to integrate with the data and portfolio components to process data to generate orders for the execution component.

### How is success defined

- Lightweight component that does not incur overhead on top of the rules according to the strategy
- Supports multiple paradigms of strategies
    - time-based
    - condition-based
    - etc
- Updating strategy parameters are easy and instantaneous
- Starting, stopping, and restarting strategies are easy and instanteneous


## Requirements

> Preamble:

### Flows

#### Building strategies

- [P0] Integrate with a data stream
- [P0] Integrate with database through a data SDK
- [P0] Integrate with an execution client

- [P1] Integrate with multiple data streams
- [P1] Integrate with multiple execution clients
- [P2] Multi-strategy integration

#### Executing strategies

- [P1] Support trigger based strategies
- [P1] Support timed, scheduled (probing) based strategies


#### Evaluating strategies

- [P2] Support tracking strategy performance
- [P2] Support tracking stragegy statistics


#### UI 

- [P1] Integrated into portfolio component to show running strategies
