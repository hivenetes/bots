#########################################################################
#
#   Tiltfile logic
#
#########################################################################


# Import required functions from Tilt extensions

load(
    "ext://namespace", 
    "namespace_create",
    "namespace_inject"
)
# Set default platform
os.putenv("DOCKER_DEFAULT_PLATFORM", "linux/amd64")

# Import settings from tilt_config.json
if not os.path.exists("./tilt_config.json"):
    fail(
        """
        # ===================================================================== #
        # Tilt config file not found in current directory!                      #
        # Please copy a template from tilt-resources dir.                       #
        #                                                                       #
        # E.g.:                                                                 #
        #    cp tilt-resources/local/tilt_config_local.json tilt_config.json    #
        # ====================================================================  #
        """
    )

config.define_string_list("allowed_contexts")
config.define_string("default_registry")
config.define_string("environment")
config.define_string_list("microservices")
config.define_string("namespace")
config.define_string_list("port_forwards")
cfg = config.parse()

# Allow default K8S context as stated in the tilt_config.json file
allow_k8s_contexts(cfg.get("allowed_contexts"))

# Set default registry as stated in the tilt_config.json file
if cfg.get("default_registry") != "":
    default_registry(cfg.get("default_registry"))

# Build each microservice image as stated in the tilt_config.json file
for microservice in cfg.get("microservices"):
    docker_build(
        microservice,'pkg'
    )

# Deploy each microservice image as stated in the tilt_config.json file
for microservice in cfg.get("microservices"):
    k8s_yaml(helm('.', microservice, values='tilt-resources/local/tilt-helm-local-values.yaml'))

# Port forwards as stated in the tilt_config.json file
for port_forward in cfg.get("port_forwards"):
    mapping = port_forward.split(":")
    if (len(mapping) != 2):
        fail(
            """
            # =================================================== #
            # Invalid port forward specified in tilt_config.json! #
            # Should be <resource>:<port_number>.                 #
            #                                                     #
            # E.g.: bots:8080                                     #
            # =================================================== #
            """
        )
    service = mapping[0]
    port = mapping[1]
    k8s_resource(service, port_forwards=port)
