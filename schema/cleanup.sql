-- clean up non-berkeley stuff from ccrs
DELETE FROM ccrs_crashes
WHERE
    city_name <> 'Berkeley';

-- delete any parties without collisions
DELETE FROM ccrs_parties
WHERE
    party_id IN (
        SELECT
            party_id
        FROM
            ccrs_parties
            LEFT OUTER JOIN ccrs_crashes ON ccrs_parties.collision_id = ccrs_crashes.collision_id
        WHERE
            ccrs_crashes.collision_id IS NULL
    );

-- delete any ccrs_injured_witness_passengers without associated collisions
DELETE FROM ccrs_injured_witness_passengers
WHERE
    injured_wit_pass_id IN (
        SELECT
            injured_wit_pass_id
        FROM
            ccrs_injured_witness_passengers
            LEFT OUTER JOIN ccrs_crashes ON ccrs_injured_witness_passengers.collision_id = ccrs_crashes.collision_id
        WHERE
            ccrs_crashes.collision_id IS NULL
    );

-- Compact the DB
VACUUM;
