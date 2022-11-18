# Haze

> A simple command line tool to save/test Minecraft Bedrock worlds

## Why?

Haze takes on the idea of keeping a single source of truth for your worlds, similar to tools like [Regolith](https://github.com/Bedrock-OSS/regolith). This mechanism of moving worlds back and forth between Minecraft and your project folder allows you to work on mutliple worlds with ease. In addition, it makes it intuitive to integrate  source control tools such as Git without weird workarounds.

![Diagram](.github/diagram.png)

## Installation

Open PowerShell on Windows and run:

```powershell
irm https://sedge.arexon.me/haze | iex
```
## Usage

Haze requires your project to include a config file that follows the [Project Config Standard](https://github.com/Bedrock-OSS/project-config-standard).

This also means that you can integrate Haze into projects that use [Regolith](https://github.com/Bedrock-OSS/regolith) or [bridge.'s Dash compiler](https://github.com/bridge-core/deno-dash-compiler) seamlessly.

### Setting up worlds

Here is the required config:

```jsonc
{
  "packs": {
    "worldTemplate": "./WT" // could also be "./packs/WT" or "./worlds"
  }
}
```

### Running commands

Run `haze --help` or reference the docs below:

| Command | Description |
| ------- | ----------- |
| `haze test <NAME>` | Copy a world from the project's worlds folder into "minecraftWorlds" |
| `haze save <NAME>` | Copy a world from "minecraftWorlds" into the project's worlds folder |

Note: `<NAME>` is the world folder name.

## License

Haze is under the MIT license.
