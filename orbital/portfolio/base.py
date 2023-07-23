from datetime import datetime
from functools import wraps
from typing import List, Dict, Union
import pandas as pd

from orbital.model.position import BasePosition, InstrumentPosition

class PortfolioState:

    def __init__(
        self,
        value: float,
        created_time: datetime = None,
    ):
        self.value = value
        self.created_time = datetime.now() if created_time is None else created_time


class Portfolio:
    """
    Very basic implementation of a portfolio. This has a one-to-many
    relationship with the `Strategy` class.

    """

    def __init__(
        self,
        cash: float,
        realised_profit: float = 0.0,
        positions: List[BasePosition] = [], #TODO This really should be a hash data structure into a list
        history: List[PortfolioState] = [], #TODO not sure the best data structure for this
    ):
        self.cash = cash
        self.realised_profit = realised_profit
        self.positions = positions
        self.history = history

    def on_update(func):
        """If a method uses the `on_update` decorator, the portfolio history will 
        be updated with the current state of the portfolio everytime the method is called.

        Args:
            func (function): function to decorate
        """
        @wraps(func)
        def wrapper(self, *args, **kwargs):
            func(self, *args, **kwargs)
            self.history.append(PortfolioState(
                self.get_portfolio_value(),
                datetime.now()
            ))
        return wrapper
    
    @on_update
    def update_position(self, position: InstrumentPosition):
        """Updates an existing position in the portfolio or adds a new position.

        If a position with the same symbol already exists in the portfolio, the function updates
        the quantity and entry price of the existing position. Otherwise, it adds the new position
        to the portfolio.

        Args:
            position (InstrumentPosition): The position to be updated or added.

        Returns:
            None
            ? should this return something more

        Examples:
            # Create a new position
            new_position = InstrumentPosition(
                symbol="AAPL",
                quantity=100,
                entry_price=150.0,
                created_time=datetime.now()
            )

            # Update an existing position
            existing_position = InstrumentPosition(
                symbol="AAPL",
                quantity=50,
                entry_price=160.0,
                created_time=datetime.now() - timedelta(days=1)
            )

            portfolio.update_position(new_position)
            portfolio.update_position(existing_position)
        """

        prev_position = None
        index = -1

        for i, pos in enumerate(self.positions):
             if position.symbol == pos.symbol:
                 prev_position = pos
                 index = i
                 break

        if prev_position is None:
            if position.quantity < 0:
                raise NotImplementedError("Shorting not yet implemented.")
            self.cash -= position.quantity * position.entry_price
            self.positions.append(position)
            return

        if position.quantity < 0: # sale
            realised_profit = (-position.quantity) * (position.entry_price - prev_position.entry_price)
            self.realised_profit += realised_profit
            self.cash += (-position.quantity) * position.entry_price
        else: # purchase
            self.cash -= position.quantity * position.entry_price

        if self.positions[index].quantity == -position.quantity:
            print("REMOVED POSITION")
            self.positions.remove(self.positions[index])
            return

        self.positions[index] = InstrumentPosition(
            symbol=position.symbol,
            quantity=prev_position.quantity + position.quantity,
            entry_price=((prev_position.quantity * prev_position.entry_price +
                         position.quantity * position.entry_price) /
                         (prev_position.quantity + position.quantity)),
            created_time=prev_position.created_time,
            updated_time=position.created_time
        )
        

    def get_position(self, symbol: str) -> Union[BasePosition, None]:
        """Get position for a specific symbol.

        Args:
            symbol (str): The symbol of the position to retrieve.

        Returns:
            BasePosition: The corresponding position object
        """

        for position in self.positions:
            if position.symbol == symbol:
                return position

        return None

    def get_portfolio_value(self) -> float:
        """Calculates the total value of the portfolio in the current moment.
        This includes realised and unrealised profits.

        Returns:
            Dict: The total value of the portfolio.
        """
        position_values = sum(position.quantity * position.market_price for position in self.positions)

        return self.cash + position_values
    
    def statistics(self) -> Dict:
        position_values = sum(
            position.quantity * position.market_price for position in self.positions)
        
        unrealised_profit = sum(
            position.quantity * (position.market_price - position.entry_price) for position in self.positions)

        return {
            'total_value': position_values + self.cash,
            'position_values': position_values,
            'free_cash': self.cash,
            'realised_profit': self.realised_profit,
            'unrealised_profit': unrealised_profit,
        }    

    def to_dataframe(self):
        """Returns a DataFrame representation of the portfolio.
            
        Returns:
            pd.DataFrame: DataFrame representation of the portfolio.
        """

        data = {
            'Symbol': [],
            'Quantity': [],
            'Entry Price': [],
            'Created Time': [],
            'Updated Time': []
        }
        
        for position in self.positions:
            data['Symbol'].append(position.symbol)
            data['Quantity'].append(position.quantity)
            data['Entry Price'].append(position.entry_price)
            data['Created Time'].append(position.created_time)
            data['Updated Time'].append(position.updated_time)
        
        return pd.DataFrame(data)
    
    def __str__(self):
       """Returns a string representation of the portfolio.

       Returns:
           str: String representation of the portfolio.
       """
       position_info = "\n".join(str(position) for position in self.positions)
       return f"Portfolio:\nCash: {self.cash}\nPositions:\n{position_info}"
    

