import pika
import pandas as pd

from typing import List
import json
import threading

from orbital.model.tick import BaseTick, BarTick
from orbital.strategy.base import BaseStrategy
from orbital.data.client import BaseDataClient

class BacktestNode:

    def __init__(
        self,
        strategy: BaseStrategy,
        data_clients: List[BaseDataClient],
    ):
        self.strategy = strategy
        self.data_clients = data_clients

    def _consume_tick(self):
        connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
        channel = connection.channel()
        channel.queue_declare("tick")

        def deserialize(obj):
            if 'timestamp' in obj:
                timestamp_str = obj['timestamp']
                obj['timestamp'] = pd.Timestamp(timestamp_str)
            return obj

        def callback(ch, method, properties, body):
            tick_data = json.loads(body, object_hook=deserialize)
            class_name = tick_data.pop('class_name', None)

            if class_name is None:
                # If class_name is not provided in the data, fallback to BaseTick
                tick = BaseTick(**tick_data)
            else:
                # Dynamically get the class using globals()
                tick_class = globals().get(class_name)
                if tick_class and issubclass(tick_class, BaseTick):
                    # Create an instance of the appropriate subclass
                    tick = tick_class(**tick_data)
                else:
                    raise ValueError(f"Invalid class_name: {class_name}")

            self.strategy.handle_data(tick)
 
        channel.basic_consume(
            queue='tick',
            on_message_callback=callback,
            auto_ack=True
        )
        channel.start_consuming()

    def _start_publishing_ticks(self):

        connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
        channel = connection.channel()
        channel.queue_declare("tick")

        active_clients = {client: True for client in self.data_clients}

        while any(active_clients.values()):
            for data_client in self.data_clients:
                try:
                    if active_clients[data_client]:
                        data = data_client._fetch_data()
                        channel.basic_publish(exchange='', routing_key='tick', body=json.dumps(data, default=lambda o: o.__dict__))
                except StopIteration:
                    active_clients[data_client] = False
                    print(f"Data client {data_client} completed publishing")

        print("All data clients completed publishing")


    def run(self):
        # prime consumers for consumption
        consumer_thread = threading.Thread(target=self._consume_tick)
        consumer_thread.start()

        # start publishing
        publisher_thread = threading.Thread(target=self._start_publishing_ticks)
        publisher_thread.start()

        # Join both threads to ensure they both finish
        # consumer_thread.join()
        publisher_thread.join()
