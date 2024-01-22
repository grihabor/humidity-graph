CREATE TABLE sensor_data
(
    `ts` DateTime,
    `humidity` Decimal(4, 1) NOT NULL,
    `temperature` Decimal(3, 1) NOT NULL
)
ENGINE = MergeTree
PRIMARY KEY tuple(ts)
