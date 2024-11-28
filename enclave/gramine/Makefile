# ARCH LINUX
#ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

# UBUNTU LINUX
ARCH_LIBDIR ?= /lib/x86_64-linux-gnu

# NOTE: MUST BE "RELATIVE PATH" TO TEE BINARY
ENCLAVEDIR ?= "."
SELF_EXE = $(ENCLAVEDIR)/bin/helios

SGX ?= 1
DEBUG ?= 1
SGX_DEV_BUILD ?= 0

.PHONY: all
all: helios.manifest
ifeq ($(SGX),1)
all: helios.manifest.sgx helios.sig
endif

ifeq ($(DEBUG),3)
GRAMINE_LOG_LEVEL = trace
else ifeq ($(DEBUG),2)
GRAMINE_LOG_LEVEL = debug
else ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = warning
else
GRAMINE_LOG_LEVEL = error
endif

ifeq ($(SGX_DEV_BUILD),1)
MANIFEST_FILE = dev_helios.manifest.template
else
MANIFEST_FILE = helios.manifest.template
endif

helios.manifest: $(MANIFEST_FILE)
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dself_exe=$(SELF_EXE) \
		-Denclave_dir=$(ENCLAVEDIR) \
		$< $@

helios.manifest.sgx helios.sig &: helios.manifest
	gramine-sgx-sign -d today \
		--manifest $< \
		--output $<.sgx

ifeq ($(SGX),)
GRAMINE = gramine-direct
else
GRAMINE = gramine-sgx
endif 

.PHONY: start-gramine-server
start-gramine-server: all
	SGX_AESM_ADDR=1 $(GRAMINE) helios \
		ethereum \
		--network mainnet \
		--execution-rpc https://eth-mainnet.g.alchemy.com/v2/Nxm3C5Q5_YETBeGfZJo73Eku7ywQiBW_ \
		--rpc-bind-ip 0.0.0.0 \
		--rpc-port 8545 \
		--data-dir /seal \
		> $(ENCLAVEDIR)/enclave.log 2>&1 &

.PHONY: clean
clean:
	$(RM) -rf $(ENCLAVEDIR)/*.token $(ENCLAVEDIR)/*.sig $(ENCLAVEDIR)/*.manifest.sgx $(ENCLAVEDIR/)*.manifest $(ENCLAVEDIR)/result-* $(ENCLAVEDIR)/*.log

.PHONY: distclean
distclean: clean
	$(RM) -rf target/ Cargo.lock

