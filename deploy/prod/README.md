## Deployment
1. Initialize VPS with ssh access via key
2. Open ports
    ```
    ufw reset && \
    ufw allow 22/tcp && \
    ufw allow 80/tcp && \
    ufw allow 443/tcp && \
    ufw reload && \
    ufw enable
    ```
3. Install `podman` on VPS and on development workstation
4. Build core (see `.vscode/tasks.json`)
5. Build frontend (see `.vscode/tasks.json`)
6. Build the dockerfile at `deploy/prod/docker/Dockerfile`
    - See the convenience build script at `deploy/prod/build.sh` for tips for how to do this using a docker registry
7. Modify the stack at `deploy/prod/docker/stack.yml` to use the correct image built in step #5
    - This depends on step 5. If using a docker registry, just set the core container image to the image at in the docker registry in use. If using local builds, the image will need to be pushed manually to the VPS.
8. Set enviornment variables in a `.env` in this directory. See `sample.env` for the list of necessary variables.
9. Deploy the stack with the convenience script in `deploy/prod/deploy.sh`