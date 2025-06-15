CREATE TABLE ccrs_crashes (
    collision_id INTEGER, -- the unique identifier of the crash report
    report_number VARCHAR2 (25), -- The unique identifier of the crash report within one NCIC, but it’s not unique across CA state
    report_version INTEGER, -- Version of the crash submitted
    is_preliminary CHAR(1), -- True/False -- when, due to unusual circumstances, the crash investigation cannot be submitted to the CHP within 15 working days. The preliminary investigation shall include at a minimum: (1) Number and names of involved parties. (2) Injuries. (3) A scene description. (4) A summary of the sequence of events that led to the crash.
    ncic_code VARCHAR2 (4), -- Four numerics assigned by DOJ
    crash_date_time TEXT, -- the date when the collision occurred (YYYYMMDD)
    crash_time_description VARCHAR2 (4), -- Data may appear with no leading zero(s). -- the time the crash occurred using a 24-hour clock
    beat VARCHAR2 (25), -- Assigned patrol area. This number may be one to three digits
    city_id INTEGER, -- The unique identifier of the city -- For internal storage
    city_code VARCHAR2 (10), -- The unique code of city in which the crash occurred.
    city_name VARCHAR2 (50), -- Name of the city where the crashed happened
    county_code INTEGER, -- The unique code of the county in which the crash occurred.
    city_is_active CHAR(1), -- True/False -- Defined by CHP whether the city is still valid or not in the database
    city_is_incorporated CHAR(1), -- True/False --
    collision_type_code CHAR(1), -- Define the type of a crash -- A B C D E F G H
    collision_type_description VARCHAR(50), -- A-HEAD-ON, B-SIDE SWIPE, C-REAR END, D-BROADSIDE, E-HIT OBJECT, F-OVERTURNED, G-VEHICLE/PEDESTRAIN, H-OTHER
    collision_type_other_desc VARCHAR(50),
    day_of_week VARCHAR(12), -- the day of the week when the crash occurred -- Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
    dispatch_notified INTEGER, -- 0 - No, 1 - Yes, 2 – NotApplicable -- Defined as smallint datatype.
    has_photographs CHAR(1), -- True/False -- Whether the crash report has photographs or not
    hit_run CHAR(1), -- F – Felony, M – Misdemeanor, Blank - None
    is_attachments_mailed CHAR(1), -- True/False --
    is_deleted CHAR(1), -- True/False -- Determine if a crash is deleted or not
    is_highway_related CHAR(1), -- True/False -- Determine if a crash happened on highway or not
    is_tow_away CHAR(1), -- True/False -- Determine if Vehicles involved in the crash towed away or not
    judicial_district VARCHAR(100), -- Judicial district where the crash occurred
    motor_vehicle_involved_with_code CHAR(1), -- A - Non-Collision, B - Pedestrian, C - Other Motor Vehicle, D - Motor Vehicle on Other Roadway, E - Parked Motor Vehicle, F - Train, G - Bicycle, H - Animal, I - Fixed Object, J - Other Object, -  - Not Stated
    motor_vehicle_involved_with_desc VARCHAR2 (50), -- Description of the Motor Vehicle Involved With code
    motor_vehicle_involved_with_other_desc VARCHAR2 (50), -- Additional description of the Motor Vehicle Involved With code
    number_injured INTEGER, -- Number injured in the crash
    number_killed INTEGER, -- Number killed in the crash
    weather_1 VARCHAR2 (50), -- Weather condition at the time of the crash -- A - Clear, B - Cloudy, C - Raining, D - Snowing, E - Fog, F - Other, G - Wind
    weather_2 VARCHAR2 (50), -- Weather condition at the time of the crash -- Same as weather 1 above
    road_condition_1 VARCHAR2 (50), -- Roadway condition at the time of the crash in the traffic lane(s) involved. -- A - Holes, Deep Ruts; B - Loose Material on Roadway; C - Obstruction on Roadway; D - Construction or Repair Zone; E - Reduced Roadway Width; F - Flooded; G - Other; H - No Unusual Condition
    road_condition_2 VARCHAR2 (50), -- Second roadway condition at the time of the crash in the traffic lane(s) involved. -- Same as road condition 1 above
    special_condition VARCHAR2 (50), -- Indicate type of crash report Note: if the report has more than one special conditions, the values are separated by a slash (/) -- Counter Report, Courtesy Report, Farm Labor Vehicle, Fatal, Hazardous Material, Late-Reported, On-Duty Emergency Vehicle, Preliminary, Private Property, School Bus Collision, No Pupils on School Bus, 550, Tribal Land Reportable, Tribal Land Non-Reportable, Autonomous Vehicle, MAIT (PRIMARY), MAIT SUPPLEMENTAL, Nonreportable, or Incident Only
    lighting_code CHAR(1), -- A-DAYLIGHT, B-DUSK-DAWN, C-DARK-STREET LIGHTS, D-DARK-NO STREET LIGHTS, E-DARK-STREET LIGHTS NOT FUNCTIONING*
    lighting_description VARCHAR2 (50), -- A-DAYLIGHT, B-DUSK-DAWN, C-DARK-STREET LIGHTS, D-DARK-NO STREET LIGHTS, E-DARK-STREET LIGHTS NOT FUNCTIONING*
    latitude FLOAT,
    longitude FLOAT,
    milepost_direction VARCHAR2 (2),
    milepost_distance FLOAT, -- reflects the distance from either the south or west county line, depending on the general direction of the highway, to that location.
    milepost_marker VARCHAR2 (100), -- Milepost markers indicate the route number, county, and post miles of the location
    milepost_unit_of_measure CHAR(1),
    pedestrian_action_code CHAR(1), -- A - No Pedestrian Involved, B - Crossing in Crosswalk at Intersection, C - Crossing in Crosswalk Not at Intersection, D - Crossing Not in Crosswalk, E - In Road, Including Shoulder, F - Not in Road, G - Approaching/Leaving School Bus
    pedestrian_action_desc VARCHAR(50), -- See above
    prepared_date TEXT, -- Date when the report was first prepared to key in the system
    primary_collision_factor_code CHAR(1), -- Primary crash factor -- A - (Vehicle) Code Violation, B - Other Improper Driving, C - Other Than Driver, D - Unknown, E - Fell Asleep, -  - Not Stated"
    primary_collision_factor_violation VARCHAR(50), -- Primary crash factor description -- see above
    primary_collision_factor_is_cited CHAR(1), -- True/False
    primary_collision_party_number INTEGER, -- Identify the party who is the primary crash factor
    primary_road VARCHAR2 (100), -- Road/location where the crash occurred
    reporting_district VARCHAR2 (100),
    reporting_district_code VARCHAR2 (10),
    reviewed_date TEXT, -- Date the report was reviewed for final approval
    road_surface_code CHAR(1), -- A-DRY, B-WET, C-SNOWY-ICY, D-SLIPPERY(MUDDY,OILY,ETC)
    secondary_direction CHAR(1), -- Direction of the secondary road from the primary road -- N - North, E - East, S - South, W - West, - or blank  - Not Stated, in Intersection
    secondary_distance FLOAT, -- Distance from the secondary road to the primary road
    secondary_road VARCHAR2 (100), -- The nearest crossed road where the crash occurred
    secondary_unit_of_measure CHAR(1), -- Unit of measure of the distance from the secondary road to the primary road
    sketch_desc TEXT,
    traffic_control_device_code CHAR(1), -- A-CONTROLS FUNCTIONING, B-CONTROLS NOT FUNCTIONING*, C-CONTROLS OBSCURED, D-NO CONTROLS PRESENT/FACTOR*
    created_date TEXT, -- Date and time when the report was created
    modified_date TEXT, -- Latest date and time when the report was modified
    is_county_road CHAR(1), -- True/False
    is_freeway CHAR(1), -- True/False
    chp555_version INTEGER, -- Version of the report identified by CHP -- 1,2,3,4
    is_additional_object_struck CHAR(1), -- True/False
    notification_date TEXT, -- Date and Time when the crashed was notified
    notification_time_description VARCHAR2 (10), -- the time the crash notified using a 24-hour clock
    has_digital_media_files CHAR(1), -- True/False
    evidence_number VARCHAR2 (25),
    is_location_refer_to_narrative CHAR(1), -- True/False
    is_aoi_one_same_as_location CHAR(1) -- True/False
)
