#!/bin/bash

#
# Start by importing the collision data.
#

# During loading of collision data, insert an "ADDRESS" column that contains
# the street names of the closest intersection to the collision. This allows
# the collision to be (roughly) geocoded in the event lat/long are not provided
# (SWITRS normally provides lat/long after the start of 2017, but not before).
# Note that a better geocoding would use the DISTANCE and DIRECTION fields to 
# place the collision location when the collision was not in an intersection.
sqlite-utils insert records.db collisions CollisionRecords.txt --csv -d --convert '
	row["ADDRESS"] = row["PRIMARY_RD"] + " and " + row["SECONDARY_RD"] + ", Berkeley, CA"
	row["SEVERITY_INDEX"] = row["COLLISION_SEVERITY"]
	return row
'

# CHP I-SWITRS data arrives with the longitude value as a positive number
# (e.g., 122.26456) when those values should be negative (-122.26456). This
# is a quick fix.
sqlite-utils convert records.db collisions LONGITUDE 'value * (-1)'

# Clean up the date format a bit.
sqlite-utils transform records.db collisions --pk CASE_ID \
	--type PROC_DATE text \
	--type COLLISION_DATE text 

sqlite-utils convert records.db collisions PROC_DATE COLLISION_DATE 'r.parsedate(value)'

# Street names show up with trailing spaces often, sometimes multiple spaces. There are a 
# bunch of other street name problems, too (like '50 feet south of Adeline' and 'Adeline 2800'),
# but this is a start.
sqlite-utils convert records.db collisions PRIMARY_RD SECONDARY_RD 'value.strip()'

#
# Now load the party and victim data.
#

sqlite-utils insert records.db parties PartyRecords.txt --csv -d

sqlite-utils insert records.db victims VictimRecords.txt --csv -d

sqlite-utils add-foreign-keys records.db    \
	parties CASE_ID collisions CASE_ID      \
	victims CASE_ID collisions CASE_ID	    

#
# Load lookup tables for integer codes
#

# [CASE_ID] INTEGER PRIMARY KEY,
# [ACCIDENT_YEAR] INTEGER,
# [PROC_DATE] INTEGER,
# [JURIS] INTEGER,
# [COLLISION_DATE] INTEGER,
# [COLLISION_TIME] INTEGER,
# [OFFICER_ID] TEXT,
# [REPORTING_DISTRICT] TEXT,
# [DAY_OF_WEEK] INTEGER,
# [CHP_SHIFT] INTEGER,
# [POPULATION] INTEGER,
# [CNTY_CITY_LOC] INTEGER,
# [SPECIAL_CONDITION] INTEGER,
# [BEAT_TYPE] INTEGER,
# [CHP_BEAT_TYPE] TEXT,
# [CITY_DIVISION_LAPD] INTEGER,
# [CHP_BEAT_CLASS] INTEGER,
# [BEAT_NUMBER] TEXT,
# [PRIMARY_RD] TEXT,
# [SECONDARY_RD] TEXT,
# [DISTANCE] FLOAT,
# [DIRECTION] TEXT,
# [INTERSECTION] TEXT,
# [WEATHER_1] TEXT,
# [WEATHER_2] TEXT,
# [STATE_HWY_IND] TEXT,
# [CALTRANS_COUNTY] TEXT,
# [CALTRANS_DISTRICT] INTEGER,
# [STATE_ROUTE] INTEGER,
# [ROUTE_SUFFIX] TEXT,
# [POSTMILE_PREFIX] TEXT,
# [POSTMILE] FLOAT,
# [LOCATION_TYPE] TEXT,
# [RAMP_INTERSECTION] TEXT,
# [SIDE_OF_HIGHWAY] TEXT,
# [TOW_AWAY] TEXT,
# [COLLISION_SEVERITY] INTEGER,
# [NUMBER_KILLED] INTEGER,
# [NUMBER_INJURED] INTEGER,
# [PARTY_COUNT] INTEGER,
# [PRIMARY_COLLISION_FACTOR] TEXT,
# [PCF_VIOLATION_CODE] TEXT,
# [PCF_VIOLATION_CATEGORY] TEXT,
# [PCF_VIOLATION] INTEGER,
# [PCF_VIOL_SUBSECTION] TEXT,
# [HIT_AND_RUN] TEXT,
# [TYPE_OF_COLLISION] TEXT,
# [MOTOR_VEHICLE_INVOLVED_WITH] TEXT,
# [PED_ACTION] TEXT,
# [ROAD_SURFACE] TEXT,
# [ROAD_CONDITION_1] TEXT,
# [ROAD_COND_2] TEXT,
# [LIGHTING] TEXT,
# [CONTROL_DEVICE] TEXT,
# [CHP_ROAD_TYPE] INTEGER,
# [PEDESTRIAN_ACCIDENT] TEXT,
# [BICYCLE_ACCIDENT] TEXT,
# [MOTORCYCLE_ACCIDENT] TEXT,
# [TRUCK_ACCIDENT] TEXT,
# [NOT_PRIVATE_PROPERTY] TEXT,
# [ALCOHOL_INVOLVED] TEXT,
# [STWD_VEHTYPE_AT_FAULT] TEXT,
# [CHP_VEHTYPE_AT_FAULT] TEXT,
# [COUNT_SEVERE_INJ] INTEGER,
# [COUNT_VISIBLE_INJ] INTEGER,
# [COUNT_COMPLAINT_PAIN] INTEGER,
# [COUNT_PED_KILLED] INTEGER,
# [COUNT_PED_INJURED] INTEGER,
# [COUNT_BICYCLIST_KILLED] INTEGER,
# [COUNT_BICYCLIST_INJURED] INTEGER,
# [COUNT_MC_KILLED] INTEGER,
# [COUNT_MC_INJURED] INTEGER,
# [PRIMARY_RAMP] TEXT,
# [SECONDARY_RAMP] TEXT,
# [LATITUDE] FLOAT,
# [LONGITUDE] FLOAT

