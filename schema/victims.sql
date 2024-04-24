CREATE TABLE victims (
    case_id VARCHAR2(19), -- Case Id: the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    party_number INTEGER, -- Party Number: 1 to 999
    victim_role CHAR(1), -- Victim Role (see lookup-tables/victim-tables/VICTIM_ROLE.csv)
    victim_sex CHAR(1), -- Victim Sex (see lookup-tables/victim-tables/VICTIM_SEX.csv)
    victim_age INTEGER, -- Victim Age: the age of the victim at the time of the collision. 0 – 125, 998 – Not Stated, 999 – Fatal Fetus
    victim_degree_of_injury CHAR(1), -- Victim Degree of Injury (see lookup-tables/victim-tables/VICTIM_DEGREE_OF_INJURY.csv)
    victim_seating_position CHAR(1), -- Victim Seating Position (see lookup-tables/victim-tables/VICTIM_SEATING_POSITION.csv)
    victim_safety_equip_1 CHAR(1), -- Victim Safety Equipment 1 (see lookup-tables/victim-tables/VICTIM_SAFETY_EQUIPMENT.csv)
    victim_safety_equip_2 CHAR(1), -- Victim Safety Equipment 2, same as Victim Safety Equipment 1 above (eff. Jan 2002) (see lookup-tables/victim-tables/VICTIM_SAFETY_EQUIPMENT.csv)
    victim_ejected CHAR(1), -- Victim Ejected (see lookup-tables/victim-tables/VICTIM_EJECTED.csv)
    PRIMARY KEY(case_id, party_number, victim_seating_position) -- Multiple parties in each case
);
