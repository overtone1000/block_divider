# block_divider
A tool to process vacation request and perform minimal necessary arbitration with fair randomization

## Database
- The development environment needs a database. Use the stack in `./deploy/dev` to quickly start a local database.
- See the [Diesel Guide](https://diesel.rs/) for how to initialize the development environment for Diesel, which includes a CLI tool. This repo assumes postgres is the database used, so make sure to install the correct backend library (`libpq` for postgres) as it says in the instructions.
    - On Ubuntu, this required `sudo apt install libpq-dev` not just `sudo apt install libpq`
- Secrets are handled with environment files and are in `./core/.env` which is excluded from the git repo for security. This file will need to be populated.

## Local Dependencies
The core is dependent on some local external rust libraries. See `core/Cargo.toml` which shows the relative path where those libraries need to be placed.

## Development vs Production
- The core server runs on port 8180. Vite is configured to make this the post root via the `defineConfig` function in `frontend/vite.config.ts`.
    - [ ] Test that this works for both development and production

## Deploy
Use the stack in `./deploy/prod` to deploy to the production server. Secrets here are excluded from the git repo for security.