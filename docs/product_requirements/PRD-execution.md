# [PRD] Execution Component

## Glossary

## Introduction

### What are we building

A component that provides the following functionalities:
- executing orders
- cancelling orders
- query:
    - order status
    - open orders
    - historical orders

### Why are we building it

To execute orders.



### How is success defined

- low latency
- high throughput
- absolute reliability


## Requirements

> Preamble:

### Flows

#### Executing orders

- [P0] Execute orders
- [P0] Cancel orders

#### Querying orders

- [P1] Order status
- [P1] Open orders
- [P1] Historical orders

#### Integrating execution client

- [P0] Support integrating execution from one source client

#### UI 

