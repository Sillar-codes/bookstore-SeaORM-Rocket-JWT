sea migrate init -d ./src/migrator
sea migrate generate -d ./src/migrator create_author_table
sea generate entity -o src/entities -u postgres://postgres:sillarpostgre123@localhost:5432/bookstore
