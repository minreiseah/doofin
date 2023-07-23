import pytest
from datetime import datetime

from orbital.model.position import InstrumentPosition
from orbital.portfolio.base import Portfolio

def test_portfolio_initialization():
    portfolio = Portfolio(cash=1000, positions=[])
    assert portfolio.cash == 1000
    assert portfolio.realised_profit == 0.0
    assert len(portfolio.positions) == 0

def test_update_position_new():
    portfolio = Portfolio(cash=1000, positions=[])
    new_position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(new_position)
    assert portfolio.get_position("AAPL").quantity == 10
    assert portfolio.cash == 0

def test_update_position_existing():
    portfolio = Portfolio(cash=2000, positions=[])
    position_1 = InstrumentPosition("AAPL", 10, 100, datetime.now())
    position_2 = InstrumentPosition("AAPL", 5, 120, datetime.now())
    portfolio.update_position(position_1)
    portfolio.update_position(position_2)
    assert portfolio.get_position("AAPL").quantity == 15
    assert portfolio.get_position("AAPL").entry_price == pytest.approx(106.6666, 0.0001)
    assert portfolio.cash == 400

def test_update_position_shorting():
    portfolio = Portfolio(cash=1000, positions=[])
    short_position = InstrumentPosition("AAPL", -10, 100, datetime.now())
    with pytest.raises(NotImplementedError):
        portfolio.update_position(short_position)

def test_get_position():
    portfolio = Portfolio(cash=1000, positions=[])
    position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(position)
    assert portfolio.get_position("AAPL").symbol == "AAPL"
    assert portfolio.get_position("MSFT") is None

def test_get_portfolio_value():
    portfolio = Portfolio(cash=1000, positions=[])
    position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(position)
    assert portfolio.get_portfolio_value() == 1000

def test_statistics_initialisation():
    portfolio = Portfolio(cash=1000, positions=[])
    # position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    # portfolio.update_position(position)
    stats = portfolio.statistics()
    assert stats['total_value'] == 1000
    assert stats['position_values'] == 0
    assert stats['free_cash'] == 1000
    assert stats['realised_profit'] == 0
    assert stats['unrealised_profit'] == 0

def test_statistics_purchase():
    portfolio = Portfolio(cash=1000, positions=[])
    position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(position)
    stats = portfolio.statistics()
    assert stats['total_value'] == 1000
    assert stats['position_values'] == 1000
    assert stats['free_cash'] == 0
    assert stats['realised_profit'] == 0
    assert stats['unrealised_profit'] == 0

def test_statistics_sale():
    portfolio = Portfolio(cash=1000, positions=[])
    position1 = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(position1)
    position2 = InstrumentPosition("AAPL", -10, 50, datetime.now())
    portfolio.update_position(position2)
    
    stats = portfolio.statistics()
    assert stats['total_value'] == 500
    assert stats['position_values'] == 0
    assert stats['free_cash'] == 500
    assert stats['realised_profit'] == -500
    assert stats['unrealised_profit'] == 0
    assert portfolio.get_position("AAPL") is None

def test_to_dataframe():
    portfolio = Portfolio(cash=1000, positions=[])
    position = InstrumentPosition("AAPL", 10, 100, datetime.now())
    portfolio.update_position(position)
    df = portfolio.to_dataframe()
    assert df.shape == (1, 5)
    assert df['Symbol'][0] == "AAPL"
