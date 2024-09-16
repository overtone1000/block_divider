# block_divider
A tool to process vacation request and perform minimal necessary arbitration with fair randomization

## Development vs Production
- The core server runs on 8180. Vite is configured to make this the post root via the `defineConfig` function in `frontend/vite.config.ts`.
    - [ ] Test that this works for both development and production

## Secrets
- E-mail depends on contents of the `core/secrets` folder, which is in .gitignore. This directory needs to be included during production deployment.