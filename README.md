# egui-fbink

Huge thanks (and code theft) to
- https://github.com/asimaranov/egui_pocketbook
- https://github.com/asimaranov/egui_template_pocketbook

# Setting this up
Things you will need:
```
Clang
Rust
GlibC
```

## Installing the toolchains

Here we download the toolchain, for this guide it is assumed that you want to store it in a `toolchains` folder in your home directory. Feel free to edit the paths to your preferences if you wish.
```
mkdir ~/toolchains
cd ~/toolchains
curl -L "http://musl.cc/armv7l-linux-musleabihf-cross.tgz" -o "armv7l-linux-musleabihf-cross.tgz"
tar -xf armv7l-linux-musleabihf-cross.tgz
```

Now we add the toolchain to our path, again, remember to add a different path if you chose to store the toolchain elsewhere.
```
PATH="$PATH:/home/$USER/toolchains/armv7l-linux-musleabihf-cross/bin"
```

Now add the rust toolchain:
```
rustup target add armv7-unknown-linux-musleabihf
```

Now switch to wherever you want to store the project.
```
git clone https://github.com/Quill-OS/egui-fbink --recursive
cd egui-fbink
```

Here we compile the libraries.
```
cd fbink-sys/FBInk
make -j8 static MINIMAL=1 BITMAP=1 OPENTYPE=1 IMAGE=1 FONTS=1 CROSS_COMPILE=armv7l-linux-musleabihf-
cd ..
cargo build --target armv7-unknown-linux-musleabihf
cd .. 
```

Here we need to edit the build.rs to edit this line from:
```rust
std::fs::copy("/home/build/qos/toolchains/armv7l-linux-musleabihf-cross/armv7l-linux-musleabihf/lib/libdl.a", out_path.join("libdl.a")).expect("Failed to copy libdl.a library");
```
To:
```rust
std::fs::copy("/home/user/toolchains/armv7l-linux-musleabihf-cross/armv7l-linux-musleabihf/lib/libdl.a", out_path.join("libdl.a")).expect("Failed to copy libdl.a library");
```

Make sure to use your own username instead of `user`.

