import json
import os
from time import sleep

from kafka import KafkaProducer

CURRENT_KEY = 0

producer = KafkaProducer(
    bootstrap_servers=os.environ["KAFKA_BOOTSTRAP_SERVERS"],
    key_serializer=lambda key: key.encode("utf-8"),
    value_serializer=lambda value: json.dumps(value).encode("utf-8"),
)

topic = os.environ.get("KAFKA_TOPIC", "test")

while True:
    producer.send(topic, key=str(CURRENT_KEY), value={"id": CURRENT_KEY, "name": "Test"})
    CURRENT_KEY += 1
    sleep(1)

producer.flush()
producer.close()
print(f"Published 3 messages to {topic}")
