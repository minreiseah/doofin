from abc import ABC, abstractmethod

from orbital.model.tick import BaseTick
from orbital.model.position import InstrumentPosition
from orbital.model.order import BaseOrder, LimitOrder, MarketOrder
from orbital.portfolio.base import Portfolio


class BaseStrategy(ABC):

    def __init__(
        self,
        portfolio: Portfolio
    ):
        self.portfolio = portfolio

    @abstractmethod
    def handle_data(self, tick: BaseTick):
        raise NotImplementedError("Data handler not implemented.")

    def place_order(self, order: BaseOrder):
        """Publishes order for the execution engine to subscribe to.
        The execution engine SHOULD update the portfolio; not done here.

        Args:
            order (BaseOrder): 
        """

        if isinstance(order, LimitOrder):
            # TODO mock execution client for now
            if order.side == "long":
                # ensure we don't leverage
                cost = order.quantity * order.price
                if (cost > self.portfolio.cash):
                    # TODO the order should be logged with a FAILED
                    # to give information to user about trades
                    return

                position = InstrumentPosition(
                    symbol=order.symbol,
                    quantity=order.quantity,
                    entry_price=order.price
                )
            elif order.side == "short":
                position = InstrumentPosition(
                    symbol=order.symbol,
                    quantity=-order.quantity,
                    entry_price=order.price
                )
            else:
                raise NotImplementedError(f'Order side {order.side} not implemented.')
            
            self.portfolio.update_position(position)

        if isinstance(order, MarketOrder):
            raise NotImplementedError("Market orders not yet implemented.")

