# Smart Rake with Caching - Example Rakefile
# When inputs change, tasks run. When nothing changed, tasks are skipped!

[build]
inputs: rake_parser/src/*.rs, rake_parser/Cargo.toml
outputs: build/rake
1) mkdir -p build
2) cd build && cmake .. -DCMAKE_BUILD_TYPE=Release && cmake --build . -j4

[test]
depends: build
inputs: rake_parser/src/*.rs
1) echo "Running tests..."
2) ./build/rake --Example_func

[clean]
outputs: build, .rake_cache
1) rm -rf build
2) rm -f .rake_cache
3) echo "Cleaned build artifacts"

[Example_func]
inputs: Rakefile
1) echo "this func is an example"

[install]
depends: build
outputs: ~/.local/bin/rake
1) ./install.sh

[docs]
inputs: BUILD.md, README.md
outputs: docs/
1) mkdir -p docs
2) cp BUILD.md README.md docs/
3) echo "Documentation generated"
content
