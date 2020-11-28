watch-fe:
	cd frontend && npm run watch

watch-be:
	cd backend && cargo watch -x 'run' -w 'src'

build:
	docker build -t secret-clan .

run:
	docker run --rm -it -p 3333:3333 secret-clan