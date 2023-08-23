SOLC=solc
SOLC_OPTIONS=--optimize --bin --abi --overwrite

CONTRACTS_DIR=contracts
BUILD_DIR=build/contracts

CONTRACTS=MyContract1.sol MyContract2.sol


all: build build-image

$(BUILD_DIR)/%.bin: $(CONTRACTS_DIR)/%.sol
	$(SOLC) $(SOLC_OPTIONS) -o $(BUILD_DIR) $<

build: $(BUILD_DIR)/SocialMedia.bin

build-image: $(call find src -name "*.rs")
	docker build . -f ./docker/images/server/Dockerfile --build-arg SRC_DIR=./server -t local-talk

clean:
	rm -rf $(BUILD_DIR)
