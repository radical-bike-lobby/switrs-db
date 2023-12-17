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

[private]
[macos]
init-sql-utils:
    brew install sqlite-utils

# Initialize all tools needed for running tests, etc.
init: init-sql-utils
    @echo 'all tools initialized'