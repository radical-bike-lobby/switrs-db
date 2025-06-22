CREATE TABLE switrs_corrected_roads (
    case_id VARCHAR2 (19), -- matches the case_id in collisions
    primary_rd VARCHAR2 (50), -- Primary Road
    secondary_rd VARCHAR2 (50), -- Secondary Road
    PRIMARY KEY (case_id)
)
