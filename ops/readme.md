## Database Migrations

### Init Database

Use `./db_init.sh` to create and initialize (populating it with all the changes that exist) the database as a Docker container.

### Apply Changes

Newer database changes introduced during development are applied as follows:

1. Create the change using `./db_add_change.sh {change-name}`.<br/>
   Ex: `./db_add_change.sh create_table_user_accounts`

2. Populate the generated file.<br/>

3. Apply the change using `./db_apply_changes.sh`.
