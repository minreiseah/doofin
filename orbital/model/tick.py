"""
`tick` wrapper but might be slower than using a dictionary. KIV for now.
"""

from abc import ABC
from datetime import datetime


class BaseTick(ABC):

    def __init__(self, timestamp: datetime):
        self.timestamp = timestamp


class BarTick(BaseTick):
    """Represents a minute-based *bar* tick that contains OHLC (Open-High-Low-Close) price information for a financial instrument.

    Args:
        timestamp (datetime): The timestamp of the tick.
        symbol (str): The symbol or identifier of the financial instrument.
        open (float): The opening price of the minute interval.
        high (float): The highest price reached during the minute interval.
        low (float): The lowest price reached during the minute interval.
        close (float): The closing price of the minute interval.
        volume (float): The total volume traded during the minute interval.

    Example:
        # Create a minute tick for Stock XYZ
        tick = BarTick(
            timestamp=datetime.now(),
            symbol="XYZ",
            open=100.50,
            high=101.20,
            low=100.10,
            close=100.80,
            volume=2000
        )
    """

    def __init__(
        self,
        timestamp: datetime,
        symbol: str,
        open: float,
        high: float,
        low: float,
        close: float,
        volume: float
    ):
        super().__init__(timestamp)
        self.symbol = symbol
        self.open = open
        self.high = high
        self.low = low
        self.close = close
        self.volume = volume


class TradeTick(BaseTick):
    """Represents a trade tick that contains information about a trade of a financial instrument.

    Args:
        timestamp (datetime): The timestamp of the tick.
        symbol (str): The symbol or identifier of the financial instrument.
        price (float): The price of the trade.
        volume (float): The volume or size of the trade.

    Example:
        # Create a trade tick for Stock XYZ
        tick = TradeTick(
            timestamp=datetime.now(),
            symbol="XYZ",
            price=100.50,
            volume=1000
        )
    """

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


class QuoteTick(BaseTick):
    """Represents a quote tick that contains bid and ask price information for a financial instrument.

    Args:
        timestamp (datetime): The timestamp of the tick.
        symbol (str): The symbol or identifier of the financial instrument.
        bid_price (float): The current highest bid price.
        bid_size (float): The current size of the bid.
        ask_price (float): The current lowest ask price.
        ask_size (float): The current size of the ask.

    Example:
        # Create a quote tick for Stock XYZ
        tick = QuoteTick(
            timestamp=datetime.now(),
            symbol="XYZ",
            bid_price=100.40,
            bid_size=500,
            ask_price=100.60,
            ask_size=800
        )
    """

    def __init__(
        self,
        timestamp: datetime,
        symbol: str,
        bid_price: float,
        bid_size: float,
        ask_price: float,
        ask_size: float
    ):
        super().__init__(timestamp)
        self.symbol = symbol
        self.bid_price = bid_price
        self.bid_size = bid_size
        self.ask_price = ask_price
        self.ask_size = ask_size


class MarketDataTick(BaseTick):
    """Represents a market data tick that contains information about a financial instrument.

    Args:
        timestamp (datetime): The timestamp of the tick.
        symbol (str): The symbol or identifier of the financial instrument.
        last_trade_price (float): The price of the last trade.
        last_trade_volume (float): The volume of the last trade.
        bid_price (float): The current highest bid price.
        bid_size (float): The current size of the bid.
        ask_price (float): The current lowest ask price.
        ask_size (float): The current size of the ask.
    
    Example:
        # Create a market data tick for Stock XYZ
        tick = MarketDataTick(
            timestamp=datetime.now(),
            symbol="XYZ",
            last_trade_price=100.50,
            last_trade_volume=1000,
            bid_price=100.40,
            bid_size=500,
            ask_price=100.60,
            ask_size=800
        )
    """

    def __init__(
        self,
        timestamp: datetime,
        symbol: str,
        last_trade_price: float,
        last_trade_volume: float,
        bid_price: float,
        bid_size: float,
        ask_price: float,
        ask_size: float
    ):
        super().__init__(timestamp)
        self.symbol = symbol
        self.last_trade_price = last_trade_price
        self.last_trade_volume = last_trade_volume
        self.bid_price = bid_price
        self.bid_size = bid_size
        self.ask_price = ask_price
        self.ask_size = ask_size
