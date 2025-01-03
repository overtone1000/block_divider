## Deployment
1. Initialize VPS with ssh access via key
2. Open ports
    ```
    ufw reset && \
    ufw allow 22/tcp && \
    ufw allow 53/udp && \
    ufw allow 80/tcp && \
    ufw allow 443/tcp && \
    ufw allow 2377/tcp && \
    ufw allow 7946/tcp && \
    ufw allow 7946/udp && \
    ufw allow 4789/udp && \
    ufw route allow in on eth0 out on podman1 to any && \
    ufw reload && \
    ufw enable
    ```
    - 22 is for ssh
    - 53 is for podman
    - route allow for eth0 and podman1 may require `ifconfig` to determine the correct networks that need binding, but this is necessary to allow external traffic to get to containers
    - Are the other ports really necessary?
    
4. Install `podman`, `podman-compose`, `slirp4netns`, and `aardvark-dns` on VPS and on development workstation
5. Build core (see `.vscode/tasks.json`)
    - This assumes the build is happening on a workstation running 64-bit Ubuntu, which is the container architexture. If not, set the build to target the correct architecture.
6. Build frontend (see `.vscode/tasks.json`)
7. Build the dockerfile at `deploy/prod/docker/Dockerfile`
    - See the convenience build script at `deploy/prod/build.sh` for tips for how to do this using a docker registry
8. Modify the stack at `deploy/prod/docker/stack.yml` to use the correct image built in step #5
    - This depends on step 5. If using a docker registry, just set the core container image to the image at in the docker registry in use. If using local builds, the image will need to be pushed manually to the VPS.
9. Set enviornment variables in a `.env` in this directory. See `sample.env` for the list of necessary variables.
10. Deploy the stack with the convenience script in `deploy/prod/deploy.sh`
