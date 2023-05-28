class BaseOrder:
    def __init__(
        self,
        symbol: str,
        quantity: int,
        side: str
    ):
        self.symbol = symbol
        self.quantity = quantity
        self.side = side

class LimitOrder(BaseOrder):
    def __init__(
        self,
        symbol: str,
        quantity: int,
        side: str,
        price: float
    ):
        super().__init__(symbol, quantity, side)
        self.price = price


class MarketOrder(BaseOrder):
    def __init__(
        self,
        symbol: str,
        quantity: int,
        side: str
    ):
        super().__init__(symbol, quantity, side)