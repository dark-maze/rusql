# rusql

A SQLite viewer in the command line interface (CLI) made with Rust.

## Commands

### Implemented:

* **tables**: Returns all table names in the database.
  * **Usage**: `rusql dbpath -tables`

  ![Tables Example](https://i.ibb.co/jfG65L5/tables-example.png)

* **records**: Returns all records from a table.
  * **Usage**: `rusql dbpath -records tablename -args`
	* **-a**: `returns all records`
	* **-r num**: `returns a random number of records between 1 and num`
	* **-n num**: `returns the first num records`

  ![Records Example](https://i.ibb.co/ZL6hKqJ/records-example.png)

* **records--no**: Returns the number of records in a specific table.
	* **Usage**: `rusql dbpath -records--no tablename`

### Not Implemented (Yet):

* **info**: Returns information about a specific table.
* **sql**: Allows running SQL queries.

More commands will be added in future updates.

