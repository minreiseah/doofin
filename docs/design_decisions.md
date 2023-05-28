# Design Decisions/Log

## 28 May

Everything runs synchronously and is not rather coupled, a proper trading system should be *asynchronous* and *distributed*. In order to overcome this, we'll have to read up on how we can further loosely decouple our components, as well as implement asynchronicity. For instance, the `DataClient` must have all subscribers subscribe to it first before starting the stream. While this is fine for backtest/simulated environments, this will not be viable for a live trading environment.

Solutions:
- Might/possibly be related to the message bus which helps to orchestrate events/messages between components.

---

Another similar issue is that the `Strategy` instance should be able to subscribe to the data clients, **but** should not start running immediately since there will be delays subscribing to different data clients (in live environments).

One possible solution could involve storing all the data events in some internal cache (queue) and then processing them sequentially once the `Strategy` instance is ready to begin. One issue with this is that the cache might overflow if there is a large delay between subscribing to the data clients and starting the instance.

---
