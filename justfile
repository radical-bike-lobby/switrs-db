export TARGET_DIR := join(join(justfile_directory(), "target"), "build")
export DB_FILE := "switrs.sqlite"

# Build the sqlite DB from the SWITRS source files
build source_dir: target_dir
    @ [[ -f "{{source_dir}}/CollisionRecords.txt" ]] || { echo "ERROR: CollisionsRecords.txt not in {{source_dir}}" && exit 1; }
    cd {{justfile_directory()}} && cargo run -r -- -d "{{source_dir}}" -f "{{TARGET_DIR}}/{{DB_FILE}}"

target_dir:
    mkdir -pv {{TARGET_DIR}}

clean:
    rm -r {{TARGET_DIR}}

deploy source_dir: (build source_dir)
    @date=$(date -Idate) && \
      eval $(sqlite3 "{{TARGET_DIR}}/{{DB_FILE}}" -line 'select * from switrs_version_view;' | sed 's/ *//g') && \
      version_str="generated: $date; first/last processed dates: $first_proc_date/$last_proc_date; first/last collision datetime: $first_collision_datetime/$last_collision_datetime" && \
      echo "Deploying with version, $version_str" && \
      datasette publish fly "{{TARGET_DIR}}/{{DB_FILE}}" --app switrs --org radical-bike-lobby --version-note "$version_str"

[private]
[macos]
init-internal:
    brew install sqlite3

# Initialize all tools needed for running tests, etc.
init: init-internal
    @echo 'all tools initialized'


## Helper scripts for generating reports
version-report:
    sqlite3 {{TARGET_DIR}}/{{DB_FILE}} -line 'select * from switrs_version_view;'

victim-cohort-report:
    sqlite3 "{{TARGET_DIR}}/{{DB_FILE}}" -line \
      'select \
          SUM(CASE WHEN victim_age < 12 THEN 1 ELSE 0 END) AS "Under 12", \
          SUM(CASE WHEN victim_age BETWEEN 12 AND 18 THEN 1 ELSE 0 END) AS "12-18", \
          SUM(CASE WHEN victim_age BETWEEN 19 AND 25 THEN 1 ELSE 0 END) AS "18-25", \
          SUM(CASE WHEN victim_age BETWEEN 26 AND 40 THEN 1 ELSE 0 END) AS "26-40", \
          SUM(CASE WHEN victim_age BETWEEN 41 AND 60 THEN 1 ELSE 0 END) AS "41-60", \
          SUM(CASE WHEN victim_age BETWEEN 61 AND 75 THEN 1 ELSE 0 END) AS "61-75", \
          SUM(CASE WHEN victim_age > 75 THEN 1 ELSE 0 END) AS "Over 75" \
       from switrs_victims_view \
       where victim_role_name = "Pedestrian";'

##
# 1,Driver
# 3,Pedestrian
# 4,Bicyclist

party-cohort-report:
    sqlite3 "{{TARGET_DIR}}/{{DB_FILE}}" -line \
      'select \
          SUM(CASE WHEN party_age < 12 THEN 1 ELSE 0 END) AS "Under 12", \
          SUM(CASE WHEN party_age BETWEEN 12 AND 18 THEN 1 ELSE 0 END) AS "12-18", \
          SUM(CASE WHEN party_age BETWEEN 19 AND 25 THEN 1 ELSE 0 END) AS "18-25", \
          SUM(CASE WHEN party_age BETWEEN 26 AND 40 THEN 1 ELSE 0 END) AS "26-40", \
          SUM(CASE WHEN party_age BETWEEN 41 AND 60 THEN 1 ELSE 0 END) AS "41-60", \
          SUM(CASE WHEN party_age BETWEEN 61 AND 75 THEN 1 ELSE 0 END) AS "61-75", \
          SUM(CASE WHEN party_age > 75 THEN 1 ELSE 0 END) AS "Over 75" \
       from switrs_parties_view \
       where party_type = "1";'

# id,name
# 1,Driver (including Hit and Run)
# 2,Pedestrian
# 3,Parked Vehicle
# 4,Bicyclist
# 5,Other
# 6,Undefined in RawData_template
# -,Not Stated
