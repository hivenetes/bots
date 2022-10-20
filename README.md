# bots

![bots](bots.gif)


An application that displays pictures of `bots` around the world. 


## Prerequisite
* [docker-desktop](https://www.docker.com/products/docker-desktop/)
* [tilt.dev](https://docs.tilt.dev/install.html)
* [doctl](https://docs.digitalocean.com/reference/doctl/)

## Local Development using Tilt
- Copy the `tilt_config.json` from `tilt-resources/` to root dir
    ```bash
    cp tilt-resources/local/tilt_config_local.json tilt_config.json
    ```
- Start tilt: 
    ```bash
     tilt up
     # Open the UI on a browser at http://localhost:10350/
     ```       
- Once the development is done, stop tilt
    ```bash
    # Cleans up all the resources specified in the Tiltfile
    tilt down
    ```

## Overview of Development workflow

![df](development-flow.png)