from orbital.model.order import BaseOrder, LimitOrder
from orbital.model.position import InstrumentPosition
from orbital.model.tick import BaseTick, BarTick
from orbital.portfolio.base import Portfolio
from orbital.strategy.base import BaseStrategy


class DemoStrategy(BaseStrategy):

    def __init__(
        self,
        portfolio: Portfolio
    ):
        super().__init__(portfolio=portfolio)

    def handle_data(self, data: BarTick):
        current_price = data.close
        symbol = data.symbol

        position = self.portfolio.get_position(symbol=symbol)

        if position:
            position.update_market_price(current_price)
            self._adjust_or_exit_position(position, current_price)
        else:
            self._initiate_position(symbol, current_price)

        self.portfolio.update_history(timestamp=data.timestamp)

    def _initiate_position(self, symbol: str, price: float):
        quantity = 100

        order = LimitOrder(
            symbol=symbol,
            quantity=quantity,
            price=price,
            side="long",
        )

        self.place_order(order)

    def _adjust_or_exit_position(self, position: InstrumentPosition, price: float):

        if price > position.entry_price * 1.1:
            order = LimitOrder(
                symbol=position.symbol,
                quantity=position.quantity,
                price=price,
                side="short"
            )
            self.place_order(order)

        elif price < position.entry_price * 0.9:
            order = LimitOrder(
                symbol=position.symbol,
                quantity=position.quantity,
                price=price,
                side="long"
            )
            self.place_order(order)
