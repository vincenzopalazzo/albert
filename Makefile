CC=cargo
FMT=fmt
NAME=go-lnmetrics
BASE_DIR=/script
OS=linux
ARCH=386

OPTS=

default: fmt
	$(CC) build

fmt:
	$(CC) fmt --all

# The run command need some work with the option passing.
run:
	@echo "Nothings yet"

build:
	$(CC) build --release

check:
	$(CC) test --all -- --show-output --nocapture $(OPTS)
