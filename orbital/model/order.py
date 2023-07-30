class BaseOrder:
    def __init__(
        self,
        symbol: str,
        quantity: int,
        side: str 
    ):
        """_summary_

        Args:
            symbol (str): _description_
            quantity (int): _description_
            side (str): long or short.
        """
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
