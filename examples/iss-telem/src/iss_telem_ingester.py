import time
import datetime as dt
from lightstreamer.client import LightstreamerClient, Subscription, SubscriptionListener
import json
from questdb.ingress import Sender, Protocol

SCHEMA = [
    "TimeStamp",
    "Value",
    "Status.Class",
    "Status.Indicator",
    "Status.Color",
    "CalibratedData",
]


def iss_time_to_timestamp(iss_time):

    year = dt.datetime.now(dt.timezone.utc).year

    day_in_year = int(iss_time // 24)

    # Calculate the remaining hours after removing the full days.
    remainder_hours = iss_time - day_in_year * 24
    hours = int(remainder_hours)

    # Convert the fractional hour to minutes.
    remainder_minutes = (remainder_hours - hours) * 60
    minutes = int(remainder_minutes)

    # Convert the fractional minute to seconds.
    seconds = (remainder_minutes - minutes) * 60

    return dt.datetime(year, 1, 1, tzinfo=dt.timezone.utc) + dt.timedelta(
        days=day_in_year - 1, hours=hours, minutes=minutes, seconds=seconds
    )


class SubListener():
    db_conf = "http::addr="
    def __init__(self, table_name, telem_definition: dict = {}):
        # SubscriptionListener.__init__(self)

        self.db_sender = Sender(
            Protocol.Http,
            'artemix-db',
            9000,
            auto_flush=True,
            auto_flush_interval=dt.timedelta(seconds=5)
        )
        self.db_sender.establish()

        self.table_name = table_name
        self.telem_definition = telem_definition

    def __del__(self):
        if hasattr(self, 'db_sender'):
            self.db_sender.flush()
            self.db_sender.close()

    def onItemUpdate(self, update):
        # print(f"UPDATE: {update}")
        item_name = update.getItemName()
        telem_def_row = self.telem_definition.get(item_name, {})

        columns = {"value": update.getValue("Value"), "description": telem_def_row.get("Description")}
        if telem_def_row.get("Units") == "STATE":
            columns["short"] = "UNKNOWN"

        # self.db_sender.row(
        #     self.table_name,
        #     symbols={
        #         "parent": telem_def_row.get("Parent", ""),
        #         "channel": telem_def_row.get("Name", "")
        #     },
        #     columns=columns,
        #     at=iss_time_to_timestamp(float(update.getValue("TimeStamp")))
        # )
        self.db_sender.flush()
        print("sent")
        
        # print(f"Telemetry from {update.getItemName()}")
        # for line in SCHEMA:
            # value = update.getValue(line)
            # print(f"\t{line}: {value}")
            # if line == "TimeStamp":
                # print(f"\tUnixTimestamp: {iss_time_to_timestamp(float(value))}")


if __name__ == "__main__":

    table_name = "iss_001"

    client = LightstreamerClient("http://push.lightstreamer.com", "ISSLIVE")
    client.connect()

    # Import the ISS Telemetry definitions
    iss_telem_def = {}
    with open("src/ISS_Public_Telemetry.json", "r") as file:
        iss_telem_def = json.load(file)

    channels = iss_telem_def.keys()

    sub = Subscription("MERGE", channels, SCHEMA)
    sub.addListener(SubListener(table_name=table_name, telem_definition=iss_telem_def))
    client.subscribe(sub)

    try:
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        print("Keyboard interrupt!")
        sub.__del__()

    client.unsubscribe(sub)
    client.disconnect()

    print("Goodbye")
