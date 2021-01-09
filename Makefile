watch-fe:
	cd frontend && npm run watch

watch-be:
	cd backend && LOG_LEVEL=debug cargo watch -x 'run' -w 'src' -c -i target-check

test-be:
	cd backend && cargo tarpaulin --out Html

bench:
	cd backend && cargo bench

build-fe:
	cd frontend && npm run build

build-be:
	cd backend && cargo build

build:
	DOCKER_BUILDKIT=1 docker build -t secret_clan .

run:
	docker run --rm -it -p 3333:3333 secret_clan