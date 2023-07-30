import time
from abc import ABC, abstractmethod
from typing import Iterable, Dict
import pandas as pd
import asyncio

from orbital.model.tick import BarTick


class BaseDataClient(ABC):
    def __init__(self):
        self.subscribers = []
        self.data = None

    def subscribe(self, subscriber):
        # TODO subsriber should be of type DataSubscriber interface
        # e.g. implemented by Strategy classes
        self.subscribers.append(subscriber)

    def unsubscribe(self, subscriber):
        self.subscribers.remove(subscriber)

    def start_streaming(self, freq: float = 0.001):
        """
        This is just a mock of the actual pub/sub architecture.
        The issue with this is that new data overwrites past data
        which might not have been handled yet.

        #TODO proper pub/sub and message queueing system.

        Args:
            freq (float, optional): "tick" frequency. Defaults to 0.001.
        """

        try:
            while True:
                self.data = self._fetch_data()

                for subscriber in self.subscribers:
                    subscriber.handle_data(self.data)

                # time.sleep(freq)
                # await asyncio.sleep(freq)
        except StopIteration:
            print("Streaming completed")
    
    def get_snapshot(self):
        return self.data

    @abstractmethod
    def _fetch_data(self) -> any:
        # TODO should implement some type checking, this works for now..
        raise NotImplementedError("Fetching data is not implemented.")


class MockBarTickDataClient(BaseDataClient):
    """There will be some sane/standard data clients for strategy components
    to hook into. e.g. MinuteDataClient(ticker="spy")

    Args:
        BaseDataClient (_type_): _description_
    """

    def __init__(
        self,
        data_file_path: str,
        file_type: str,
        max_ticks: int = None,
        symbol: str = None,
    ):
        super().__init__()
        if (file_type == 'parquet'):
            df = pd.read_parquet(data_file_path)
            print(f"Data Client has {len(df)} ticks")
        else:
            raise NotImplementedError(f"Fetching data for {file_type} is not implemented.")
        
        if max_ticks is not None:
            df = df[:max_ticks]
        
        data_stream = df.iterrows()

        def _generate() -> Iterable[BarTick]:
            for _, row in data_stream:
                tick = BarTick(
                    timestamp=row['datetime'],
                    symbol=symbol,
                    open=row['open'],
                    high=row['high'],
                    low=row['low'],
                    close=row['close'],
                    volume=row['volume']
                )
                yield tick
        
        self.data_stream = _generate()

    def _fetch_data(self) -> BarTick:
        return next(self.data_stream)

