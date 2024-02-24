# rusql

A SQLite viewer in the command line interface (CLI) made with Rust.

## Commands

### Implemented:

* **tables**: Returns all table names in the database.
  * **Usage**: `rusql dbpath -tables`

* **records**: Returns all records from a table.
  * **Usage**: `rusql dbpath -records tablename`

### Not Implemented (Yet):

* **info**: Returns information about a specific table.
* **sql**: Allows running SQL queries.

More commands will be added in future updates.
