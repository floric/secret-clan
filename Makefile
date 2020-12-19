watch-fe:
	cd frontend && npm run watch

watch-be:
	cd backend && LOG_LEVEL=debug cargo watch -x 'run' -w 'src'

build:
	DOCKER_BUILDKIT=1 docker build -t secret_clan .

run:
	docker run --rm -it -p 3333:3333 secret_clan