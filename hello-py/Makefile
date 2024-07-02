.PHONY: build run deploy test

IMAGE=localhost:5001/hello-py:latest
CLUSTER_NAME?=kind

build:
	docker build -t $(IMAGE) .

run:
	docker compose up --build

deploy: build
	docker push $(IMAGE)
	kind load docker-image $(IMAGE) --name $(CLUSTER_NAME)
	kubectl apply -f manifest.yaml

test:
	@echo "Testing the server..."
	@kubectl port-forward service/hello-py-service 8000:8000 & \
	PID=$$!; \
	sleep 5; \
	curl -s http://localhost:8000 || echo "Failed to connect to the server"; \
	kill $$PID

clean:
	kubectl delete -f manifest.yaml || true
