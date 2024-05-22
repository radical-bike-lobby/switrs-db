CREATE TABLE normalized_roads (
    case_id VARCHAR2(19), -- matches the case_id in collisions
    primary_rd VARCHAR2(50), -- Primary Road
    primary_rd_address VARCHAR2(10), -- address if one exists on the road where the collision occured
    primary_rd_block VARCHAR2(10), -- block (i.e. address at the corner) on the road where the collision occured
    primary_rd_direction VARCHAR2(10), -- direction of travel when the collision occured
    secondary_rd VARCHAR2(50), -- Secondary Road 
    secondary_rd_address VARCHAR2(10), -- address if one exists on the road where the collision occured
    secondary_rd_block VARCHAR2(10), -- block (i.e. address at the corner) on the road where the collision occured
    seconardy_rd_direction VARCHAR2(10), -- direction of travel when the collision occured
    PRIMARY KEY(case_id)
)