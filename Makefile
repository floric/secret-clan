build-release:
	docker build -t secret-clan .

run-release:
	docker run --rm -it secret-clan