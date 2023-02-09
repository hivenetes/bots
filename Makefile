V := $(shell cat pkg/VERSION)
all:
	cd pkg && docker buildx build . -t abigillu/bots --push
deploy:
	helm upgrade --install bots chart  --values chart/values.yaml
deploy-fail:
	helm upgrade --install bots chart --set=failure=true --values chart/values.yaml
undeploy:
	helm uninstall bots
