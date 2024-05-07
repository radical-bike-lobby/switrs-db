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