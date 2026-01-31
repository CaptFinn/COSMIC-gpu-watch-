name := 'cosmic-applet-gpu-watch'
appid := 'com.captfinn.CosmicAppletGpuWatch'

prefix := '/usr'
bindir := prefix / 'bin'
datdir := prefix / 'share'

# Build in release mode
build:
    cargo build --release

# Install to system (removes old conflicting binaries first)
install: build
    -rm -f /usr/local/bin/{{name}}
    -rm -f ~/.local/bin/{{name}}
    -rm -f ~/.local/share/cosmic/applets/{{name}}
    install -Dm0755 target/release/{{name}} {{bindir}}/{{name}}
    install -Dm0644 data/{{appid}}.desktop {{datdir}}/applications/{{appid}}.desktop

# Uninstall from system
uninstall:
    rm -f {{bindir}}/{{name}}
    rm -f {{datdir}}/applications/{{appid}}.desktop

# Clean build artifacts
clean:
    cargo clean
