import pytest
from typing import List
from pandas import Timestamp

from orbital.strategy.base import BaseStrategy
from orbital.strategy.demo import DemoStrategy
from orbital.portfolio.base import Portfolio

from orbital.model.position import InstrumentPosition
from orbital.model.order import BaseOrder, LimitOrder, MarketOrder
from orbital.model.tick import BarTick

STARTING_CASH = 1_000_000

@pytest.fixture
def portfolio():
    return Portfolio(STARTING_CASH)

@pytest.fixture
def my_strategy(portfolio):
    return DemoStrategy(portfolio)

@pytest.fixture
def limit_order():
    order = LimitOrder(
        symbol="TEST_LIMIT",
        quantity=100,
        side="long",
        price=50
    )
    return order

@pytest.fixture
def market_order():
    order = MarketOrder(
        symbol="TEST_MARKET",
        quantity=100,
        side="short",
    )
    return order

@pytest.fixture
def bar_ticks() -> List[BarTick]:
    tick_0 = BarTick(
        timestamp = Timestamp('2007-04-27 10:43:00'),
        symbol = 'TEST',
        open = 98.41000366210938,
        high = 98.45999908447266,
        low = 98.30000305175781,
        close = 98.30000305175781,
        volume = 46196
    )

    tick_1 = BarTick(
        timestamp = Timestamp('2007-04-27 10:44:00'),
        symbol = 'TEST',
        open = 99.41000366210938,
        high = 99.45999908447266,
        low = 97.30000305175781,
        close = 99.30000305175781,
        volume = 46112
    )

    return [tick_0, tick_1]

# TESTING

def test_strategy_init(
    portfolio: Portfolio,
    my_strategy: BaseStrategy
):
    assert my_strategy.portfolio == portfolio

def test_strategy_handle_data(
    bar_ticks: List[BarTick],
    my_strategy: BaseStrategy,
    mocker,
):
    mocked_update_position = mocker.patch.object(Portfolio, "update_position")
    for tick in bar_ticks:
        my_strategy.handle_data(tick)
    assert mocked_update_position.call_count == 2
    print(my_strategy.portfolio.statistics())

def test_place_limit_order_long_side(my_strategy, limit_order, mocker):
    my_strategy.portfolio.cash = STARTING_CASH
    mocked_update_position = mocker.patch.object(Portfolio, "update_position")
    my_strategy.place_order(limit_order)
    assert mocked_update_position.call_count == 1
    called_position = mocked_update_position.call_args[0][0]
    assert called_position.symbol == limit_order.symbol
    assert called_position.quantity == limit_order.quantity
    assert called_position.entry_price == limit_order.price

def test_base_strategy_handle_data(my_strategy, market_order):
    with pytest.raises(NotImplementedError, match="Market orders not yet implemented."):
        my_strategy.place_order(market_order)

def test_place_order_insufficient_funds(my_strategy, limit_order, mocker):
    my_strategy.portfolio.cash = 100
    mocked_update_position = mocker.patch.object(Portfolio, "update_position")
    my_strategy.place_order(limit_order)
    mocked_update_position.assert_not_called()

def test_place_order_invalid_side(my_strategy, limit_order):
    limit_order.side = "invalid_side"
    with pytest.raises(NotImplementedError, match='Order side invalid_side not implemented.'):
        my_strategy.place_order(limit_order)

