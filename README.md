# avm [![Build Status](https://travis-ci.org/schultyy/avm.svg)](https://travis-ci.org/schultyy/avm)

## Motivation

If you want to install multiple versions of node.js or Ruby on your machine, there are mostly tools available written in Shell ([nvm](https://github.com/creationix/nvm), [rvm](https://rvm.io/), [rbenv](https://github.com/rbenv/rbenv)).
On the one hand that's nice because it's easy to install on Unix machines, but on the other hand it's not usable on Windows machines and Shell code is not easy to understand. At least for me.
Especially the latter reason is important for me. It is not that easy to find a person who can maintain Shell code and also it's not that easy to figure out where to look when something goes wrong.
Since Rust has become stable, I took the opportunity and began to write a replacement tool. It's called avm as abbreviation for "All version manager".
Right now it manages:

- ✅ node.js
- ✅ Ruby 

The other advantage is since Rust runs on many platforms, there's also the possibility to run avm on machines without Bash e.g. Windows.

## Installation

### Via Cargo Install

If you use Cargo 0.6.0 (ships with Rust 1.5) you can install avm via:

```bash
$ cargo install avm
```

After installation you need to add avm to your `PATH` variable as described in [After Installation](https://github.com/schultyy/avm#After-Installation).

### From Source
It installs from source, for that you need to have git and Rust stable installed.

To install avm on your system, run the following:

```bash
curl https://raw.githubusercontent.com/schultyy/avm/master/install.sh | bash
```

If Rust is not installed yet, visit [https://www.rust-lang.org/downloads.html](https://www.rust-lang.org/downloads.html) and download the version for your operating system.

### Required Packages for installing Ruby

- zlib development packages (Ubuntu: `zlib1g-dev`)
- readline support (Ubuntu: `libreadline6` `libreadline6-dev`)
- C Compiler (Ubuntu: `build-essential`)
- OpenSSL (Ubuntu: `libssl-dev`, RHEL: `openssl-dev`, Mac: `openssl`)

By default, avm uses `/usr/include/openssl` as a lookup path. If you want to use a custom path, for example to link against an OpenSSL version installed via homebrew, export `OPENSSL_INCLUDE_DIR`:

```bash
export OPENSSL_INCLUDE_DIR="$(brew --prefix openssl)"
```

### After installation

After installation finished, you need to make sure that `avm` is in your `PATH`.
For that you need to append the following line to either `~/.zshrc` or `~/.bash_profile`:

```bash
export PATH=~/.avm/:~/.avm/node/bin:~/.avm/ruby/bin:$PATH
```

### Supported platforms

Right now, it is possible to run avm on the major Linux distributions and Mac OS X. There is no support for Windows right now. See [#33](https://github.com/schultyy/avm/issues/33) for details.

### Upgrading to avm 1.x from 0.6 or before

See [https://github.com/schultyy/avm/blob/master/upgrade_notice.md](https://github.com/schultyy/avm/blob/add-upgrade-notice/upgrade_notice.md) for instructions.

## Usage

### node.js

Install a new node version:

```bash
$ avm install node 4.1.2
```

Please note, that right now avm installs precompiled versions of Node.js. There is no supported yet for installing from source.

Use `4.1.2` by default:
```bash
$ avm use node 4.1.2
```
Use your system node version:

```bash
$ avm use node system
```

List all installed versions:

```bash
$ avm node ls
```

Uninstall a version:

```bash
$ avm uninstall node 4.1.2
```

Select the node version based on the `package.json` in the current directory:

```bash
$ avm autoselect node
```
Note that this depends on the `engines` property set in the `package.json`. If `engines`
specifies a node version < 4.x, then it checks for strict equality only. It does not support any
modifiers like `^` or ranges for these versions.

### Ruby

Install a new Ruby version:

```bash
$ avm install ruby 2.3.0
```

Please note, that it installs from source only. It grabs the source tarballs from [ruby-lang.org](https://cache.ruby-lang.org/pub/ruby/). Right now it installs versions only which do not have a `-pxyz` suffix in their url.

Use `2.3.0` by default:
```bash
$ avm use ruby 2.3.0
```

List all installed versions:

```bash
$ avm ruby ls
```

Uninstall a version:

```bash
$ avm uninstall ruby 4.1.2
```
