watch-fe:
	cd frontend && npm run watch

watch-be:
	cd backend && LOG_LEVEL=info cargo watch -x 'run --target-dir watch-target' -w 'src' -c -i 'src/model/proto'

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

protoc-fe:
	protoc --plugin=./frontend/node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=env=browser --ts_proto_opt=useOptionals=true --ts_proto_opt=oneof=unions --ts_proto_out=./frontend/src/types/proto --proto_path=./schema message.proto
