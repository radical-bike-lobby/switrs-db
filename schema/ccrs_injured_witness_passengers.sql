CREATE TABLE IF NOT EXISTS ccrs_injured_witness_passengers (
    collision_id INTEGER,
    injured_wit_pass_id INTEGER,
    stated_age VARCHAR2 (10),
    gender CHAR(1),
    gender_desc VARCHAR2 (50),
    race CHAR(1),
    race_desc VARCHAR2 (50),
    is_witness_only TEXT, -- True/False
    is_passenger_only TEXT, -- True/False
    extent_of_injury_code VARCHAR2 (50),
    injured_person_type VARCHAR2 (50),
    seat_position VARCHAR2 (50),
    seat_position_other VARCHAR2 (50),
    air_bag_code CHAR(1),
    air_bag_description VARCHAR2 (50),
    safety_equipment_code CHAR(1),
    safety_equipment_description VARCHAR2 (50),
    ejected VARCHAR2 (50),
    is_vovc_notified TEXT, -- True/False
    party_number SMALLINT,
    seat_position_description VARCHAR2 (50),
    PRIMARY KEY (collision_id, injured_wit_pass_id)
)
