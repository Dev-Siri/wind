# wind

This Discord bot knows ball.

## Getting Started

Clone the repository.

```sh
$ git clone https://github.com/Dev-Siri/wind
```

You need to have Rust installed on your system. Compile and run the project with `cargo` in release mode:

```sh
$ cargo run --release
```

## Usage

You need to define a config in a `wind.json` file. Example config:

```json
{
  "rules_channel_id": 1460202768714305566,
  "server_id": 1436348966332665969,
  "controller_roles": [1466421366545977365],
  "privileged_roles": [
    1436358588821143634, 1441440233261502484, 1449053795081388042
  ],
  "rules_title": "constitution",
  "rules": "./rules.json",
  "blame": {
    "response": "ts was NOT me 💔",
    "phrases": [
      "must've been the wind",
      "must been the widn",
      "ts was the wind"
    ]
  }
}
```

Note the `blame` option's `phrases` field: Since the bot scans messages in lower case, even putting all capped phrases will make the bot pick up. The `"rules"` field must point to a file that is a JSON array of rules for your server:

```json
["Rule 1", "Rule 2", "Rule 3"]
```

Additionally, of course you would need a `DISCORD_CLIENT_ID` and `DISCORD_TOKEN` from the discord developer portal. Paste them according to the `.env.example` file in the repository root.

That is pretty much it for the config.

## License

This project is [MIT](LICENSE) licensed.
