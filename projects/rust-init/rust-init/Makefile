.PHONY: tag push
default: push

IMAGE=rust-init
DEPS=Dockerfile init $(wildcard etc/*) $(wildcard etc/init.d/*) $(wildcard src/*)

HASH?=$(shell git ls-tree HEAD -- ../$(notdir $(CURDIR)) | awk '{print $$3}')

tag: $(DEPS)
	docker build --no-cache -t mato/$(IMAGE):$(HASH) .

push: tag
	docker pull mato/$(IMAGE):$(HASH) || \
	docker push mato/$(IMAGE):$(HASH)
