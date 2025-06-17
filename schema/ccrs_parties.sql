CREATE TABLE IF NOT EXISTS ccrs_parties (
    party_id INTEGER, -- the unique identifier of the party involved in the crash
    collision_id INTEGER, -- the unique identifier of the crash involved in the crash
    party_number INTEGER, -- Number identifier of the parties in the crash -- 1 to 999
    party_type VARCHAR2 (50), -- DRIVER, PEDESTRIAN, PARKED VEHICLE, BICYCLIST, OTHER, OPERATOR
    is_at_fault CHAR(1), -- indicates whether the party was at fault in the crash -- True/False
    is_on_duty_emergency_vehicle CHAR(1), -- Indicates whether the party is an on-duty emergency vehicle or not in the crash -- True/False
    is_hit_and_run CHAR(1), -- Indicates whether the party is hit and run or not --  True/False
    airbag_code CHAR(1), -- B-UNKNOWN, L-AIR BAG DEPLOYED, M-AIR BAG NOT DEPLOYED, N-OTHER, P-NOT REQUIRED
    airbag_description VARCHAR2 (50), -- See above
    safety_equipment_code CHAR(1), -- A - None in Vehicle, B - Unknown, C - Lap Belt Used, D - Lap Belt Not Used, E - Shoulder Harness Used, F - Shoulder Harness Not Used, G - Lap/Shoulder Harness Used, H - Lap/Shoulder Harness Not Used, J - Passive Restraint Used, K - Passive Restraint Not Used, L - Air Bag Deployed, M - Air Bag Not Deployed, N - Other, P - Not Required, Q - Child Restraint in Vehicle Used, R - Child Restraint in Vehicle Not Used, S - Child Restraint in Vehicle, Use Unknown, T - Child  Restraint in Vehicle, Improper Use, U - No Child Restraint in Vehicle, V - Driver, Motorcycle Helmet Not Used, W - Driver, Motorcycle Helmet Used, X - Passenger, Motorcycle Helmet Not Used, Y - Passenger, Motorcycle Helmet Used, -  or blank - Not Stated
    safety_equipment_description VARCHAR2 (50), -- see above
    special_information VARCHAR2 (50), -- Value A Indicates that the crash involved a vehicle known to be, or believed to be, transporting a hazardous material. Value E indicates that the crash involved a motor vehicle in-transport passing a stopped school bus with its red signal lamps in operation, pursuant to CVC Section 22112, or reacting to, pursuant to CVC Section 22454viii. Other values are related to cell phone in use. -- A - Hazardous Materials, B - Cell Phone in Use (4/1/01), C - Cell Phone Not  in, Use (4/1/01), D - No Cell Phone/Unknown (4/1/01), 1 - Cell Phone Handheld in Use, 2 - Cell Phone Handsfree in Use, 3 - Cell Phone Not in Use, 4 - Cell Phone Use Unknown, E - School Bus Related (1/1/02)
    other_associate_factor VARCHAR2 (50), -- A factor that contributed to the crash but was not the primary cause of the crash. Note: if the report has more than one special conditions, the values are separated by a slash (/) --
    inattention_direction_of_travel VARCHAR2 (50),
    street_or_highway_name VARCHAR2 (75),
    speed_limit SMALLINT,
    movement_prec_coll_code CHAR(1),
    movement_prec_coll_description VARCHAR2 (50),
    sobriety_drug_physical_code1 CHAR(1),
    sobriety_drug_physical_description1 VARCHAR2 (50),
    sobriety_drug_physical_code2 CHAR(1),
    sobriety_drug_physical_description2 VARCHAR2 (50),
    gender_code CHAR(1),
    gender_description VARCHAR2 (50),
    stated_age VARCHAR2 (10),
    driver_license_class VARCHAR2 (2),
    driver_license_state_code CHAR(2),
    race_code CHAR(1),
    race_desc VARCHAR2 (50),
    v1_year INTEGER,
    v1_make VARCHAR2 (50),
    v1_model VARCHAR2 (50),
    v1_color VARCHAR2 (50),
    v1_is_vehicle_towed TEXT, -- True/False
    lane TEXT,
    thru_lanes TEXT,
    total_lanes TEXT,
    is_dre_conducted TEXT, -- True/False
    PRIMARY KEY (party_id, collision_id)
)
