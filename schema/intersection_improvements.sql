-- table of intersection improvements from across the city
CREATE TABLE intersection_improvements (
    id INTEGER PRIMARY KEY,
    primary_rd VARCHAR2(50),   -- primary road where the infrastructure was installed
    secondary_rd VARCHAR2(50), -- secondary or cross road of the intersection
    date_completed TEXT,       -- date, YYYY-MM-DD, when the infrastructure was completed
    improvement_type INTEGER,  -- type of intersection installed
    FOREIGN KEY(improvement_type) REFERENCES improvement_types(id)
);

CREATE VIEW intersection_improvements_view (
    id,
    primary_rd,
    secondary_rd,
    date_completed,
    improvement_type,
    -- joined table names
    improvement_name
) AS SELECT 
    i.id,
    i.primary_rd,
    i.secondary_rd,
    i.date_completed,
    i.improvement_type,
    -- joined table names
    improvement_types.name
FROM intersection_improvements AS i
-- join all the foreign key tables
LEFT JOIN improvement_types ON i.improvement_type = improvement_types.id
;

CREATE VIEW intersection_performance_view (
    id,
    primary_rd,
    secondary_rd,
    date_completed,
    improvement_type,
    -- joined table names
    improvement_name,
    case_id,
    party_count,
    before_improvement,
    collision_datetime,
    pedestrian_accident,
    bicycle_accident,
    number_killed,
    number_injured,
    count_ped_killed,
    count_ped_injured,
    count_bicyclist_killed,
    count_bicyclist_injured
) AS SELECT 
    i.id,
    i.primary_rd,
    i.secondary_rd,
    i.date_completed,
    i.improvement_type,
    -- joined table names
    improvement_types.name,
    c.case_id,
    c.party_count,
    c.collision_datetime < i.date_completed,
    c.collision_datetime,
    c.pedestrian_accident,
    c.bicycle_accident,
    c.number_killed,
    c.number_injured,
    c.count_ped_killed,
    c.count_ped_injured,
    c.count_bicyclist_killed,
    c.count_bicyclist_injured
FROM intersection_improvements AS i
-- join all the foreign key tables
LEFT JOIN improvement_types ON i.improvement_type = improvement_types.id
LEFT JOIN collisions_view as c ON (c.corrected_primary_rd = i.primary_rd AND c.corrected_secondary_rd = i.secondary_rd)
                               OR (c.corrected_secondary_rd = i.primary_rd AND c.corrected_primary_rd = i.secondary_rd)
;

