<h1 align="center">webout</h1>

<p align="center">Easily stream terminal sessions</p>

[![webout example](https://asciinema.org/a/360725.svg)](https://asciinema.org/a/360725)

## Usage

### Stream terminal session

```
$ webout stream
Webout session started
View online: http://localhost:9000/session/82937d04-be96-459b-ab9d-d813fec738e3
Session id:  82937d04-be96-459b-ab9d-d813fec738e3
```

Once done, use the `exit` command.

```
$ exit
Webout session ended! Bye :)
```

### Watch terminal session

```
$ webout watch <session id> 
```

## Install

### MacOS

```
$ brew tap bansco/webout
$ brew install webout
```

### Cargo

```
$ cargo install webout
```

### Manually

Download the latest [released binary](https://github.com/Bansco/webout/releases)
and add executable permissions:

```bash
# Linux example:
$ wget -O webout "https://github.com/Bansco/webout/releases/download/v0.1.0/webout-x86-64-linux"
$ chmod +x webout
```

## Goals

- Stream a terminal session
- No support for full screen capture programs (top, vim or tmux)
- Sessions are not persisted
- CLI client to stream and watch a terminal session
- Web client to watch terminal sessions

## LICENSE

[MIT License](/LICENSE) © [Bansco](https://bansco.tech)
