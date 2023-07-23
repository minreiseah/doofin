import pytest
from orbital.model.order import LimitOrder
from orbital.model.position import InstrumentPosition

from orbital.strategy.demo import DemoStrategy
from orbital.data.client import MockBarTickDataClient
from orbital.portfolio.base import Portfolio

def test_place_order():
    portfolio = Portfolio(cash=1_000_000, positions=[])

    strategy = DemoStrategy(portfolio)

    strategy.place_order(LimitOrder(
        symbol="aapl",
        quantity=100,
        price=200,
        side="long"
    ))

    aapl_position = portfolio.get_position("aapl")
    assert aapl_position is not None

    assert aapl_position.symbol == 'aapl'
    assert aapl_position.quantity == 100
    assert aapl_position.entry_price == 200

    assert portfolio.cash == 980_000

def test_add_new_position():
    portfolio = Portfolio(cash=1_000_000, positions=[])

    strategy = DemoStrategy(portfolio)

    data_client = MockBarTickDataClient(
        data_file_path="./orbital/data/mocked/AAPL.parquet",
        file_type="parquet",
        max_ticks=100,
        symbol='aapl',
    )

    data_client.subscribe(strategy)

    data_client.start_streaming()

    # Assuming aapl doesn't exist in portfolio initially
    aapl_position = portfolio.get_position('aapl')
    
    assert aapl_position is not None
    assert isinstance(aapl_position, InstrumentPosition)

# Test if the position is removed correctly when quantity is zero.
def test_remove_position():
    portfolio = Portfolio(cash=1_000_000, positions=[])

    strategy = DemoStrategy(portfolio)

    data_client = MockBarTickDataClient(
        data_file_path="./orbital/data/mocked/AAPL.parquet",
        file_type="parquet",
        max_ticks=100,
        symbol='aapl',
    )

    data_client.subscribe(strategy)

    data_client.start_streaming()

    # Assuming AAPL does exist in portfolio
    aapl_position = portfolio.get_position('aapl')

    # Manually set position quantity to zero.
    reverse_aapl_position = InstrumentPosition(
        symbol = aapl_position.symbol,
        quantity = -aapl_position.quantity,
        entry_price = aapl_position.entry_price
    )

    # The position should be removed from portfolio.
    portfolio.update_position(reverse_aapl_position)

    # Check if the position was removed.
    assert portfolio.get_position('aapl') is None

def test_full_backtest():
    
    portfolio = Portfolio(cash=1_000_000, positions=[])

    strategy = DemoStrategy(portfolio)

    data_client = MockBarTickDataClient(
        data_file_path="./orbital/data/mocked/AAPL.parquet",
        file_type="parquet",
        max_ticks=100_000,
        symbol='aapl',
    )

    data_client.subscribe(strategy)

    data_client.start_streaming()

    aapl_position = portfolio.get_position("aapl")

    assert aapl_position is not None
    assert isinstance(aapl_position, InstrumentPosition)

    assert portfolio.cash != 1_000_000