# Some tables use integer ids, like sensible tables do. Let's import them first
# since we favor them.

for TABLE in DAY_OF_WEEK CHP_SHIFT POPULATION SPECIAL_CONDITION BEAT_TYPE COLLISION_SEVERITY
do
	sqlite-utils create-table records.db $TABLE id integer name text --pk=id
	sqlite-utils insert records.db $TABLE lookup-tables/$TABLE.csv --csv
	sqlite-utils add-foreign-key records.db collisions $TABLE $TABLE id
	sqlite-utils create-index records.db collisions $TABLE
done

# *Other* tables use letter keys, like they were raised by WOLVES. Let's put them
# at the end of the import queue.

for TABLE in WEATHER_1 WEATHER_2 LOCATION_TYPE RAMP_INTERSECTION SIDE_OF_HIGHWAY \
PRIMARY_COLLISION_FACTOR PCF_VIOLATION_CODE PCF_VIOLATION_CATEGORY TYPE_OF_COLLISION MOTOR_VEHICLE_INVOLVED_WITH \
PED_ACTION ROAD_SURFACE ROAD_CONDITION_1 ROAD_COND_2 LIGHTING CONTROL_DEVICE \
STWD_VEHTYPE_AT_FAULT CHP_VEHTYPE_AT_FAULT PRIMARY_RAMP SECONDARY_RAMP
do
	sqlite-utils create-table records.db $TABLE key text name text --pk=key
	sqlite-utils insert records.db $TABLE lookup-tables/$TABLE.csv --csv
	sqlite-utils add-foreign-key records.db collisions $TABLE $TABLE key
	sqlite-utils create-index records.db collisions $TABLE
done	

# Drop some columns that just aren't that useful, or at least aren't useful yet.
# These are entered by line so they can be easily commented out later if the
# time comes to care about them.

sqlite-utils transform records.db collisions \
	--drop JURIS \
    --drop ACCIDENT_YEAR \
    --drop PROC_DATE \
    --drop POPULATION \
    --drop CHP_SHIFT \
    --drop CHP_BEAT_TYPE \
    --drop BEAT_TYPE \
    --drop CITY_DIVISION_LAPD \
    --drop CHP_BEAT_CLASS \
    --drop CHP_ROAD_TYPE \
    --drop PCF_VIOLATION_CODE \
    --drop SPECIAL_CONDITION \
    --drop ROUTE_SUFFIX \
    --drop POSTMILE_PREFIX
