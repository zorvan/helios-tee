# TEE AI AGENT DEPLOYMENT

## Update Ubuntu

``` shell
sudo apt update
sudo apt upgrade
```

## Install Intel SGX DCAP

``` shell
wget http://archive.ubuntu.com/ubuntu/pool/main/p/protobuf/
libprotobuf17_3.6.1.3-2ubuntu5_amd64.deb

sudo dpkg -i libprotobuf17_3.6.1.3-2ubuntu5_amd64.deb

sudo apt install sgx-aesm-service libsgx-aesm-launch-plugin libsgx-aesm-quote-ex-plugin libsgx-aesm-ecdsa-plugin libsgx-dcap-quote-verify libsgx-dcap-ql libsgx-quote-ex libsgx-dcap-default-qpl-dev

sudo nano /etc/sgx_default_qcnl.conf
```

- edit pccs_url , collateral url, …
- edit certs

## Install gramine

``` shell
sudo curl -fsSLo /etc/apt/keyrings/gramine-keyring-$(lsb_release -sc).gpg https://packages.gramineproject.io/gramine-keyring-$(lsb_release -sc).gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/gramine-keyring-$(lsb_release -sc).gpg] https://packages.gramineproject.io/ $(lsb_release -sc) main" \
| sudo tee /etc/apt/sources.list.d/gramine.list

sudo curl -fsSLo /etc/apt/keyrings/intel-sgx-deb.asc https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/intel-sgx-deb.asc] https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -sc) main" \
| sudo tee /etc/apt/sources.list.d/intel-sgx.list

sudo apt update
sudo apt install gramine
```

## Build the code

``` shell
cargo build
```

## Copy the binary to enclave

``` shell
cp target/debug/helios enclave/gramine/bin/
```

## Update the Trusted Files

This will include the current input toml files:

- config.toml
- prompts.toml

``` shell
cd enclave/gramine/trusted
./update-trusted.sh
```

## Start the Enclave

``` shell
cd enclave
scripts/resume-server.sh
```

## Restart the Enclave

If the binary or trusted files have been changed, the enclave should be re-deployed. The MRENCLAVE will change.

``` shell
cd enclave
scripts/clear-server.sh
```

- Remove the files in gramine/seal folder manually.

``` shell
cd enclave
scripts/resume-server.sh
```

## Stop the Enclave

There may be multiple enclaves deployed. Choose the right PID.

``` shell
ps aux | grep gramine
kill -9 <PID>
```
