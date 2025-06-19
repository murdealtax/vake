<p align="center">
  <img src="https://raw.githubusercontent.com/murdealtax/vake/main/assets/logo.png" alt="Logo" height="200"/>
</p>

<div align="center">

![GitHub License](https://img.shields.io/github/license/murdealtax/vake?style=plastic)
![GitHub top language](https://img.shields.io/github/languages/top/murdealtax/vake?style=plastic)
<a href="https://github.com/murdealtax/vake/releases">
![GitHub Release](https://img.shields.io/github/v/release/murdealtax/vake?style=plastic)
</a>
<a href="https://create.roblox.com/store/asset/113389894632576">
![Roblox Plugin](https://img.shields.io/badge/roblox-plugin-red?style=plastic)
</a>
<a href="https://crates.io/crates/vake">
![Crate](https://img.shields.io/crates/v/vake?style=plastic)
</a>

</div>

<hr>

**Vake** is an unopinionated build system written in Rust to build and sync the In Silico project to Roblox Studio. It allows you to organize your Roblox projects in the file system without being confined to a specific organizational ideology. Project structures are defined in a `vakefile`, and can be used to describe any hierarchy model in Roblox Studio.

# Installation

Install the plugin on [Roblox](https://create.roblox.com/store/asset/113389894632576), use the attached Lua file, or build it yourself using [MoonScript](https://moonscript.org). If you are building the plugin yourself, use `moonc plugin.moon`, which will build `plugin.lua`.

Install the CLI application by building it yourself by running `cargo build --release` in the project directory, using the executable provided under [Releases](https://github.com/murdealtax/releases), or through crates.io using `cargo install vake`.

# Usage



# Configuration

Run `vake` in an uninitialized directory to create a vakefile with the default configuration. The contents of the configuration file describe a `Recipe`, which contains 3 types of configuration:

## Recipe Options

Options provide metadata about how the project is going to be constructed. Options are always prefixed with a `:` and defined with the `:option = value` syntax.

```ruby
:active_directory = "src" # Path specifying the working directory
:case_type = "pascal" # Specifies the case of script names in Roblox (pascal, camel, snake, kebab)
:case_abbreviations = false # Preserve all-caps file names to support abbreviations
:case_exceptions = [ "", "" ] # Files to skip case conversion on
:entry_name = "main.lua" # The entry file for the project, currently has no effect
:preprocess_text = true # `.txt` files are preprocessed to return strings
:preprocess_pretty = false # Stylistic changes to preprocessed files
:cc = "" # Currently unused, will eventually be used to compile files of arbitrary type
:cflags = "" # Currently unused, will eventually be used to specify flags to the compiler
```

## Recipe Associations

Associations relate specific directories to a specific instance type. This allows certain directories to create Local, Module, and Server script instances according to the specification. Associations use a `::` to separate the directory from the script type.

```ruby
client :: LocalScript
modules :: ModuleScript
server :: ServerScript # This is optional, all scripts are ServerScripts by default
```

## Recipe Entries

Entries describe how file system directories will be mapped to the Roblox explorer tree. Entries use an `->` to separate the directory from the explorer path. Special separators are used in entry paths describing the Roblox explorer hierarchy which allow precise control over how the directories are managed. Roblox paths always begin with the name of a service (`Workspace`, `ServerScriptService`, `StarterPlayer`, ...etc) and are dilineated with `.`, `:`, or `!` with each separator having a different action:

* The `.` operator simply indexes the child element of the lefthand side, where `Parent.Child` is equivalent to `Parent:FindFirstChild("Child")`. 
* The `:` operator waits for the child element asynchronously to allow dependency chains to be resolved, where `Parent:Child` is equivalent to `Parent:WaitForChild("Child")`. The `:` operator is non-blocking and works even with bad ordering.
* The `!` operator simply creates the child if it does not exist (in the form of a `Folder`), and then indexes it.

An example entry where all files in the `shaders` directory are placed into a folder under a pre-existing `Client` script titled `Shaders` in `StarterPlayerScripts`:

```ruby
shaders -> StarterPlayer.StarterPlayerScripts:Client!Shaders
```

## Recipe Paths

Both associations and entries use special `Recipe Paths` to describe the file location of a specific folder in the configuration. A recipe path is delineated with either a `.` or a `/`, and are always descendants of the `:active_directory`. An example of defining an association and entry for a deeply nested folder:

```ruby
:active_directory = "src"
...
very.specific.folder :: LocalScript # Alternatively, very/specific/folder
...
very.specific.folder -> StarterPlayer.StarterPlayerScripts # Alternatively, very/specific/folder
```

> Note: To specify the current directory, simply use `.` or `/` as the path.

## Example Recipe

Here is an example recipe that describes a project with a specific folder structure that utilizes a variety of options: 

```ruby
# Options
:active_directory = "src"
:entry_name = "game.luau"

# Associations
client :: LocalScript
client.shaders :: ModuleScript
client.modules :: ModuleScript

# Entries
/ -> ServerScriptService
server -> ServerScriptService:Main

client -> StarterPlayer.StarterPlayerScripts
client.modules -> StarterPlayer.StarterPlayerScripts:Client!Modules
client.shaders -> StarterPlayer.StarterPlayerScripts:Client!Shaders
```