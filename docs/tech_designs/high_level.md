# Tech Design: Trading Platform

## Glossary

> Terms that need to be established

## Goals

> (link to PRD doc for trading platform) how do i do this in github pages uh

- Strategy component as entrypoint and interface with below components
- Execution component to execute various types of orders
- Data component that supports integrating data streams
- Portfolio component that provides current portfolio data

### System Goals

- Low latency
- High availability
- Extensibility 

## Architecture

### Development dependencies


### State design


### Event-driven system

For the event-driven event, components consume events to execute its functionality and produce events for other components to consume.
- Key constructs of the event-driven system: Event buses to store events
  - Pub/Sub model (Kafka)
  - Event semantics, at least once, multiple consume, bus clock, TTL(Dwell), etc
  - IDL of events

### Error

### Metrics


## References
