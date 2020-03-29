<h1 align="center">Swizzler</h1>

![Swizzler Demo](/images/cli.gif)

<small style="text-align: center">Thanks to the <a href="https://freepbr.com/about-free-pbr/">Free PBR</a> website for the textures used in this demo</small>

## Installation

> NOTE: **Swizzler!** isn't available (yet) on [crates.io](https://crates.io).

Depending on your use case, you have several options available:

#### 1. Install binary from sources

You can download, build and install locally the _CLI_ using:

```sh
$ cargo install --git https://github.com/albedo-engine/swizzler.git
```

Check that the installation was successful:

```sh
$ swizzler --version
swizzler-cli 0.1.0
```

#### 2. Install library as a dependency

**Swizzler!** can also be used programmatically. Simply add a dependency to **Swizzler!** in your `Cargo.toml`:

```toml
[dependencies]
swizzler = { git = "https://github.com/albedo-engine/swizzler.git" }
```
## CLI Usage

### Manual

You can generate manually a new texture by providing, for each texture source,
which channel to extract.

```sh
$ swizzler manual -i ./texture_1.png:0 -i ./texture_2.png:0 ...
```

Each `-i` argument takes the input source, followed by the delimiting  character `:`, and the channel to read from the source.

The position of each `-i` argument is used to select the destination channel.

For instance, if you have a _RGB_ source image (`source.png`), and you want to shuffle the channels as _BGR_, you simply need to run:

```sh
$ swizzler manual -i ./source.png:2 -i ./source.png:1 -i ./source.png:0
```

The number of arguments determines the number of channels of the output image. Calling:

```sh
$ swizzler manual -i ./source.png:0
```

Will generate a _Grayscale_ image.

You can let some channels empty, by specifying the `none` keyword on a channel:

```sh
$ swizzler manual -i red.png:0 -i none -i none -i alpha.png:3
```

### Folder processing

