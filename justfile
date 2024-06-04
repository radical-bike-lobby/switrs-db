export TARGET_DIR := join(join(justfile_directory(), "target"), "build")

# Build the sqlite DB from the SWITRS source files
build source_dir: target_dir
    @ [[ -f "{{source_dir}}/CollisionRecords.txt" ]] || { echo "ERROR: CollisionsRecords.txt not in {{source_dir}}" && exit 1; }
    cd {{justfile_directory()}} && cargo run -r -- -d "{{source_dir}}" -f "{{TARGET_DIR}}/switrs.sqlite"

target_dir:
    mkdir -pv {{TARGET_DIR}}

clean:
    rm -r {{TARGET_DIR}}

deploy:
    datasette publish fly switrs.sqlite --app switrs --org radical-bike-lobby --version-note "generated: 2024-05-20; report date: 2024-03-10; first/last processed dated: 2012-08-28/2024-03-07; first/last collision datetime: 2012-01-01T00:15/2024-03-02T15:02"

[private]
[macos]
init-internal:
    brew install sqlite3

# Initialize all tools needed for running tests, etc.
init: init-internal
    @echo 'all tools initialized'