# TEO (TrustETHreOrigin) PoW chain.
## You can refer what is TEO  and what is TAO go to this URL [TEO Materials and Documents](https://github.com/tao-foundation/teo-material)

----------------------

# rTEO is based on Parity - fast, light, and robust client

##
##  Using Binary download
##  

[Go Binary Release tab](https://github.com/tao-foundation/rteo/releases)

### Prepared for public mainnet releases v1.12.3


----------------------

## Join the chat!

Get in touch with us on Telegram and Online Forum: <br>
 
 * [<img src="https://upload.wikimedia.org/wikipedia/commons/8/82/Telegram_logo.svg" width="32"> Telegram Chat](https://t.me/trustfarmblockchaintalk) <br>

 * [<img src="https://forum.tao.foundation/assets/uploads/system/site-logo.png" width="32"> TAO Forum](https://forum.tao.foundation) <br>
 
## Sites informations

Here is space of discussion for New TAO Universal Blockchain Architecture.

* [TAO Foundation homepage](https://tao.foundation)
* [TAO Foundation Forum](https://forum.tao.foundation)
* [TEO Project  client repository](https://github.com/tao-foundation/rteo)
* [TEO Material includes resources for TEO, you can arrive here](https://github.com/tao-foundation/teo-material)
* [TEO Explorer](https://explorer.tao.foundation) 
* [TEO Explorer2](https://explorer2.tao.foundation) 
* [TEO Wallet](https://wallet.tao.foundation)
* [TEO Bounty Program v0.3](https://github.com/tao-foundation/teo-material/blob/master/documents/TEO-Airdrop-BountyProgram-v0.3.pdf)

Please enjoy this projects,
We are open community  on your contribution and particiaption.

----

## About TEO (Trust Eth-re-Origin) Blockchain

TEO is First phase of TAO Blockchain architecture Project.

You can do with rteo clients.
- create and manage your TAO Accounts and Ethereum accounts;
- manage your TAO Tokens and Coins [TEO, TEOS, TEOA , iTAO] and Future allocated ERC Tokens.
- manage your Ether and any Ethereum tokens;
- create and register your own contract tokens;
- and chain exploreration.


## Build dependencies

**rteo requires Rust toolchain version 1.26.0 to build**

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have rustup, you can install it like this:

- Linux:
	```bash
	$ curl https://sh.rustup.rs -sSf | sh
	```

	rteo also requires `gcc`, `g++`, `libssl-dev`/`openssl`, `libudev-dev` and `pkg-config` packages to be installed.

- OSX:
	```bash
	$ curl https://sh.rustup.rs -sSf | sh
	```

	`clang` is required. It comes with Xcode command line tools or can be installed with homebrew.

- Windows
  Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the rustup installer from
	https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the msvc toolchain:
  ```bash
	$ rustup default stable-x86_64-pc-windows-msvc
  ```

Once you have rustup installed, then you need to install:
* [Perl](https://www.perl.org)
* [Yasm](http://yasm.tortall.net)

Make sure that these binaries are in your `PATH`. After that you should be able to build parity from source.

----

### We will not recommend to use snap. for old linux platforms [eg. <~ Ubuntu 14.04]

----

## Build from source

```bash
# download Parity code
$ git clone https://github.com/tao-foundation/rteo
$ cd rteo

# add rust toolchain v1.26.0
$ rustup toolchain install 1.26.0 

# set CL is should be need for windows10 MSVS builds, in linux no influences
$ set CL="/wd5045"    

$ cargo +1.26.0 build --release
# build in release mode
$ cargo build --release
```

This will produce an executable in the `./target/release` subdirectory.


#### Cleaning the repository will most likely solve the issue, try:

```bash
$ cargo clean
```

## Start rteo

### Manually

To start rteo testnet manually, just run

```bash
$ ./target/release/rteo --chain=teo
```

and rteo will begin syncing the TEO testnet blockchain.


### Using systemd service file

To start Parity as a regular user using systemd init:

1. Copy `./scripts/rteo.service` to your
systemd user directory (usually `~/.config/systemd/user`).
2. To configure Parity, write a `${HOME}/config.toml` config file, see [Configuring rteo - it is most of options are compatible with parity](https://paritytech.github.io/wiki/Configuring-Parity) for details.


--------------------

##  TESTNET Old release binary download link
### [» Download the rteo windows release rteo.win10.zip «](https://github.com/tao-foundation/rteo/raw/master/target/release/rteo.win10.zip) 
### [» Download the rteo Linux( > Ubuntu16.04 )   release rteo.linux.zip «](https://github.com/tao-foundation/rteo/raw/master/target/release/rteo.linux.zip) 
##
##  After download binary and unzip it
  run examples
```
 rteo --chain=teotest
```

