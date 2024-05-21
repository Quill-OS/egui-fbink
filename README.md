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

Here we download the toolchain.
We will start by defining a variable which will be where we store the path, feel free to specify wherever you want for this.
```
export ARMV7l_MUSL_CROSS="/home/user/projects/toolchains/armv7l-linux-musleabihf-cross" # set this to whatever you want
```
Add this to your shells config file, for example if you use bash:
```
echo "export ARMV7l_MUSL_CROSS=$ARMV7l_MUSL_CROSS" >> ~/.bash_profile 
```

```
mkdir -p $ARMV7l_MUSL_CROSS
curl -L "http://musl.cc/armv7l-linux-musleabihf-cross.tgz" -o "armv7l-linux-musleabihf-cross.tgz"
tar -xf armv7l-linux-musleabihf-cross.tgz -C $ARMV7l_MUSL_CROSS --strip 1
```

Now we add the toolchain to our path, again, remember to add a different path if you chose to store the toolchain elsewhere.
```
export PATH="$PATH:$ARMV7l_MUSL_CROSS/bin"
```
To then add this to your path permanently add it to your shell's config like we did before:
```
echo "export PATH=$PATH" >> ~/.bash_profile 
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
And we can now successfully build egui-fbink
```
cargo build --target armv7-unknown-linux-musleabihf
```
