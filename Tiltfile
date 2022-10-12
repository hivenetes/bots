# -*- mode: Python -*-

# Useful commands
#tilt up 
# tilt down --delete-namespaces

ENVIRONMENT = "dev"
os.putenv("DOCKER_DEFAULT_PLATFORM", "linux/amd64")
DOCKER_REGISTRY = "registry.digitalocean.com/bootstrap-doks-cr/hivenetes/bots"

local_resource('docr', 'doctl registry login', allow_parallel=True)
allow_k8s_contexts('do-ams3-abhi-doks-dev')

docker_build(DOCKER_REGISTRY, "pkg/")


k8s_yaml(helm('.', name='bots', values='values.yaml'))

k8s_resource("bots", port_forwards="8080")