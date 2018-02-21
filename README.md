wemux-login
===========

**Work In Progress: This software may kill your kitten!**

Replace your login-shell with `wemux-login` for your wemux-only ssh user.

## Installation

### From Source

- Get this repository:
```sh
$ git clone https://github.com/tryone144/wemux-login
$ cd wemux-login
```

- Build with `cargo`:
```sh
$ cargo build --release
```

- Install into `/usr/local/bin`:
```sh
$ sudo install ./target/release/wemux-login /usr/local/bin/wemux-login
```

## Configuration

- Create a new user and group:
```sh
$ sudo groupadd -g 22423 wemux
$ sudo useradd -u 22423 -g 22423 -s /sbin/nologin -d /usr/local/share/wemux -c 'WEMUX Remote User' -m -N -G wemux wemux
$ sudo passwd wemux
```

- Add `wemux-login` to `/etc/shells` as a valid login-shell:
```sh
$ echo "/usr/local/bin/wemux-login" | sudo tee -a /etc/shells
```

- Add `wemux-login` as the default login-shell for this user:
```sh
$ sudo usermod -s /usr/local/bin/wemux-login wemux
```

## Usage

- Start a `tmux`/`wemux` session on the host:
```sh
$ wemux start
```

- Connect to this remote session using ssh:
```sh
$ ssh -t wemux@remotehost pair
```

### Advanced

- Show a help message:
```sh
$ ssh wemux@remotehost help
```

- List Remote sessions / servers:
```sh
$ ssh wemux@remotehost list
```

- Connect to specific session / server `SESSION`:
```sh
$ ssh wemux@remotehost <mode> SESSION
```

Supported [modes](https://github.com/zolrath/wemux/wiki/Client-Commands) include:
|: Mode |: Description |
| --- | --- |
| `mirror` | No interaction, i.e. read-only viewer. See [wiki](https://github.com/zolrath/wemux/wiki/Client-Commands#wemux-mirror) |
| `pair` | Default tmux interactions. See [wiki](https://github.com/zolrath/wemux/wiki/Client-Commands#wemux-pair) |
| `rogue` | *Not Supported!* Independent session, i.e. free interaction. See [wiki](https://github.com/zolrath/wemux/wiki/Client-Commands#wemux-rogue) |

---

(c) 2018 Bernd Busse, [The MIT License](./LICENSE)

