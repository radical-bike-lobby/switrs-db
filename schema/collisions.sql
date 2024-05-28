CREATE TABLE collisions (
    case_id VARCHAR2(19), -- Case Id: the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    accident_year INTEGER, -- Collision Year: the year when the collision occurred
    proc_date TEXT, -- Process Date: (YYYYMMDD)
    juris INTEGER, -- Jurisdiction: Four numerics assigned by DOJ
    collision_date TEXT, -- Collision Date: the date when the collision occurred (YYYYMMDD)	
    collision_time TEXT, -- Collision Time: the time when the collision occurred (24 hour time)	Data may appear with no leading zero(s).
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
    FOREIGN KEY(chp_vehtype_at_fault) REFERENCES chp_vehtype(id)
    FOREIGN KEY(primary_ramp) REFERENCES ramp(id)
    FOREIGN KEY(secondary_ramp) REFERENCES ramp(id)
);

CREATE VIEW collisions_view (
    case_id,
    address,
    proc_date,
    collision_datetime,
    primary_rd,
    secondary_rd,
    state_route,
    pedestrian_accident,
    bicycle_accident,
    count_severe_inj,
    count_visible_inj,
    count_complaint_pain,
    count_ped_killed,
    count_ped_injured,
    count_bicyclist_killed,
    count_bicyclist_injured,
    latitude,
    longitude,
    -- joined table names
    day_name,
    chp_shift_name,
    population_name,
    city_name,
    special_cond_name,
    beat_type_name,
    chp_beat_type_name,
    direction_name,
    weather_1_name,
    weather_2_name,
    location_type_name,
    ramp_intersection_name,
    side_of_hwy_name,
    collision_severity_name,
    primary_coll_factor_name,
    pcf_code_of_viol_name,
    pcf_viol_category_name,
    hit_and_run_name,
    type_of_collision_name,
    mviw_name,
    ped_action_name,
    road_surface_name,
    road_cond_1_name,
    road_cond_2_name,
    lighting_name,
    control_device_name,
    stwd_vehtype_at_fault_name,
    chp_vehtype_at_fault_name,
    primary_ramp_name,
    secondary_ramp_name,
    corrected_primary_rd,
    corrected_secondary_rd
) AS SELECT 
    c.case_id,
    printf("%s %s%s, CA", c.primary_rd, iif(c.secondary_rd IS NOT NULL, printf("and %s ", c.secondary_rd), ""), cnty_city_loc.city),
    printf("%s-%s-%s", substr(proc_date,1,4), substr(proc_date,5,2), substr(proc_date,7,2)),
    printf("%s-%s-%sT%s:%s", substr(collision_date,1,4), substr(collision_date,5,2), substr(collision_date,7,2), substr(collision_time,1,2), substr(collision_time,3,2)),
    c.primary_rd,
    c.secondary_rd,
    c.state_route,
    c.pedestrian_accident,
    c.bicycle_accident,
    c.count_severe_inj,
    c.count_visible_inj,
    c.count_complaint_pain,
    c.count_ped_killed,
    c.count_ped_injured,
    c.count_bicyclist_killed,
    c.count_bicyclist_injured,
    latitude,
    longitude * (-1),
    -- joined table names
    day_of_week.name,
    chp_shift.name,
    population.name,
    cnty_city_loc.city,
    special_cond.name,
    beat_type.name,
    chp_beat_type.name,
    direction.name,
    weather_1.name,
    weather_2.name,
    location_type.name,
    ramp_intersection.name,
    side_of_hwy.name,
    collision_severity.name,
    primary_coll_factor.name,
    pcf_code_of_viol.name,
    pcf_viol_category.name,
    hit_and_run.name,
    type_of_collision.name,
    mviw.name,
    ped_action.name,
    road_surface.name,
    road_cond_1.name,
    road_cond_2.name,
    lighting.name,
    control_device.name,
    stwd_vehtype_at_fault.name,
    chp_vehtype.name,
    primary_ramp.name,
    secondary_ramp.name,
    corrected_roads.primary_rd,
    corrected_roads.secondary_rd
FROM collisions AS c
-- join all the foreign key tables
LEFT JOIN day_of_week ON c.day_of_week = day_of_week.id
LEFT JOIN chp_shift ON c.chp_shift = chp_shift.id
LEFT JOIN population ON c.population = population.id
LEFT JOIN cnty_city_loc ON c.cnty_city_loc = cnty_city_loc.id
LEFT JOIN special_cond ON c.special_cond = special_cond.id
LEFT JOIN beat_type ON c.beat_type = beat_type.id
LEFT JOIN chp_beat_type ON c.chp_beat_type = chp_beat_type.id
LEFT JOIN direction ON c.direction = direction.id
LEFT JOIN weather AS weather_1 ON c.weather_1 = weather_1.id
LEFT JOIN weather AS weather_2 ON c.weather_2 = weather_2.id
LEFT JOIN location_type ON c.location_type = location_type.id
LEFT JOIN ramp_intersection ON c.ramp_intersection = ramp_intersection.id
LEFT JOIN side_of_hwy ON c.side_of_hwy = side_of_hwy.id
LEFT JOIN collision_severity ON c.collision_severity = collision_severity.id
LEFT JOIN primary_coll_factor ON c.primary_coll_factor = primary_coll_factor.id
LEFT JOIN pcf_code_of_viol ON c.pcf_code_of_viol = pcf_code_of_viol.id
LEFT JOIN pcf_viol_category ON c.pcf_viol_category = pcf_viol_category.id
LEFT JOIN hit_and_run ON c.hit_and_run = hit_and_run.id
LEFT JOIN type_of_collision ON c.type_of_collision = type_of_collision.id
LEFT JOIN mviw ON c.mviw = mviw.id
LEFT JOIN ped_action ON c.ped_action = ped_action.id
LEFT JOIN road_surface ON c.road_surface = road_surface.id
LEFT JOIN road_cond AS road_cond_1 ON c.road_cond_1 = road_cond_1.id
LEFT JOIN road_cond AS road_cond_2 ON c.road_cond_2 = road_cond_2.id
LEFT JOIN lighting ON c.lighting = lighting.id
LEFT JOIN control_device ON c.control_device = control_device.id
LEFT JOIN stwd_vehtype_at_fault ON c.stwd_vehtype_at_fault = stwd_vehtype_at_fault.id
LEFT JOIN chp_vehtype ON c.chp_vehtype_at_fault = chp_vehtype.id
LEFT JOIN ramp primary_ramp ON c.primary_ramp = primary_ramp.id
LEFT JOIN ramp secondary_ramp ON c.secondary_ramp = secondary_ramp.id
LEFT JOIN corrected_roads ON c.case_id = corrected_roads.case_id
WHERE 
c.cnty_city_loc IN ("0102", "0103") -- see lookup-tables/CNTY_CITY_LOC.csv
;

-- view of the data included in this DB
CREATE VIEW version_view (
    first_proc_date,
    last_proc_date,
    first_collision_datetime,
    last_collision_datetime
) AS SELECT
    (SELECT c.proc_date FROM collisions_view c ORDER BY c.proc_date LIMIT 1),
    (SELECT c.proc_date FROM collisions_view c ORDER BY c.proc_date DESC LIMIT 1),
    (SELECT c.collision_datetime FROM collisions_view c ORDER BY c.collision_datetime LIMIT 1),
    (SELECT c.collision_datetime FROM collisions_view c ORDER BY c.collision_datetime DESC LIMIT 1)
;