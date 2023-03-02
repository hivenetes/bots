## It "works" on my machine

Have you ever been in a situation where your code worked perfectly fine in your local environment but broke in production? Great, this article will help you prevent being in that dire situation. Furthermore, this tutorial will help you create a streamlined microservice development workflow for Kubernetes.

Developing microservices in a local development environment can present several challenges for developers.

- **Complex environment setup**: Setting up a local development environment that accurately mimics a production environment can be tricky and time-consuming.
- **Difficulty with inter-service communication**: In a microservices architecture, different services must communicate, often over network connections.
- **Managing dependencies and version conflicts**: Microservices often have many dependencies, and keeping track of these dependencies and ensuring that they are all compatible can be challenging.
- **Debugging and testing**: Debugging and testing microservices can be complex and time-consuming, primarily when services depend on each other and require integration testing.

## Tilt + Docker Desktop (Kubernetes)

To overcome some of these challenges, organizations use tools and frameworks such as [Tilt](https://tilt.dev/) and [Docker Desktop(Kubernetes)](https://docs.docker.com/desktop/kubernetes/) that streamline the local development process. These tools help automate many manual steps in setting up a local development environment, making it easier to focus on writing code and testing applications and not worry about the underlining infrastructure, cross-dependency, and software packaging.

> Test it like it's in production

[Tilt: Development Environment as Code](https://tilt.dev/) is a powerful tool that makes it easy to develop, build, and test applications in a Kubernetes environment. It automates the manual steps in setting up a development environment, such as building containers, uploading them to a registry, and deploying them to a cluster.

Some of the standout features include,

- Immersive [web-ui](https://docs.tilt.dev/tutorial/3-tilt-ui.html)
- [Snapshots](https://docs.tilt.dev/snapshots.html)
- Smart [rebuilds](https://docs.tilt.dev/tutorial/5-live-update.html)

[Docker Desktop](https://docs.docker.com/desktop/) provides an easy-to-use development environment for building, shipping, and running dockerized applications. Cross-platform compatibility, intuitive GUI, and [docker-extensions](https://docs.docker.com/desktop/extensions/) make it a must-have on every developer toolkit.

**Fun fact**: Docker has [acquired](https://www.docker.com/blog/welcome-tilt-fixing-the-pains-of-microservice-development-for-kubernetes/) Tilt.

In this tutorial, we will leverage the capabilities of Tilt and Docker Desktop to build a streamlined microservice development workflow.

## Dive into Tilt

We have set up a [sample repository](https://github.com/hivenetes/bots) that demonstrates the *Dev Environment as Code* workflow. Please give it a spin for hands-on exploration. You can also jump straight to [tilt-internals](#tilt-internals) to understand it better.

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) with [Docker Desktop/Kubernetes](https://docs.docker.com/desktop/kubernetes/) enabled
- [Tilt.dev](https://docs.tilt.dev/install.html)

### Getting started with Tilt

We have configured an example repository with Tilt to make it easy for you to try. You can get your microservices dev environment up and running with three commands.

- Clone the `hivenetes/bots` repository:

    ```bash
    git clone https://github.com/hivenetes/bots
    ```

- Copy the `tilt_config.json` from `tilt-resources/` to the root dir:

    ```bash
    # Configurations of k8s contexts, service names, ports, etc. `tilt_config.json` is referenced in the `Tiltfile.`
    cp tilt-resources/local/tilt_config_local.json tilt_config.json
    ```

- Start Tilt:

    ```bash
     tilt up
     # Open the UI on a browser at http://localhost:10350/
     ```

    When you run `tilt up`, Tilt looks for a file named `Tiltfile` in the current directory, which defines your *dev-environment-as-code*. Next, you will be redirected to the [Tilt-UI](https://docs.tilt.dev/tutorial/3-tilt-ui.html) as shown below:

    ![tilt-ui](./assets/tilt-ui.png?raw=true)

    The `bots` application is now accessible via `localhost:8080.`

    ![bots](./assets/bots.png?raw=true)

- Stop Tilt

    ```bash
    # Cleans up all the resources specified in the Tiltfile
    tilt down
    ```

> Wait, what just happened?

Don't worry. Let's now dive into the internals to see what was going on!

### Tilt internals

#### Control flow of Tilt

![tilt-internals](./assets/tilt-internals.jpeg?raw=true)

#### Resource Definitions

A resource is a collection of workloads managed by Tilt. Some of the workloads include,

- Container images
- Kubernetes manifests (YAML)
- A local command

[Here](https://github.com/hivenetes/bots/blob/main/tilt-resources/local/tilt-helm-local-values.yaml) is an example of a resource definition.

#### Tiltfile

A [Titfile](https://github.com/hivenetes/bots/blob/main/Tiltfile) is a configuration file written in [Starlark](https://github.com/bazelbuild/starlark), a dialect of `python.` It consists of all the [resource definitions](#resource-definitions) and how they connect. It's actual code; you can use conditionals, loops, and functions to define your microservice environment.

```python
# Example Tiltfile 

# Read the configuration
if not os.path.exists("./tilt_config.json"):
    fail("Config not found")

config.define_string_list("microservices")

cfg = config.parse()

# Build each microservice image as stated in the tilt_config.json file
for microservice in cfg.get("microservices"):
    docker_build(
        microservice,' pkg'
    )
```

When the Tiltfile is executed, the YAML is packaged as a resource and sent to the `Tilt engine.`

#### Tilt Engine

The `Tilt engine` is a controller that constantly watches the files fed into it. If Tilt detects any changes that might affect the output of the Tiltfile, it re-evaluates the Tiltfile.

The Tilt engine watches critical events like:

- Change the resource's definition in the Tiltfile.
- Manual triggering of a particular resource by the developer

For example, as you edit your code, Tilt will automatically update steps such as building an updated container image, and if any resources contain Kubernetes objects, these end up automatically deploying to your development cluster.

## Microservice development workflow

*Inner development loop* is the iterative process of writing, building, and debugging code that a single developer performs before sharing the code publicly or with their team. Then, there are *outer development loop*, which starts as soon as your code is pushed into version control (or you send a PR for the same). This is where you hand over most of the responsibilities to automated systems and CI/CD pipelines to do the job for you as per a typical GitOps workflow.

Development workflows will vary from organization to organization. For example, the figure below demonstrates the [hivenetes/bots](https://github.com/hivenetes/bots) application workflow. The goal is to take inspiration from it and build your optimized developer workflows.

![local-development-flow](./assets/local-development-flow.png?raw=true)

If we had to break down the flow, it would be

**Inner development loop**

- Setup Kubernetes development environment using Docker Desktop
- Use Tilt to develop and test your application, leveraging smart-rebuilds
- Use Docker Desktop extensions such as [Synk](https://hub.docker.com/r/snyk/snyk-docker-desktop-extension/tags) to scan your container images
- Push to GitHub (create a PR)

**Outer development loop**

- CI pipeline checks
- Build, test, and scan the docker images
- Publish the image to [DigitalOcean container registry](https://docs.digitalocean.com/products/container-registry/) using [Github actions](https://github.com/hivenetes/bots/blob/main/.github/workflows/docker-image.yml)

### Developer thoughts

- Tools such as Tilt provide us the flexibility to configure our environment. For instance, You are not limited to using Docker Desktop as you can combine Tilt with any K8s distribution. Even on a [remote managed Kubernetes.](https://www.digitalocean.com/products/kubernetes)
- Leverage [docker-extensions](https://docs.docker.com/desktop/extensions/) to power-up your development experience.
- Watch out for [docker dev-environments](https://docs.docker.com/desktop/dev-environments/): With the acquisition of Tilt, I believe they will combine the capabilities and provide an out-of-box docker desktop developer experience.
- Other popular inner development loop tools for Kubernetes:
  - [skaffold](https://skaffold.dev/)
  - [telepresence](https://www.telepresence.io/)
  - [devspace](https://www.devspace.sh/)

### Useful Resources

- [Tilt+Docker Desktop example](https://github.com/hivenetes/bots)
- [Remote development using Tilt](https://digitalocean.github.io/k8s-adoption-journey/02-development/tilt-remote/)
- [Container scanning using Snyk](https://docs.snyk.io/scan-containers/image-scanning-library/digitalocean-image-scanning/scan-container-images-from-digitalocean-in-snyk)
