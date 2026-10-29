import json
import os
from random import random
from time import sleep

from kafka import KafkaProducer

CURRENT_KEY = 0

producer = KafkaProducer(
    bootstrap_servers=os.environ["KAFKA_BOOTSTRAP_SERVERS"],
    key_serializer=lambda key: key.encode("utf-8"),
    value_serializer=lambda value: json.dumps(value).encode("utf-8"),
)

while True:
    producer.send("test_in", key=str(CURRENT_KEY), value={"id": CURRENT_KEY, "value": random()})
    CURRENT_KEY += 1
    sleep(1)
