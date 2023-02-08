# BazelCon 2022 Demo, Now With [`swift-bridge`](https://github.com/chinedufn/swift-bridge)

`swift-bridge` integration could certainly be improved, but it's a start. See
`Sources/RustLib/BUILD` for the details. Currently only supports Swift calling
Rust.

<a href="https://youtu.be/B__SHnz3K3c" title="YouTube - How to integrate Bazel with Xcode using rules_xcodeproj"><img src="https://img.youtube.com/vi/B__SHnz3K3c/maxresdefault.jpg" alt="YouTube - How to integrate Bazel with Xcode using rules_xcodeproj" width="640"></a>

## Commands

### Whole project

```
bazel run //:xcodeproj
xed Demo.xcodeproj
```

### Focused project

```
bazel run //:xcodeproj-focused
xed Demo-Focused.xcodeproj
```
