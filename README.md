<h1 align="center">webout</h1>

<p align="center">Easily stream terminal sessions</p>

[![webout example](https://asciinema.org/a/360725.svg)](https://asciinema.org/a/360725)

## Goals

- Securely (e2e encrypted) stream a terminal session
- No support for full screen capture programs (top, vim or tmux)
- Sessions are not persisted
- CLI client to stream and watch a terminal session
- Web client to watch terminal sessions

## Usage

### Stream terminal session

```
$ webout stream
Webout session started
View online: http://localhost:9000/session/82937d04-be96-459b-ab9d-d813fec738e3
Session id:  82937d04-be96-459b-ab9d-d813fec738e3
```

Once done, exit `ctrl + d` or the `exit` command

```
$ exit
Webout session ended! Bye :)
```

### Watch terminal session

```
$ webout watch <session id> 
```

## Install

For now only instaling from source works :sweat_smile:

```
git clone git@github.com:Bansco/webout.git
cd webout
cargo install --path .
```

## LICENSE

[MIT License](/LICENSE) © [Bansco](https://bansco.tech)
