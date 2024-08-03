# bevy_mod_lockdown

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=flat-square)
![Crates.io Version](https://img.shields.io/crates/v/bevy_mod_lockdown.svg?style=flat-square)
![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=flat-square)

bevy_mod_lockdown is a library to reduce the attack surface your application offers.

> [!WARNING]  
> This plugin is in a very early stage of development.  
> Expect major changes to the contained features and to the api of existing features.

## Description

> [!NOTE]  
> This repository is open for additional features and the expansion of existing features to other platforms.  
> It is currently limited to the features I use in my personal project and the platforms I have available.

bevy_mod_lockdown offers ways to reduce the attack surface your application offers.
It can for example disable network usage or limit file system access to specific directories.
This is mainly intended for applications that load third-party code but can also help in other situations.

This plugin also offers some `Resources` to check the status of the "lockdown".
This allows the application to e.g. terminate if it was unable to disable network access.
See this [chapter](#status-resources) for more information.

> [!WARNING]  
> This does only reduce the attack surface and does not make running unknown code safe.  
> It is recommended to ensure the application is already safe without the usage of bevy_mod_lockdown.

### Status resources

The resource `LockdownAdjustments` can be used to check the status of the adjustments.
The features each add a field to the resource that matches the feature name.
These fields are enums with variants describing the current status.

## Features

To be as modular as possible this library has most of its functionality gated behind separate features.  
None of those features are enable by default.
You need to enable some features for this library to be useful.

You can see the availability of features and their inclusion in full_speed in this table.

> [!NOTE]
> By default only the feature `feature_warning` is enabled which logs a warning when no other feature is enabled.

### Platform independent features

The features described here are independent of the used platform.

#### Feature:  `egui`

Adds a convenience method to display the `LockdownAdjustments` with egui.
See the `egui` example for usage.

Run the example with `cargo run --example egui --features="egui"`.
Include other features to see something beside the warning.

### Platform dependent features

Feature support on different platform:

|               |       Linux        |      Windows       |
| :------------ | :----------------: | :----------------: |
| `filesystem`  | :white_check_mark: |  :grey_question:   |
| `network`     |  :grey_question:   |  :grey_question:   |
| `privilege`   | :white_check_mark: | :white_check_mark: |
| `system_call` |  :grey_question:   |  :grey_question:   |

__Legend:__  
:grey_question: = To be evaluated  
:white_square_button: = Not yet implemented  
:white_check_mark: = Implemented  
:negative_squared_cross_mark: = Feature not supported on this platform

> [!NOTE]  
> Features enabled on a platform they do not support do nothing[^1].

[^1]: Beside a log entry in some cases.

#### Feature:  `filesystem`

Restricts access to the filesystem, blocking access to all not explicitly allowed paths.

Use the resource `AllowedFilesystemAccess` to adjust which path are allowed to be read and or written.
By default this already includes the paths needed for bevy on the current platform.

This happens in `Startup` which means that full access to the file system is still possible in `PreStartup` and partly in `Startup`.

> [!NOTE]  
> You probably need to add at least the paths for your configuration and save game for your game to work as expected.

Currently only implemented on linux.
Where it uses landlock and requires a kernel with support for it.

#### Feature:  `network`

Not yet implemented for any platform.

#### Feature:  `privilege`

Reduces the privilege granted to the application.

* __Linux:__ Adjust r/e/s uid and gid. Currently not checking capabilities.
* __Windows:__ Uses AdjustTokenPrivileges to disable all privileges.

This happens in `PostStartup` which means that elevated privileges are still available in `PreStartup`, `Startup` and partly in `PostStartup`.

#### Feature:  `system_call`

Not yet implemented for any platform.

## Installation

Include the library in your project by adding it to your `Cargo.toml`.

```toml
[dependencies]
bevy = "0.14.0"
bevy_mod_lockdown = "0.3.0"
```

Then add the `LockdownPlugin` to your app like shown below.

```rust
use bevy::prelude::*;
use bevy_mod_lockdown::LockdownPlugin;

fn main(){
  App::new()
    .add_plugins(LockdownPlugin)
    .run();
}
```

## Further reading / references

* TODO

## Contributing

Contributions are welcome.
For larger changes please open a issue first.

### Your contributions

Unless explicitly stated otherwise, any contribution submitted to this project shall be dual licensed under the [MIT License](LICENSE-MIT) and [Apache License, Version 2.0](LICENSE-APACHE), without any additional terms or conditions.

## License

All code in this repository is dual-licensed under either:

* [MIT License](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)

## Bevy compatibility

|   bevy | bevy_mod_lockdown |
| -----: | ----------------: |
| 0.14.1 |             0.4.0 |
| 0.14.0 |             0.3.0 |
| 0.13.1 |     0.1.0 - 0.2.0 |
