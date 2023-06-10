DOCKER = docker
COMPOSE = docker compose


dev-build:
	$(DOCKER) build --rm . --target dev


dev-container:
	$(COMPOSE) run -it --rm --name dev-container php-rs bash
