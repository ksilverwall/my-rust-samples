SOLC=solc
SOLC_OPTIONS=--optimize --bin --abi --overwrite

CONTRACTS_DIR=contracts
BUILD_DIR=build/contracts

CONTRACTS=MyContract1.sol MyContract2.sol


all: build

$(BUILD_DIR)/%.bin: $(CONTRACTS_DIR)/%.sol
	$(SOLC) $(SOLC_OPTIONS) -o $(BUILD_DIR) $<

build: $(BUILD_DIR)/SocialMedia.bin

clean:
	rm -rf $(BUILD_DIR)
