# [PRD] Portfolio Component

## Glossary

## Introduction

### What are we building

A component that provides the following functionalities:
- query:
    - current positions
    - historical positions
    - historical value of portfolio over time
- support multiple distinct portfolios at the same time

### Why are we building it

Many (strategy) decisions are made around the state of the portfolio. Hence, ore specific interfaces are useful to effectively capture the state of the portfolio and assess metrics.

### How is success defined

- accurate
- statistics near-instantaneously calculated

## Requirements

> Preamble:

### Flows

#### Querying

- [P0] Open positions
- [P0] Open orders
- [P1] Historical positions
- [P1] Historical orders
- [P1] Aggregated statistics

#### UI 
