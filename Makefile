
# Bins: src/bin/*.rs -> target/debug/*
DEBUG_BINS := $(patsubst src/bin/%.rs, target/debug/%, $(wildcard src/bin/*.rs))

# Bins: src/bin/*.rs -> target/release/*
RELEASE_BINS := $(patsubst src/bin/%.rs, target/release/%, $(wildcard src/bin/*.rs))

all: $(DEBUG_BINS) $(DEBUG_LIBS)

release: $(RELEASE_BINS)

debug: $(DEBUG_BINS)

$(DEBUG_BINS):
	cargo build
	echo "Built debug binaries at target/debug"

# Rule to build release binaries
$(RELEASE_BINS):
	cargo build --release
	echo "Built release binaries at target/release"

# Test target: runs tests for each binary if corresponding script exists
test: $(DEBUG_BINS)
	for bin in clean combine kmer metrics pe quality_filter randomize regex repair replace sample shuffle straighten trim validate sort progress mutate normalize convert inspect; do \
		if [ -e ./tests/fasten_$$bin.sh ]; then \
			bash ./tests/fasten_$$bin.sh; \
		fi; \
	done

# Clean target: removes the target directories if they exist
clean:
	if [[ -e ./target/debug ]]; then \
		rm -rf ./target/debug; \
	fi
		if [[ -e ./target/release ]]; then \
		rm -rf ./target/release; \
	fi