You may want to process an entire folder hierarchy. The [Manual Command](#manual) is handy, but can turn to be difficult to use when you need to find what files should be grouped together.

The `session` command let you use a JSON configuration file contains information about how to resolve files, and what textures to generate.

Let's see how you could process an entire folder of images, retrieve files that belong to a common asset, and generate textures containing the metalness in the `red` channel, and the roughness in the `alpha channel`. Let's assume the textures are in a folder named `textures`:

```sh
$ ls ./textures
enemy_albedo.png    enemy_metalness.png enemy_roughness.png hero_albedo.png     hero_metalness.png  hero_roughness.png
```

We can use this configuration file to generate our textures:

```sh
$ cat ./config.json
{
  "base": "(.*)_.*",
  "matchers": [
      { "id": "metalness", "matcher": "(?i)metal(ness)?" },
      { "id": "roughness", "matcher": "(?i)rough(ness)?" },
      { "id": "albedo", "matcher": "(?i)albedo" }
  ],
  "targets": [
    {
      "name": "-metalness-roughness.png",
      "output_format": "png",
      "inputs": [
          [ "metalness", 0 ],
          null,
          null,
          [ "roughness", 0 ]
      ]
    }
  ]
}
```

#### `base` attribute

The `base` attribute describes how to extract the name of the asset from a path.
This **has to be** a [Regular Expression](https://en.wikipedia.org/wiki/Regular_expression) with **one** capturing group. In this example, the base captures everything before the last `_` character.
All the files starting with `hero_` would have the base `hero`, and all the files
starting with `enemy_` the base `enemy`.

#### `matchers` attribute

The `matchers` attribute provide a list of files to match under the same asset. In this
example, the metalness, roughness, and albedo textures belonging to a same
asset will get resolved together.

#### `targets` attributes

The `targets` attribute makes use of the `matchers` list in order to know what textures
to use as sources. Each target generates one texture, containing the
combination of specificied sources.

We use here the `metalness` and `roughness` identifiers in order to create
a new texture, containing **4** channels. The `red` channel will be filled with
the metalness texture `red channel`, and the `alpha` channel will be filled with
the roughness texture `red channel`.

The `name` attribute allows you to customize the name used when saving the file,
and the `output_format` allows you to specify an [encoding format](#arguments).

We can now run the CLI on our `textures/` folder:

```sh
$ swizzler session --folder ./textures --config ./config.json
```

Alternatively, you can provide the `config.json` file on `stdin`:

```sh
$ cat ./config.json | swizzler session --folder ./textures
```

The results should be generated in the folder `__swizzler_build`, as follows:

```sh
$ ls ./__swizzler_build
enemy-metalness-roughness.png hero-metalness-roughness.png
```

For more information about all arguments accepted by the CLI, have a look at the
[Arguments Section](#arguments)

### Arguments

#### Manual command

Usage:

```sh
$ swizzler manual [-i PATH] ... [-i PATH]
```

|Argument|Value|Description|
|:--:|:--:|:--------------------|
|**-o, --output**|_Path_|Relative path to which output the texture|
|**-i, --input**|_Path_|Relative path to the texture source to use|
|**-f, --format**|_String_|Format to use for saving. Default to the extension format if not provided|

#### Session command

Usage:

```sh
$ swizzler session --folder PATH [--config PATH_TO_CONFIG]
```

|Argument|Value|Description|
|:--:|:--:|:--------------------|
|**-f, --folder**|_Path_|Relative path to the folder to process|
|**-o, --output**|_[Path]_|Relative path to the folder in which to output files|
|**-c, --config**|_[Path]_|Relative path to the config to use|
|**-n, --num_threads**|_[Number]_|Number of threads to use. Default to the number of logical core of the machine|

List of all available encoding format:

* `png`
* `jpg`
* `tga`
* `tif`
* `pnm`
* `ico`
* `bmp`

Those formats can be used directly on the CLI using the `manual` command, or via
a configuration file (for `session` run).

## Library usage

### Swizzle

Channel descriptors describe how to use a source image, and what channel to extract.

There are several ways to create descriptors:

```rust
use swizzler::{ChannelDescriptor};

// From a string.
let descriptor = ChannelDescriptor::from_description("./my_input.png:0").unwrap();

// From path + channel
let path = std::Path::PathBuf::from("./my_input.png");
let descriptor = ChannelDescriptor::from_path(path, 0).unwrap();

// From an image + channel
let descriptor = ChannelDescriptor::from_path(my_image, 0).unwrap();
```

You can then use any of the following to create a swizzled image:

* `to_luma()` ⟶ swizzle inputs into a _Grayscale_ image
* `to_luma_a()` ⟶ swizzle inputs into a _Grayscale-Alpha_ image
* `to_rgb()` ⟶ swizzle inputs into a _RGB_ image
* `to_rgba()` ⟶ swizzle inputs into a _RGBA_ image

Example:

```rust
use swizzler::{to_rgba};

let r_channel = ChannelDescriptor::from_path(..., ...).unwrap();
let a_channel = ChannelDescriptor::from_path(..., ...).unwrap();

// Generates a RGBA image with two descriptors. The output image `green`
// and `blue` channels are left empty.
let result = to_rgba(Some(r_channel), None, None, Some(a_channel)).unwrap();
```

> NOTE: you can use `None` to let a channel empty.

The result image is an `ImageBuffer` from the [image crate](https://docs.rs/image/0.23.2/image/struct.ImageBuffer.html), that you can manipulate like any other image:

```rust
result.save("./output.png").unwrap();
```

### Running a session

You can run a session programmatically by creating an `AssetReader` (A.K.A a "resolver"),
and a `Session`.

```rust
use regex::Regex;
use swizzler::session::{
    GenericAssetReader
    GenericTarget,
    RegexMatcher,
    Session,
};

// Creates a resolver and add matcher to it. Remember that matchers
// are used to group files together under a common asset.
let resolver = GenericAssetReader::new()
  .set_base(Regex::new("(.*)_.*").unwrap())
  .add_matcher(
    Box::new(RegexMatcher::new("metalness", Regex::new(r"(?i)metal(ness)?").unwrap()))
  )
  .add_matcher(
    Box::new(RegexMatcher::new("roughness", Regex::new(r"(?i)rough(ness)?").unwrap()))
  )

// Creates a target. Each target describes a texture to generate.
let metal_roughness_target = GenericTarget::new(vec![
  ("metalness", 0),
  None,
  None,
  ("roughness", 0),
])

// The `Session` will generate images using multiple threads, and save them
// to disk.
let session = Session::new()
  .set_output_folder(...)
  .set_max_threads_nb(...)
  .add_target(metal_roughness_target);

// Reads all assets on the main thread, using our assets reader.
let assets = match resolve_assets_dir(&command.folder, &resolver) {
  Some(list) => list,
  Err(error) => eprintln!("Error reading folder: {:?}", error),
};

// Goes through all assets, load all sources, swizzle the textures and save them
// to disk.
let errors = session.run(&assets);
for e in &errors {
    eprintln!("Error processing file: {:?}", e);
}
```
