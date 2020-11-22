build:
	docker build -t secret-clan .

run:
	docker run --rm -it -p 3333:3333 secret-clan