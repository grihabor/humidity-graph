import logging
import struct
import time
from datetime import datetime
from decimal import Decimal

import clickhouse_connect
import serial

logger = logging.getLogger(__name__)


def main():
    client = clickhouse_connect.get_client(
        host="localhost",
        port=8123,
        username="default",
        password="",
    )
    while True:
        try:
            loop(client)
        except Exception:
            logger.exception("loop error")

        time.sleep(1)


def loop(client):
    with serial.Serial("/dev/rfcomm0", 9600, timeout=5) as ser:
        while True:
            buf = ser.read(4)
            humidity, temperature = struct.unpack("<hh", buf)

            data = [
                [
                    datetime.now(),
                    Decimal(humidity) / Decimal(10),
                    Decimal(temperature) / Decimal(10),
                ],
            ]
            client.insert(
                "sensor_data",
                data,
                column_names=["ts", "humidity", "temperature"],
            )
            print("hum", humidity, "temp", temperature)


if __name__ == "__main__":
    main()
