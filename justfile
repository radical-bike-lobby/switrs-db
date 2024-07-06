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
      eval $(sqlite3 "{{TARGET_DIR}}/{{DB_FILE}}" -line 'select * from version_view;' | sed 's/ *//g') && \
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