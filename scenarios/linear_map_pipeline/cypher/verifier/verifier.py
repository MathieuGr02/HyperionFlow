from ast import arg
from math import exp
import json
from math import isclose
import os
from threading import Thread
from time import sleep

from kafka import KafkaProducer, KafkaConsumer
from queue import Queue

def create_node(id: int) -> dict:
    return {
        "_type": "node",
        "id": id,
        "labels": ["Person"],
        "value":
            {
                "id": id,
                "name": "mg"
            }
    };

def create_edge(from_id: int, to_id: int) -> dict:
    return {
        "_type": "edge",
        "label": "KNOWS",
        "from":
            {
                "id": from_id,
                "label": "Person"
            },
        "to":
            {
                "id": to_id,
                "label": "Person"
            },
        "value": { }
    }

def producer(produced_items: Queue):
    CURRENT_KEY = 1

    producer = KafkaProducer(
        bootstrap_servers=os.environ["KAFKA_BOOTSTRAP_SERVERS"],
        key_serializer=lambda key: key.encode("utf-8"),
        value_serializer=lambda value: json.dumps(value).encode("utf-8"),
    )

    while True:
        key, value = str(CURRENT_KEY), create_node(CURRENT_KEY)
        res = producer.send("test_in", key=key, value=value)
        while not res.is_done:
            print("Waiting")
            sleep(0.1)
        print(f"Adding produced message {(key, value)}")
        produced_items.put((key, value))
        sleep(1)

        if CURRENT_KEY % 2 == 0:
            key, value = str(CURRENT_KEY), create_edge(CURRENT_KEY - 1, CURRENT_KEY)
            res = producer.send("test_in", key=key, value=value)
            while not res.is_done:
                print("Waiting")
                sleep(0.1)
            sleep(1)

        CURRENT_KEY += 1


def consumer(consumed_items: Queue):
    def key_decode(item) -> str:
        if item is None:
            return ""
        return item.decode("utf-8")

    consumer = KafkaConsumer("test_out", bootstrap_servers=os.environ["KAFKA_BOOTSTRAP_SERVERS"], key_deserializer=key_decode, value_deserializer=lambda value: json.loads(value.decode("utf-8")))

    while True:
        for msg in consumer:
            print(f"Adding consumed message {msg}")
            consumed_items.put((msg.key, msg.value))


def verifier(produced_items: Queue, consumed_items: Queue):
    while True:
        produced_item = produced_items.get()
        consumed_item = consumed_items.get()

        id_equal = produced_item[1]["id"] == consumed_item[1]["id"]
        a = float(produced_item[1]["value"])
        b = float(consumed_item[1]["value"])
        correct_map = isclose(exp(a), b)
        print(f"CHECKING RECORDS ({produced_item} | {consumed_item}) | ID EQUAL: {id_equal} | CORRECT MAP: {correct_map}")

if __name__ == "__main__":
    produced_items = Queue()
    consumed_items = Queue()

    print("Creating threads")
    producer_thread = Thread(target=producer, args=(produced_items,))
    consumer_thread = Thread(target=consumer, args=(consumed_items,))
    verifier_thread = Thread(target=verifier, args=(produced_items, consumed_items))

    print("Starting threads")
    producer_thread.start()
    consumer_thread.start()
    verifier_thread.start()

    producer_thread.join()
    consumer_thread.join()
    verifier_thread.join()

    print("Done")
