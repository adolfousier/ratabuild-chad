goood now lets update to next version v0.1.1 on cargo and CHANGELOG new entry with the above, and then commit and push to main and new version with release notes similarly to last version



cargo build --release && RUST_LOG=debug cargo run 2> debug.log

git add . && git commit -m "Release v0.1.5: Add cross-platform support, AFK detection, app categorization, commands popup, and responsive UI" && git push origin main && git tag v0.1.5 && git push origin v0.1.5

