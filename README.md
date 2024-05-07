# switrs-db

Tool for extracting data from the [SWITRS database](https://www.chp.ca.gov/programs-services/services-information/switrs-internet-statewide-integrated-traffic-records-system) into SQLITE files.

There are two tools in this repo. The original shell script by marc, and the newer Rust based CLI. Both should work, but the Rust tool works faster and with fewer dependencies.

## switrs-db cli

This requires Rust, please install from here: https://rustup.rs

or just run this command

```shell
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Run the CLI from Cargo (simplest method), run from the project root directory, everything after the `--` are options to the command

```shell
> cargo run -r -- --help
Usage: switrs-db [OPTIONS] -d <DATA_PATH> -f <SQLITE_FILE>

Options:
  -d <DATA_PATH>        Path to the raw data dump from iswitrs
  -f <SQLITE_FILE>      SQLITE db file to create from the raw data
  -s <SCHEMA>           Path to the Schemas TOML configuration file [default: Schemas.toml]
  -h, --help            Print help
  -V, --version         Print version
```

- Download the raw SWITRS db from https://iswitrs.chp.ca.gov/Reports/jsp/RawData.jsp

Ensure you've navigated to the `Raw Data` section.
In the `INCLUDES IN THE REPORT FILE` section, select both `LAT/LONG` and `HEADER` options. It's fast enough to download the entire DB from the past, e.g. 2010. *note* TBD for a start date.

Insert dates for the the request, await email. Download and unzip the file. Copy the path to the file, this will hence forth be referred to as `${REPORT_DIR}`

- Run the CLI

Extract the raw data, for example a directory similar to this `~/Downloads/4851866028832156906`

Run the CLI, this will put the DB into `target/switrs.sqlite`

```shell
> cargo run -r -- -d target/4481761401380215189 -f target/switrs.sqlite
Loading data from target/4481761401380215189 and writing to target/switrs.sqlite
LOADING ...
LOADING collisions
LOADING parties
LOADING victims
Successfully imported data, writing DB to target/switrs.sqlite
```

Now the sqlite tools or other programs can be used with the DB.

```shell
> sqlite3 target/switrs.sqlite
SQLite version 3.43.2 2023-10-10 13:08:14
Enter ".help" for usage hints.
sqlite> SELECT * FROM collisions_view WHERE primary_rd LIKE "%Hopkins%" AND bicycle_accident == 'Y';
6141237|HOPKINS ST and PERALTA AV Berkeley, CA|2014-03-07|2013-06-18T17:04|HOPKINS ST|PERALTA AV|||Y|0|0|1|0|0|0|1|||Tuesday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|East|Clear|Not Stated||||Injury (Complaint of Pain)|(Vehicle) Code Violation|Not Stated|Improper Turning|Not Hit and Run|Overturned|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Bicycle|Bicycle|Not Stated|Not Stated
7181630|HOPKINS ST and PERALTA AV Berkeley, CA|2016-02-18|2015-12-01T08:20|HOPKINS ST|PERALTA AV|||Y|0|1|0|0|0|0|1|||Tuesday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|West|Clear|Not Stated||||Injury (Other Visible)|(Vehicle) Code Violation|Not Stated|Unsafe Speed|Not Hit and Run|Hit Object|Fixed Object|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Bicycle|Bicycle|Not Stated|Not Stated
8050043|HOPKINS ST and CARLOTTA AV Berkeley, CA|2016-06-10|2016-04-30T12:30|HOPKINS ST|CARLOTTA AV|||Y|1|0|0|0|0|0|1|||Saturday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|East|Clear|Not Stated||||Injury (Severe)|(Vehicle) Code Violation|Not Stated|Improper Turning|Not Hit and Run|Rear End|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Bicycle|Bicycle|Not Stated|Not Stated
8091540|HOPKINS ST and CEDAR ST Berkeley, CA|2016-07-29|2016-06-01T16:59|HOPKINS ST|CEDAR ST|||Y|0|0|1|0|0|0|1|||Wednesday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP||Clear|Not Stated||||Injury (Complaint of Pain)|Not Stated|Not Stated|Not Stated|Not Hit and Run|Sideswipe|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Not Stated|Not Stated or Unknown (Hit and Run)|Not Stated|Not Stated
8375570|HOPKINS and BEVERLY PL Berkeley, CA|2017-06-09|2017-04-02T09:28|HOPKINS|BEVERLY PL|||Y|0|0|1|0|0|0|1|37.88425|-122.27676|Sunday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|West|Clear|Not Stated||||Injury (Complaint of Pain)|(Vehicle) Code Violation|Not Stated|Other Hazardous Violation|Not Hit and Run|Vehicle/Pedestrian|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Passenger Car/Station Wagon|Passenger Car, Station Wagon, or Jeep|Not Stated|Not Stated
8446899|HOPKINS ST and MONTEREY AV Berkeley, CA|2017-09-14|2017-06-11T15:07|HOPKINS ST|MONTEREY AV|||Y|1|0|0|0|0|0|1|37.88168|-122.28193|Sunday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|West|Clear|Not Stated||||Injury (Severe)|(Vehicle) Code Violation|Not Stated|Unsafe Speed|Not Hit and Run|Rear End|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Bicycle|Bicycle|Not Stated|Not Stated
8595513|HOPKINS ST and CURTIS ST Berkeley, CA|2018-05-01|2018-02-05T12:21|HOPKINS ST|CURTIS ST|||Y|0|1|0|0|0|0|1|||Monday|Not CHP|Incorporated (100000 - 250000)|Berkeley|Not Above|Not CHP|Not CHP|West|Clear|Not Stated||||Injury (Other Visible)|(Vehicle) Code Violation|Not Stated|Improper Turning|Not Hit and Run|Broadside|Bicycle|No Pedestrian Involved|Dry|No Unusual Condition|Not Stated|Daylight|None|Bicycle|Bicycle|Not Stated|Not Stated
```

## import-bicycl-crashes.sh

The original shell script from marc

*note*: These instructions rely on `brew` being installed, https://brew.sh

- Install sqlite-utils and just:

```shell
brew install sqlite-utils just
```

- Download the raw SWITRS db from https://iswitrs.chp.ca.gov/Reports/jsp/RawData.jsp

Ensure you've navigated to the `Raw Data` section.
In the `INCLUDES IN THE REPORT FILE` section, select both `LAT/LONG` and `HEADER` options. It's fast enough to download the entire DB from the past, e.g. 2010. *note* TBD for a start date.

Insert dates for the the request, await email. Download and unzip the file. Copy the path to the file, this will hence forth be referred to as `${REPORT_DIR}`

- just run the build target from

Using the directory from above as `${REPORT_DIR}`:

```shell
just build ${REPORT_DIR}
```

For example, if the download was extracted in your `~/Downloads` directory and named `4481761401380215189`, the output would be something like:

```shell
> just build ~/Downloads/4481761401380215189
mkdir -pv /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build
/Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build
[[ -L /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build/lookup-tables ]] || ln -s /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/lookup-tables /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build/lookup-tables 
cd target/4481761401380215189 && cp CollisionRecords.txt PartyRecords.txt VictimRecords.txt /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build
cd /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/target/build && /bin/bash /Users/benjaminfry/Development/radical-bike-lobby/switrs-db/import-bicycle-crashes.sh
  [###################################-]   99%  00:00:00
  [####################################]  100%
  [####################################]  100%          
  [####################################]  100%
  [###################################-]   99%  00:00:00
  [###################################-]   97%  00:00:00
  [#################################---]   91%
  [##################################--]   94%
  [##################################--]   96%
  [###################################-]   98%
  [###################################-]   97%
  [##################################--]   95%
  [################################----]   90%
  [################################----]   90%
  [#################################---]   94%
  [##################################--]   96%
  [##################################--]   94%
  [##################################--]   95%
  [##################################--]   95%
  [##################################--]   95%
  [#################################---]   93%
  [#################################---]   94%
  [##################################--]   96%
  [#################################---]   94%
  [##################################--]   95%
  [##################################--]   95%
  [##################################--]   95%
  [#################################---]   93%
  [##################################--]   95%
  [###################################-]   97%
  [##################################--]   94%
  [##################################--]   94%
```

- Use the Sqlite DB

The Sqlite DB should be in `target/build/records.db`.

For example, dump the schema:

```shell
sqlite-utils schema target/build/records.db
```
