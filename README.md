# swirts-db

Tool for extracting data from the [SWITRS database](https://www.chp.ca.gov/programs-services/services-information/switrs-internet-statewide-integrated-traffic-records-system) into SQLITE files.

## Getting started (macos)

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
