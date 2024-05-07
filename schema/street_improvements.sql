-- table of street improvements from across the city, like bike lanes that stretch over a range of intersections
CREATE TABLE street_improvements (
    id INTEGER PRIMARY KEY,
    primary_rd VARCHAR2(50),   -- primary road where the infrastructure was installed
    start_intersection VARCHAR2(50), -- beginning, first cross street, of the improvements
    end_intersection VARCHAR2(50), -- end, last cross street, of the improvements
    date_completed TEXT,       -- date, YYYY-MM-DD, when the infrastructure was completed
    improvement_type INTEGER,  -- type of intersection installed
    ca_bike_lane_type INTEGER, -- California classification of bike infrastructure, 0 for none
    FOREIGN KEY(improvement_type) REFERENCES improvement_types(id)
    FOREIGN KEY(ca_bike_lane_type) REFERENCES ca_bike_lane_types(id)
);

CREATE VIEW street_improvements_view (
    id,
    primary_rd,
    start_intersection,
    end_intersection,
    date_completed,
    improvement_type,
    ca_bike_lane_type,
    -- joined table names
    improvement_name,
    ca_bike_lane_name
) AS SELECT 
    s.id,
    s.primary_rd,
    s.start_intersection,
    s.end_intersection,
    s.date_completed,
    s.improvement_type,
    s.ca_bike_lane_type,
    -- joined table names
    improvement_types.name,
    ca_bike_lane_types.name
FROM street_improvements AS s
-- join all the foreign key tables
LEFT JOIN improvement_types ON s.improvement_type = improvement_types.id
LEFT JOIN ca_bike_lane_types ON s.ca_bike_lane_type = ca_bike_lane_types.id
;