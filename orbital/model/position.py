from abc import ABC
from datetime import datetime


class BasePosition(ABC):

    def __init__(
        self,
        symbol: str,
        quantity: int,
        entry_price: float,
        created_time: datetime = None,
        updated_time: datetime = None
    ):
        self.symbol = symbol
        self.quantity = quantity
        self.entry_price = entry_price
        self.created_time = created_time if created_time is not None else datetime.now()
        self.updated_time = updated_time if updated_time is not None else datetime.now()
    
    def update_entry_price(self, new_price) -> 'BasePosition':

        return BasePosition(
            self.symbol,
            self.quantity,
            new_price
        )

class EquityPosition(BasePosition):

    def __init__(
        self,
        symbol: str,
        quantity: int,
        entry_price: float,
        created_time: datetime = None,
        updated_time: datetime = None
    ):
        super().__init__(symbol, quantity, entry_price, created_time, updated_time)

    def update_entry_price(self, new_price) -> 'EquityPosition':

        return EquityPosition(
            self.symbol,
            self.quantity,
            new_price,
            self.created_time,
            datetime.now()
        )

class OrderPosition(BasePosition):

    def __init__(
        self,
        symbol: str,
        quantity: int,
        entry_price: float,
        order_type: str, # TODO model enums
        created_time: datetime = None,
        updated_time: datetime = None
    ):
        super().__init__(symbol, quantity, entry_price, created_time, updated_time)

    def update_entry_price(self, new_price) -> 'OrderPosition':

        return OrderPosition(
            self.symbol,
            self.quantity,
            new_price,
            self.created_time,
            datetime.now()
        )