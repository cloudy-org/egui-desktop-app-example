This is just a **simplified example** of an Egui app at cloudy-org.

For more advanced and up to date material you'll want to read into the source code of existing projects like [Roseate](https://github.com/cloudy-org/roseate). Cloudy-org is still **very new** so documentation and the cirrus toolkit is still not there or ready yet for new developers to board ship and work on new products. **Not** to deter you from contributions that is.

# How to run example in dev mode.

1. Clone repo.
```sh
git clone https://github.com/cloudy-org/egui-desktop-app-example
cd egui-desktop-app-example
```

2. Pull submodules.
```sh
git submodule update --init --recursive
```

3. Run dev build.
```sh
cargo run
```

You can also mess around with the theming by passing `--theme light` like so:

```sh
cargo run -- --theme light
```