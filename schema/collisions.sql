CREATE TABLE collisions (
    case_id VARCHAR2(19), -- Case Id: the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    accident_year INTEGER, -- Collision Year: the year when the collision occurred
    proc_date INTEGER, -- Process Date: (YYYYMMDD)
    juris INTEGER, -- Jurisdiction: Four numerics assigned by DOJ
    collision_date INTEGER, -- Collision Date: the date when the collision occurred (YYYYMMDD)	
    collision_time INTEGER, -- Collision Time: the time when the collision occurred (24 hour time)	Data may appear with no leading zero(s).
    officer_id VARCHAR2(8), -- Officer Id
    reporting_district VARCHAR2(5), -- Reporting District
    day_of_week CHAR(1), -- Day of Week: the code for the day of the week when the collision occurred (see lookup-tables/DAY_OF_WEEK.csv)
    chp_shift CHAR(1), -- CHP Shift (see lookup-tables/CHP_SHIFT.csv)
    population CHAR(1), -- Population (see lookup-tables/POPULATION.csv)
    cnty_city_loc VARCHAR2(4), -- County City Location: the location code of where the collision occurred. Data may appear with no leading zero.
    special_cond CHAR(1), -- Special Condition (see lookup-tables/SPECIAL_CONDITION.csv)
    beat_type CHAR(1), -- Beat Type (see lookup-tables/BEAT_TYPE.csv)
    chp_beat_type CHAR(1), -- CHP Beat Type (see lookup-tables/CHP_BEAT_TYPE.csv)
    city_division_lapd CHAR(1), -- City Division LAPD: Includes blanks and dashes as not stated.
    chp_beat_class CHAR(1), -- CHP Beat Class
    beat_number VARCHAR2(6), -- Beat Number
    primary_rd VARCHAR2(50), -- Primary Rd
    secondary_rd VARCHAR2(50), -- Secondary Rd
    distance DECIMAL(9,2), -- Distance: distance converted to feet
    direction CHAR(1), -- Direction (see lookup-tables/DIRECTION.csv)
    intersection CHAR(1), -- Intersection: Y - Intersection, N - Not Intersection, Blank - Not stated
    weather_1 CHAR(1), -- Weather 1: the weather condition at the time of the collision (see lookup-tables/WEATHER_1.csv)
    weather_2 CHAR(1), -- Weather 2: the weather condition at the time of the collision, if a second description is necessary (see lookup-tables/WEATHER_1.csv)
    state_hwy_ind CHAR(1), -- State Highway Indicator: Y - State Highway, N - Not State Highway, Blank - Not stated
    caltrans_county CHAR(3), -- Caltrans County: Includes blanks and nulls
    caltrans_district INTEGER, -- Caltrans District
    state_route INTEGER, -- State Route: 0 = Not State Highway
    route_suffix CHAR(1), -- Route Suffix
    postmile_prefix Char(1), -- Postmile Prefix
    postmile DECIMAL(6,3), -- Postmile
    location_type CHAR(1), -- Location Type (see lookup-tables/LOCATION_TYPE.csv)
    ramp_intersection CHAR(1), -- Ramp Intersection (see lookup-tables/RAMP_INTERSECTION.csv)
    side_of_hwy CHAR(1), -- Side Of Highway: Code provided by Caltrans Coders; applies to divided highway, based on nominal direction of route; for single vehicle is same as nominal direction of travel, overruled by impact with second vehicle after crossing median (see lookup-tables/SIDE_OF_HIGHWAY.csv)
    tow_away CHAR(1), -- Tow Away: Y - Yes, N - No
    collision_severity CHAR(1), -- Collision Severity (see lookup-tables/COLLISION_SEVERITY.csv)
    number_killed INTEGER, -- Killed victims: counts victims in the collision with degree of injury of 1 0 to N for each collision
    number_injured INTEGER, -- Injured victims: counts victims in the collision with degree of injury of 2, 3, or 4	0 to N for each collision
    party_count INTEGER, -- Party Count: counts total parties in the collision 1 to N for each collision
    primary_coll_factor CHAR(1), -- Primary Collision Factor (see lookup-tables/PRIMARY_COLLISION_FACTOR.csv)
    pcf_code_of_viol CHAR(1), -- PCF Violation Code (see lookup-tables/PCF_VIOLATION_CODE.csv)
    pcf_viol_category CHAR(2), -- PCF Violation Category (see lookup-tables/PCF_VIOLATION_CATEGORY.csv)
    pcf_violation INTEGER, -- PCF Violation
    pcf_viol_subsection CHAR(1), -- PCF Violation Subsection: Blank if no subsection.
    hit_and_run CHAR(1), -- Hit And Run (see lookup-tables/HIT_AND_RUN.csv)
    type_of_collision CHAR(1), -- Type of Collision (see lookup-tables/TYPE_OF_COLLISION.csv)
    mviw CHAR(1), -- Motor Vehicle Involved With (see lookup-tables/MOTOR_VEHICLE_INVOLVED_WITH.csv)
    ped_action CHAR(1), -- Ped Action (see lookup-tables/PED_ACTION.csv)
    road_surface CHAR(1), -- Road Surface (see lookup-tables/ROAD_SURFACE.csv)
    road_cond_1 CHAR(1), -- Road Condition 1 (see lookup-tables/ROAD_CONDITION_1.csv)
    road_cond_2 CHAR(1), -- Road Condition 2 same as road condition 1 above (see lookup-tables/ROAD_CONDITION_1.csv)
    lighting CHAR(1), -- Lighting (see lookup-tables/LIGHTNING.csv)
    control_device CHAR(1), -- Control Device (see lookup-tables/CONTROL_DEVICE.csv)
    chp_road_type CHAR(1), -- CHP Road Type: May be blank
    pedestrian_accident CHAR(1), -- Pedestrian Collision: indicates whether the collision involved a pedestrian, Y or blank
    bicycle_accident CHAR(1), -- Bicycle Collision: indicates whether the collision involved a bicycle, Y or blank
    motorcycle_accident CHAR(1), -- Motorcycle Collision: indicates whether the collision involved a motorcycle, Y or blank
    truck_accident CHAR(1), -- Truck Collision: indicates whether the collision involved a big truck, Y or blank
    not_private_property CHAR(1), -- Not Private Property: indicates whether the collision occurred on private property, Y or blank
    alcohol_involved CHAR(1), -- Alcohol Involved: indicates whether the collision involved a party that had been drinking, Y or blank
    stwd_vehtype_at_fault CHAR(1), -- Statewide Vehicle Type At Fault: indicates the Statewide Vehicle Type of the party who is at fault, see Party folder Statewide Vehicle Type item (see lookup-tables/STWD_VEHTYPE_AT_FAULT.csv)
    chp_vehtype_at_fault CHAR(2), -- CHP Vehicle Type At Fault: indicates the CHP Vehicle Type of the party who is at fault, see Party folder CHP Vehicle Type Towing item (see lookup-tables/CHP_VEHTYPE_AT_FAULT.csv)
    count_severe_inj INTEGER, -- Severe Injury count: counts victims in the collision with degree of injury of 2, 0 to N for each collision
    count_visible_inj INTEGER, -- Other Visible Injury count: counts victims in the collision with degree of injury of 3, 0 to N for each collision
    count_complaint_pain INTEGER, -- Complaint of Pain Injury count: counts victims in the collision with degree of injury of 4, 0 to N for each collision
    count_ped_killed INTEGER, -- Pedestrian Killed count: Counts the victims in the collision with party type of 2 and degree of injury is 1, 0 or 1 for each collision
    count_ped_injured INTEGER, -- Pedestrian Injured count: Counts the victims in the collision with party type of 2 and degree of injury is 2, 3, or 4. 0 or 1 for each collision
    count_bicyclist_killed INTEGER, -- Bicyclist Killed count: Counts the victims in the collision with party type of 4 and degree of injury is 1. 0 to N for each collision
    count_bicyclist_injured INTEGER, -- Bicyclist Injured count: Counts the victims in the collision with party type of 4 and degree of injury is 2, 3, or 4. 0 to N for each collision
    count_mc_killed INTEGER, -- Motorcyclist Killed count: counts victims in the collision with statewide vehicle type of C or O and degree of injury of 1. 0 to N for each collision
    count_mc_injured INTEGER, -- Motorcyclist Injured count: counts victims in the collision with statewide vehicle type of C or O and degree of injury of 2, 3, or 4. 0 to N for each collision
    primary_ramp VARCHAR2(2), -- Primary Ramp: NO-NB On Ramp, NF-NB Off Ramp, SO-SB On Ramp, SF-SB Off Ramp, EO-EB On Ramp, EF-EB Off Ramp, WO-WB On Ramp, WF-WB Off Ramp, To, From, Transition, Collector, Connector & blank (see lookup-tables/PRIMARY_RAMP.csv)
    secondary_ramp, -- Same as above (see lookup-tables/PRIMARY_RAMP.csv)
    latitude FLOAT,
    longitude FLOAT,
    PRIMARY KEY(case_id)
    -- all foreign keys
    FOREIGN KEY(day_of_week) REFERENCES day_of_week(id)
    FOREIGN KEY(chp_shift) REFERENCES chp_shift(id)
    FOREIGN KEY(population) REFERENCES population(id)
    FOREIGN KEY(cnty_city_loc) REFERENCES cnty_city_loc(id)
    FOREIGN KEY(special_cond) REFERENCES special_cond(id)
    FOREIGN KEY(beat_type) REFERENCES beat_type(id)
    FOREIGN KEY(chp_beat_type) REFERENCES chp_beat_type(id)
    FOREIGN KEY(direction) REFERENCES direction(id)
    FOREIGN KEY(weather_1) REFERENCES weather(id)
    FOREIGN KEY(weather_2) REFERENCES weather(id)
    FOREIGN KEY(location_type) REFERENCES location_type(id)
    FOREIGN KEY(ramp_intersection) REFERENCES ramp_intersection(id)
    FOREIGN KEY(side_of_hwy) REFERENCES side_of_hwy(id)
    FOREIGN KEY(collision_severity) REFERENCES collision_severity(id)
    FOREIGN KEY(primary_coll_factor) REFERENCES primary_coll_factor(id)
    FOREIGN KEY(pcf_code_of_viol) REFERENCES pcf_code_of_viol(id)
    FOREIGN KEY(pcf_viol_category) REFERENCES pcf_viol_category(id)
    FOREIGN KEY(hit_and_run) REFERENCES hit_and_run(id)
    FOREIGN KEY(type_of_collision) REFERENCES type_of_collision(id)
    FOREIGN KEY(mviw) REFERENCES mviw(id)
    FOREIGN KEY(ped_action) REFERENCES ped_action(id)
    FOREIGN KEY(road_surface) REFERENCES road_surface(id)
    FOREIGN KEY(road_cond_1) REFERENCES road_cond(id)
    FOREIGN KEY(road_cond_2) REFERENCES road_cond(id)
    FOREIGN KEY(lighting) REFERENCES lighting(id)
    FOREIGN KEY(control_device) REFERENCES control_device(id)
    FOREIGN KEY(stwd_vehtype_at_fault) REFERENCES stwd_vehtype_at_fault(id)
    FOREIGN KEY(chp_vehtype_at_fault) REFERENCES chp_vehtype_at_fault(id)
    FOREIGN KEY(primary_ramp) REFERENCES primary_ramp(id)
    FOREIGN KEY(secondary_ramp) REFERENCES secondary_ramp(id)
);
