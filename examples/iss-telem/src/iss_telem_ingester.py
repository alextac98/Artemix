import time
import datetime as dt
from lightstreamer.client import LightstreamerClient, Subscription, SubscriptionListener

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

    return dt.datetime(year, 1, 1, tzinfo=dt.timezone.utc) + dt.timedelta(days=day_in_year - 1, hours=hours, minutes=minutes, seconds=seconds)

class SubListener(SubscriptionListener):
    def onItemUpdate(self, update):
        # print(f"UPDATE: {update}")
        print("New Telemetry!")
        for line in SCHEMA:
            value = update.getValue(line)
            print(f"\t{line}: {value}")
            if line == "TimeStamp":
                print(f"\tUnixTimestamp: {iss_time_to_timestamp(float(value))}") 


if __name__ == "__main__":

    client = LightstreamerClient("http://push.lightstreamer.com", "ISSLIVE")
    client.connect()

    sub = Subscription("MERGE", ["TIME_000001"], SCHEMA)
    sub.addListener(SubListener())
    client.subscribe(sub)

    try:
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        print("Keyboard interrupt!")

    client.unsubscribe(sub)
    client.disconnect()

    print("Goodbye")
