CREATE TABLE parties (
    case_id VARCHAR2(19), -- Case Id: the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    party_number INTEGER, -- Party Number: 1 to 999
    party_type CHAR(1), -- Party Type (see lookup-tables/party-tables/PARTY_TYPE.csv)
    at_fault CHAR(1), -- At Fault: indicates whether the party was at fault in the collision, Y
    party_sex CHAR(1),-- Party Sex: the code of the sex of the party (see lookup-tables/party-tables/PARTY_SEX.csv)
    party_age INTEGER, -- Party Age: the age of the party at the time of the collision, 0 to 100+ (0 & blank = Not Stated)
    party_sobriety CHAR(1), -- Party Sobriety (see lookup-tables/party-tables/PARTY_SOBRIETY.csv)
    party_drug_physical CHAR(1), -- Party Drug Physical (see lookup-tables/party-tables/PARTY_DRUG_PHYSICAL.csv)
    dir_of_travel CHAR(1), -- Direction Of Travel (see lookup-tables/party-tables/DIRECTION_OF_TRAVEL.csv)
    party_safety_equip_1 CHAR(1), -- Party Safety Equipment 1 (see lookup-tables/party-tables/PARTY_SAFETY_EQUIPMENT.csv)
    party_safety_equip_2 CHAR(1), -- Party Safety Equipment 2: same as Party Safety Equipment 1 above (see lookup-tables/party-tables/PARTY_SAFETY_EQUIPMENT.csv)
    finan_respons CHAR(1), -- Financial Responsibility (see lookup-tables/party-tables/FINANCIAL_RESPONSIBILITY.csv)
    sp_info_1 CHAR(1), -- Special Information 1 (see lookup-tables/party-tables/SPECIAL_INFORMATION_1.csv)
    sp_info_2 CHAR(1), -- Special Information 2 (see lookup-tables/party-tables/SPECIAL_INFORMATION_2.csv)
    sp_info_3 CHAR(1), -- Special Information 3 (see lookup-tables/party-tables/SPECIAL_INFORMATION_3.csv)
    oaf_violation_code CHAR(2), -- OAF Violation Code (see lookup-tables/party-tables/OAF_VIOLATION_CODE.csv)
    oaf_viol_cat CHAR(2), -- OAF Violation Category (see, lookup-tables/party-tables/OAF_VIOLATION_CATEGORY.csv)
    oaf_viol_section INTEGER, -- OAF Violation Section
    oaf_violation_suffix CHAR(1), -- OAF Violation Suffix: Blank may appear if no suffix.
    oaf_1 CHAR(1), -- Other Associated Factor 1 (see lookup-tables/party-tables/OTHER_ASSOCIATED_FACTOR.csv)
    oaf_2 CHAR(1), -- Other Associated Factor 2: same as OAF 1 above (see lookup-tables/party-tables/OTHER_ASSOCIATED_FACTOR.csv)
    party_number_killed INTEGER, -- Party Number Killed: counts victims in the party with degree of injury of 1. 0 to N for each party
    party_number_injured INTEGER, -- Party Number Injured: counts victims in the party with degree of injury of 2, 3, or 4. 0 to N for each party
    move_pre_acc CHAR(1), -- Movement Preceding Collision (see lookup-tables/party-tables/MOVEMENT_PRECEDING_COLLISION.csv)
    vehicle_year INTEGER, -- Vehicle Year: the model year of the party's vehicle, 9999 or blank = not stated
    vehicle_make VARCHAR2(50), -- Vehicle Make	Varchar2(50)	the full description of the make of the party's vehicle	
    stwd_vehicle_type CHAR(1), -- Statewide Vehicle Type, TODO: is this STWD_VEHTYPE_AT_FAULT?
    chp_veh_type_towing CHAR(2), -- CHP Vehicle Type Towing (see lookup-tables/party-tables/CHP_VEHICLE_TYPE_TOWING.csv)
    chp_veh_type_towed CHAR(2), -- CHP Vehicle Type Towed (see lookup-tables/party-tables/CHP_VEHICLE_TYPE_TOWED.csv)
    race CHAR(1), -- Party Race (see lookup-tables/party-tables/PARTY_RACE.csv)
    inattention, -- Undocumented, unused?
    special_info_f, -- Undocumented, unused?
    special_info_g, -- Undocumented, unused?
    PRIMARY KEY(case_id, party_number) -- Multiple parties in each case
    -- add foreign keys
    FOREIGN KEY(case_id) REFERENCES collisions(case_id)
    FOREIGN KEY(party_type) REFERENCES party_type(id)
    FOREIGN KEY(party_sex) REFERENCES party_sex(id)
    FOREIGN KEY(party_sobriety) REFERENCES party_sobriety(id)
    FOREIGN KEY(party_drug_physical) REFERENCES party_drug_physical(id)
    FOREIGN KEY(dir_of_travel) REFERENCES dir_of_travel(id)
    FOREIGN KEY(party_safety_equip_1) REFERENCES party_safety_equip(id)
    FOREIGN KEY(party_safety_equip_2) REFERENCES party_safety_equip(id)
    FOREIGN KEY(finan_respons) REFERENCES finan_respons(id)
    FOREIGN KEY(sp_info_1) REFERENCES sp_info_1(id)
    FOREIGN KEY(sp_info_2) REFERENCES sp_info_2(id)
    FOREIGN KEY(sp_info_3) REFERENCES sp_info_3(id)
    FOREIGN KEY(oaf_violation_code) REFERENCES oaf_violation_code(id)
    FOREIGN KEY(oaf_viol_cat) REFERENCES oaf_viol_cat(id)
    FOREIGN KEY(oaf_1) REFERENCES oaf(id)
    FOREIGN KEY(oaf_2) REFERENCES oaf(id)
    FOREIGN KEY(move_pre_acc) REFERENCES move_pre_acc(id)
    FOREIGN KEY(chp_veh_type_towing) REFERENCES chp_veh_type_towing(id)
    FOREIGN KEY(chp_veh_type_towed) REFERENCES chp_veh_type_towed(id)
    FOREIGN KEY(race) REFERENCES race(id)
);
CREATE INDEX idx_parties_case_id ON parties(case_id);

