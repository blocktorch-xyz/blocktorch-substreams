include .env
export

START_BLOCK ?= 18000000
STOP_BLOCK ?= 0

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_filter_transactions -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: sink
sink: build
	substreams-sink-sql setup "psql://$(SINK_DB_NAME):$(SINK_DB_PASS)@$(SINK_DB_URL)?sslmode=disable" ./sink/substreams.dev.yaml
	substreams-sink-sql run "psql://$(SINK_DB_NAME):$(SINK_DB_PASS)@$(SINK_DB_URL)?sslmode=disable" ./sink/substreams.dev.yaml --on-module-hash-mistmatch=warn

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_filter_transactions -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
