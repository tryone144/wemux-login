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
# install ./target/release/wemux-login /usr/local/bin/wemux-login
```

## Configuration

- Create a new user and group:
```sh
# groupadd -g 22423 wemux
# useradd -u 22423 -g 22423 -s /sbin/nologin -d /usr/local/share/wemux -c 'WEMUX Remote User' -m -N -G wemux wemux
# passwd wemux
```

- Add `wemux-login` to `/etc/shells` as a valid login-shell:
```sh
# echo "/usr/local/bin/wemux-login" >> /etc/shells
```

- Add `wemux-login` as the default login-shell for this user:
```sh
# usermod -s /usr/local/bin/wemux-login wemux
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

---

(c) 2018 Bernd Busse, [The MIT License](./LICENSE)

