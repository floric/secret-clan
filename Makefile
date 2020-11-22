watch-fe:
	cd frontend && yarn watch

watch-be:
	cd backend && cargo watch -x 'run'

build:
	docker build -t secret-clan .

run:
	docker run --rm -it -p 3333:3333 secret-clan