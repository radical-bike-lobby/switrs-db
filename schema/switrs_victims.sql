CREATE TABLE switrs_victims (
    case_id VARCHAR2 (19), -- Case Id: the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    party_number INTEGER, -- Party Number: 1 to 999
    victim_role CHAR(1), -- Victim Role (see lookup-tables/victim-tables/VICTIM_ROLE.csv)
    victim_sex CHAR(1), -- Victim Sex (see lookup-tables/victim-tables/VICTIM_SEX.csv)
    victim_age INTEGER, -- Victim Age: the age of the victim at the time of the collision. 0 – 125, 998 – Not Stated, 999 – Fatal Fetus
    victim_degree_of_injury CHAR(1), -- Victim Degree of Injury (see lookup-tables/victim-tables/VICTIM_DEGREE_OF_INJURY.csv)
    victim_seating_position CHAR(1), -- Victim Seating Position (see lookup-tables/victim-tables/VICTIM_SEATING_POSITION.csv)
    victim_safety_equip_1 CHAR(1), -- Victim Safety Equipment 1 (see lookup-tables/victim-tables/VICTIM_SAFETY_EQUIPMENT.csv)
    victim_safety_equip_2 CHAR(1), -- Victim Safety Equipment 2, same as Victim Safety Equipment 1 above (eff. Jan 2002) (see lookup-tables/victim-tables/VICTIM_SAFETY_EQUIPMENT.csv)
    victim_ejected CHAR(1), -- Victim Ejected (see lookup-tables/victim-tables/VICTIM_EJECTED.csv)
    local_report_number, -- Local Police Report Number
    -- add foreign keys
    FOREIGN KEY (case_id, party_number) REFERENCES switrs_parties (case_id, party_number) FOREIGN KEY (victim_role) REFERENCES victim_role (id) FOREIGN KEY (victim_sex) REFERENCES victim_sex (id) FOREIGN KEY (victim_degree_of_injury) REFERENCES victim_degree_of_injury (id) FOREIGN KEY (victim_seating_position) REFERENCES victim_seating_position (id) FOREIGN KEY (victim_safety_equip_1) REFERENCES victim_safety_equip (id) FOREIGN KEY (victim_safety_equip_2) REFERENCES victim_safety_equip (id) FOREIGN KEY (victim_ejected) REFERENCES victim_ejected (id)
);

CREATE INDEX idx_victims_case_id ON switrs_victims (case_id);

CREATE INDEX idx_victims_case_id_party_number ON switrs_victims (case_id, party_number);

CREATE VIEW victims_view (
    case_id,
    party_number,
    victim_age,
    -- joined table names
    victim_role_name,
    victim_sex_name,
    victim_degree_of_injury_name,
    victim_seating_position_name,
    victim_safety_equip_1_name,
    victim_safety_equip_2_name,
    victim_ejected_name
) AS
SELECT
    v.case_id,
    v.party_number,
    v.victim_age,
    -- joined table names
    victim_role.name,
    victim_sex.name,
    victim_degree_of_injury.name,
    victim_seating_position.name,
    victim_safety_equip_1.name,
    victim_safety_equip_2.name,
    victim_ejected.name
FROM
    switrs_victims AS v
    -- join all the foreign key tables
    LEFT JOIN victim_role ON v.victim_role = victim_role.id
    LEFT JOIN victim_sex ON v.victim_sex = victim_sex.id
    LEFT JOIN victim_degree_of_injury ON v.victim_degree_of_injury = victim_degree_of_injury.id
    LEFT JOIN victim_seating_position ON v.victim_seating_position = victim_seating_position.id
    LEFT JOIN victim_safety_equip victim_safety_equip_1 ON v.victim_safety_equip_1 = victim_safety_equip_1.id
    LEFT JOIN victim_safety_equip victim_safety_equip_2 ON v.victim_safety_equip_2 = victim_safety_equip_2.id
    LEFT JOIN victim_ejected ON v.victim_ejected = victim_ejected.id;