CREATE VIEW parties_view (
    case_id,
    party_number,
    at_fault,
    party_age,
    oaf_viol_section,
    oaf_violation_suffix,
    party_number_killed,
    party_number_injured,
    vehicle_year,
    -- joined table names
    party_type_name,
    party_sex_name,
    party_sobriety_name,
    party_drug_physical_name,
    dir_of_travel_name,
    party_safety_equip_1_name,
    party_safety_equip_2_name,
    finan_respons_name,
    sp_info_1_name,
    sp_info_2_name,
    sp_info_3_name,
    oaf_violation_code_name,
    oaf_viol_cat_name,
    oaf_1_name,
    oaf_2_name,
    move_pre_acc_name,
    chp_veh_type_towing_name,
    chp_veh_type_towed_name,
    race_name
) AS SELECT 
    p.case_id,
    p.party_number,
    p.at_fault,
    p.party_age,
    p.oaf_viol_section,
    p.oaf_violation_suffix,
    p.party_number_killed,
    p.party_number_injured,
    p.vehicle_year,
    -- joined table names
    party_type.name,
    party_sex.name,
    party_sobriety.name,
    party_drug_physical.name,
    dir_of_travel.name,
    party_safety_equip_1.name,
    party_safety_equip_2.name,
    finan_respons.name,
    sp_info_1.name,
    sp_info_2.name,
    sp_info_3.name,
    oaf_violation_code.name,
    oaf_viol_cat.name,
    oaf_1.name,
    oaf_2.name,
    move_pre_acc.name,
    chp_veh_type_towing.name,
    chp_veh_type_towed.name,
    race.name
FROM parties AS p
-- join all the foreign key tables
LEFT JOIN party_type ON p.party_type = party_type.id
LEFT JOIN party_sex ON p.party_sex = party_sex.id
LEFT JOIN party_sobriety ON p.party_sobriety = party_sobriety.id
LEFT JOIN party_drug_physical ON p.party_drug_physical = party_drug_physical.id
LEFT JOIN dir_of_travel ON p.dir_of_travel = dir_of_travel.id
LEFT JOIN party_safety_equip party_safety_equip_1 ON p.party_safety_equip_1 = party_safety_equip_1.id
LEFT JOIN party_safety_equip party_safety_equip_2 ON p.party_safety_equip_2 = party_safety_equip_2.id
LEFT JOIN finan_respons ON p.finan_respons = finan_respons.id
LEFT JOIN sp_info_1 ON p.sp_info_1 = sp_info_1.id
LEFT JOIN sp_info_2 ON p.sp_info_2 = sp_info_2.id
LEFT JOIN sp_info_3 ON p.sp_info_3 = sp_info_3.id
LEFT JOIN oaf_violation_code ON p.oaf_violation_code = oaf_violation_code.id
LEFT JOIN oaf_viol_cat ON p.oaf_viol_cat = oaf_viol_cat.id
LEFT JOIN oaf oaf_1 ON p.oaf_1 = oaf_1.id
LEFT JOIN oaf oaf_2 ON p.oaf_2 = oaf_2.id
LEFT JOIN move_pre_acc ON p.move_pre_acc = move_pre_acc.id
LEFT JOIN chp_veh_type_towing ON p.chp_veh_type_towing = chp_veh_type_towing.id
LEFT JOIN chp_veh_type_towed ON p.chp_veh_type_towed = chp_veh_type_towed.id
LEFT JOIN race ON p.race = race.id
;