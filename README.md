<div align="center">
<img alt="XMODITS Logo" src="icon.png">     

# XMODITS

# A tool to bulk extract samples from various tracker modules with ease.
</div>
<div align="center">

![GitHub all releases](https://img.shields.io/github/downloads/B0ney/xmodits/total)
![GitHub](https://img.shields.io/github/license/B0ney/xmodits)
[![GitHub release (with filter)](https://img.shields.io/github/v/release/B0ney/xmodits)](https://github.com/B0ney/xmodits/releases)
![Repo size](https://img.shields.io/github/repo-size/B0ney/xmodits)
[!["Release RSS Feed"](https://img.shields.io/badge/rss-releases-ffa500?logo=rss)](https://github.com/B0ney/xmodits/releases.atom)

</div>

<div align="center">
<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>
</div>

## Download
You can download builds for xmodits [here](https://github.com/B0ney/xmodits/releases/latest).

If you wish to build from source, go to [building](#building).

If you prefer to use a minimal version of this tool, a command line version of xmodits can be found [here](https://github.com/B0ney/xmodits-cli). Additionally, the command line version has more supported architectures. 

## Supported Tracker Formats
| Extension | Format | 
| - | - |
| ``.it`` | Impulse Tracker |
| ``.xm`` | Extended Module | 
| ``.s3m`` | Scream Tracker 3 |
| ``.mod`` | Amiga Pro Tracker |
| ``.mptm`` | ModPlug Tracker module |
| ``.umx`` | Unreal Music Package (Containing above) |

# Supported Exports
| Extension | Format |
|-|-|
|``.wav``| Microsoft Wave|
|``.aiff``| Audio Interchange File Format |
|``.its``| Impulse Tracker 2 sample |
|``.s3i``| Scream Tracker 3 Instrument |
|``.8svx``| 8-Bit Sampled Voice |
|``.raw``| Headerless pcm |

## Features
* A robust naming system for extracted samples (see [Sample Naming](#sample-naming)).
* Can export samples to less common audio formats used by music trackers: ``.its``, ``.s3i`` & ``.8svx``
* Can show information about a module.
* A sample previewer.
* Multithreaded ripping* for better efficiency.
* Cute animated fox to make ripping less tedious.
<!-- * Resuming -->
<!-- * History -->

\* xmodits will only use threads if it is ripping from a directory.

## Screenshots
![XMODITS initial screen](./assets/screenshots/home.png)
<details>
<summary>Click to show more</summary>

![Selecting tracker modules](./assets/screenshots/selection.png)
![Ripping samples from 33 tracker modules](./assets/screenshots/ripping.png)
![XMODITS has finished ripping with no error. Yay! ](./assets/screenshots/ripping_done.png)


Click [here](./assets/screenshots/README.md) for different themes

</details>

## How to Use
1) Open application
2) Drag and drop a module, and or a folder full of modules.
3) (Optional) Press "Open" to set the destination folder. Your downloads folder is the default.
5) Press "Start"
6) Press "Show Folder" to see the results.

(**Windows Only**) If you just want to simply extract samples, you can also drag and drop a module(s) onto the binary. XMODITS will (by default) place the samples in a self contained folder in your ```~/Downloads``` folder.

### Sample Naming
Configure how ripped samples are named.

|Parameter| Description|
|--|--|
| ``Index Only`` | Samples will only be named with an index. |
| ``Preserve Index`` | Sample index will match how it is represented internally. |
| ``Prefix Samples`` | Samples will be prefixed with the tracker's filename. |
| ``Upper Case`` | Samples will be named in upper case.|
| ``Lower Case`` | Samples will be named in lower case.|
| ``Prefer Filename`` | Some samples have an additional filename. If present, xmodits will name samples with that. |
| ``Index Padding`` | Set the minimum amount of digits an index must have. Indexes will be padded with zeros to match the minimum amount of digits*. Set to 1 to disable padding.|

\* xmodits may override this value to ensure that samples are named consistently (unless it is set to 1).

### Ripping Configuration

|Parameter| Description|
|--|--|
| ``Self Contained`` | XMODITS will put samples in a self contained folder.|
| ``Export Format`` | Samples can be saved to the following formats: [ ``wav``, ``aiff``, ``8svx``, ``its``, ``s3i``, ``raw`` ]|
| ``Folder Scan Depth`` | Limit how far a folder can be traversed. |
| ``Worker Threads`` | Set how many threads can be used to rip samples in parallel.|

<!-- ### Filters
Only rip from files/folders if they satisfy a set of defined conditions.

|Attribute| description |
|-|-|
|size||
|contains||
|starts with||
|ends with||
|regex||
|extension| | -->

<!-- ### Previewing Samples
![Previewing samples from "UNATCO_Music.umx"](./assets/screenshots/sample_preview.png) -->

### Saving Configuration
Any changes made to the configuration **must be saved manually**.<br>The configuration file can be located at:

|OS|Path|
|-|-|
|Windows|``%appdata%\xmodits\config.toml``|
|Linux|``~/.config/xmodits/config.toml``|
|MacOs|``~/Library/Application Support/xmodits/config.toml``|

### Keyboard Shortcuts
|Shortcut| Action|
|-|-|
|<kbd>delete</kbd>| Clears the selected entries|
|<kbd>shift</kbd> + <kbd>delete</kbd>| Clears the entries|
|<kbd>ctrl</kbd>/<kbd>⌘</kbd> + <kbd>S</kbd>| Save Configuration|

### Command Line Arguments
|short|long|Description|
|-|-|-|
|`-h`| `--help` | Prints help information |
|`-V`| `--version` | prints app version |
|`-i`| `--info` | Prints build information |
|`-m`| `--manual`| Prints manual |

## Building
Requirements:
* Rust compiler: https://www.rust-lang.org/tools/install
* Minimum rust version: `1.75`
* A decent computer if you don't want to wait a while:
  * *At least* 4 cores, 
  * *At least* 8GB of RAM

(Linux) Dependencies:
* `fontconfig`
* `libasound2-dev` (If building with `audio` feature)

Clone the source code (latest):
```shell
git clone https://github.com/B0ney/xmodits
```

Alternatively, you can download different versions from: https://github.com/B0ney/xmodits/tags


Compile:
```shell
cd xmodits

cargo build --release
```

**NOTE**: Older versions will have slightly different build instructions.
Additionally, versions below [v0.9.8](https://github.com/B0ney/xmodits/releases/tag/0.9.8) are CLI only.

### Build Flags

|Feature | Description | Enabled by Default? |
|-|-|-|
|``audio``|Add audio playback, used to preview samples.|**yes**|
|``built``|Includes metadata about the binary and the environment it was compiled in. Useful for bug reporting.|**yes**|
|``jemalloc``| (*nix only) Use the [jemalloc](https://jemalloc.net/) memory allocator. Used to mitigate memory fragmentation which can improve memory footprint.  |no|
|``wgpu``| Enables hardware acceleration (`DX12`/`Vulkan`/`Metal`). |no|
|``iced_gif``| Include animated GIF | **YES**|
|``manual``| Bundle a simplified readme in the application | **yes**|


For example, to compile XMODITS with ``jemalloc`` and ``wgpu``:
```shell
cargo build --release --features="jemalloc","wgpu"
```

Compile XMODITS with default features disabled:
```shell
cargo build --release --no-default-features
```

### Build flags used in official releases:
|Target|Features|
|-|-|
|Windows|`audio`, `built`, `iced_gif`,`manual`|
|MacOS|`audio`, `built`, `iced_gif`,`manual`, `wgpu`|
|Linux|`audio`, `built`, `iced_gif`,`manual`, `jemalloc`|


## License
Unless noted otherwise, XMODITS is licensed under the GPLv3, see [license](LICENSE) for more detail.


## Other xmodits projects

|Program| License|Description|
|--|--|--|
|[XMODITS-CLI](https://github.com/B0ney/xmodits-cli) | LGPLv3 | xmodits cli app|
|[XMODITS-PY](https://github.com/B0ney/xmodits-py) | LGPLv3 | xmodits Python library. [PyPi link](pypi.org/project/xmodits-py/) <br> (Mainly used for [DawVert](https://github.com/SatyrDiamond/DawVert))<br> |
|[XMODITS-LIB](https://github.com/B0ney/xmodits-lib) | MPLv2 | xmodits core library|

## Special Thanks
- The GUI was made with [Iced](https://github.com/iced-rs/iced)
- [0x192](https://github.com/0x192) (and contributors) for their [Universal Android Debloat tool](https://github.com/0x192/universal-android-debloater/). I've learned a lot of gui stuff from that project.
- [SatyrDiamond](https://github.com/SatyrDiamond)'s [DawVert](https://github.com/SatyrDiamond/DawVert), A program to convert different daw project files to other formats. 
- The animated fox gif was obtained from: https://github.com/tonybaloney/vscode-pets
- [Halloy](https://github.com/squidowl/halloy)