all: run

beforeBuild: clean
	./scripts/before_build.sh

build: beforeBuild
	./scripts/build.sh

afterBuild: build
	./scripts/after_build.sh

run: afterBuild
	./scripts/run.sh

clean:
	./scripts/clean.sh