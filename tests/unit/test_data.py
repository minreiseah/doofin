import pytest

import pandas as pd

from orbital.data.client import MockBarTickDataClient
from orbital.model.tick import BarTick

@pytest.fixture
def client(mocker):
    # set up mock for pandas' read_parquet method
    data = {'datetime': ['2023-01-01'],
            'open': [100.0],
            'high': [110.0],
            'low': [90.0],
            'close': [105.0],
            'volume': [1000]}
    df = pd.DataFrame(data)
    mocker.patch('pandas.read_parquet', return_value = df)
    client = MockBarTickDataClient('dummy_path', 'parquet', 10, 'SPY')
    return client

def test_data_client_subscribe(client):
    subscriber = object()
    client.subscribe(subscriber)
    assert subscriber in client.subscribers

def test_data_client_unsubscribe(client):
    subscriber = object()
    client.subscribe(subscriber)
    client.unsubscribe(subscriber)
    assert subscriber not in client.subscribers

def test_data_client_fetch_data(client, mocker):
    bar_tick = client._fetch_data()

    assert isinstance(bar_tick, BarTick)
    assert bar_tick.timestamp == '2023-01-01'
    assert bar_tick.symbol == 'SPY'
    assert bar_tick.open == 100.0
    assert bar_tick.high == 110.0
    assert bar_tick.low == 90.0
    assert bar_tick.close == 105.0
    assert bar_tick.volume == 1000
