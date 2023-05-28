from abc import ABC
from datetime import datetime

class BaseTick(ABC):

    def __init__(self, timestamp: datetime):
        self.timestamp = timestamp

class PriceTick(BaseTick):

    def __init__(
        self,
        timestamp: datetime,
        symbol: str,
        price: float,
        volume: float
    ):
        super().__init__(timestamp)
        self.symbol = symbol
        self.price = price
        self.volume = volume

