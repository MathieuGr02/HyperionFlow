import json
import os

from kafka import KafkaConsumer

CURRENT_KEY = 0

def key_decode(item) -> str:
    if item is None:
        return ""
    return item.decode("utf-8")

consumer = KafkaConsumer("test_out", bootstrap_servers=os.environ["KAFKA_BOOTSTRAP_SERVERS"], key_deserializer=key_decode, value_deserializer=lambda value: json.loads(value.decode("utf-8")))

while True:
    for msg in consumer:
        print(msg)
