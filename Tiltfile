# -*- mode: Python -*-

# Useful commands
#tilt up 
# tilt down --delete-namespaces

ENVIRONMENT = "dev"
os.putenv("DOCKER_DEFAULT_PLATFORM", "linux/amd64")
DOCKER_REGISTRY = "registry.digitalocean.com/abhi-playground-cr/hivenetes/cats"

local_resource('docr', 'doctl registry login', allow_parallel=True)
allow_k8s_contexts('do-lon1-remote-dev')

docker_build(DOCKER_REGISTRY + ":dev", "pkg/")


k8s_yaml(helm('.', name='cats', values='values.yaml'))

k8s_resource("cats", port_forwards="8080")