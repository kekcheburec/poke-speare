# Poke-speare

[![Crates.io](https://img.shields.io/crates/v/poke-speare.svg)](https://crates.io/crates/poke-speare)
[![CI](https://github.com/MarcoIeni/poke-speare/workflows/General/badge.svg)](https://github.com/MarcoIeni/poke-speare/actions)
[![Coverage Status](https://coveralls.io/repos/github/MarcoIeni/poke-speare/badge.svg?branch=master)](https://coveralls.io/github/MarcoIeni/poke-speare?branch=master)
[![docker](https://img.shields.io/docker/cloud/build/marcoieni/poke-speare.svg)](https://hub.docker.com/r/marcoieni/poke-speare/builds)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![API docs](http://validator.swagger.io/validator?url=https://raw.githubusercontent.com/MarcoIeni/poke-speare/gh-pages/swagger.yaml)](https://marcoieni.github.io/poke-speare/)

REST API that, given a Pokémon name, returns its description in Shakespeare's
style.

Pokemon description is taken from [PokéAPI](https://pokeapi.co/) and it is converted
by using [Shakespeare translator](https://funtranslations.com/api/shakespeare).

## Usage

You can use `poke-speare` both as a web server and as a rust library

### Web server

```sh
$ curl http://localhost:5000/pokemon/charizard

{
    "name": "charizard",
    "description": "Charizard flies 'round the sky in search of powerful opponents."
}

```

### Library

```rust
let shakespeare_api_token = None;
let pokemon_descr = poke_speare::get_description("charizard", shakespeare_api_token).await;
println!("description: {}", pokemon_descr);
```

## Install

### Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install poke-speare`

### Docker

```
$ docker pull marcoieni/poke-speare
```

## Run

### Cargo

Make sure that `~/.cargo/bin` is in your `PATH`.

```
$ poke-speare
```

### Docker
```
$ docker run -p 5000:5000 marcoieni/poke-speare
```

## Settings

You can set environment variable in order to change settings. For example:
```
$ RUST_LOG=debug POKE_SPEARE_PORT=5001 poke-speare
```

In the following there are all environment variables with their default values:
- `RUST_LOG`: `error`. Other possible values: `info`
- `POKE_SPEARE_HOST`: `"127.0.0.1"`
- `POKE_SPEARE_PORT`: `5000`
- `POKE_SPEARE_SHAKESPEARE_TOKEN`: `""`.
  [Shakespeare translator](https://funtranslations.com/api/shakespeare)
  has a rate limit of 60 API calls a day with distribution of 5 calls an hour.
  Set the environment variable `POKE_SPEARE_SHAKESPEARE_TOKEN` if you have a
  FunTranslations Api Secret.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

- github actions are taken from [LukeMathWalker](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3).
