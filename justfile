export TARGET_DIR := join(join(justfile_directory(), "target"), "build")
export IMPORT_SH := join(justfile_directory(), "import-bicycle-crashes.sh")

# Build the sqlite DB from the SWITRS source files
build source_dir: target_dir
    [[ -L {{TARGET_DIR}}/lookup-tables ]] || ln -s {{justfile_directory()}}/lookup-tables {{TARGET_DIR}}/lookup-tables 
    cd {{source_dir}} && cp CollisionRecords.txt PartyRecords.txt VictimRecords.txt {{TARGET_DIR}}
    cd {{TARGET_DIR}} && /bin/bash {{IMPORT_SH}}

target_dir:
    mkdir -pv {{TARGET_DIR}}

clean:
    rm -r {{TARGET_DIR}}

deploy:
    datasette publish fly switrs.sqlite --app switrs --org radical-bike-lobby --version-note "generated: 2024-05-20; report date: 2024-03-10; first/last processed dated: 2012-08-28/2024-03-07; first/last collision datetime: 2012-01-01T00:15/2024-03-02T15:02"

[private]
[macos]
init-sql-utils:
    brew install sqlite-utils

# Initialize all tools needed for running tests, etc.
init: init-sql-utils
    @echo 'all tools initialized'