load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")
load("@build_bazel_rules_swift//swift:swift.bzl", "swift_library")

rust_library(
    name = "lib",
    srcs = glob(["src/**/*.rs"]),
    edition = "2018",
    deps = [
        ":alloc",
        "@crate_index_rustlib//:swift-bridge",
        "//Sources/SwiftLib",  # Swift library we can call from Rust
    ],
)

# This being required is a bug in objc_library, ideally 'RustLib' could depend
# directly on 'lib'
cc_library(
    name = "lib_shim",
    deps = [":lib"],
)

cc_library(
    name = "alloc",
    srcs = ["alloc.cc"],
)

# Swift-bridge integration

# Build our swift-bridge binary, which will generate the bridge code.
rust_binary(
    name = "swift_bridge_tool",
    srcs = ["swift_bridge.rs"],
    edition = "2021",
    deps = [
        "@crate_index_rustlib//:swift-bridge-build",
    ],
)

# Collect the rust sources to pass to the swift-bridge tool.
filegroup(
    name = "rustsrc",
    srcs = glob(["src/**/*.rs"]),
)

# Run the tool once to generate the C headers.
genrule(
    name = "gen_hdrs",
    srcs = [":rustsrc"],
    outs = [
        "SwiftBridgeCore.h",
        "gen.h",
    ],
    cmd = """
$(location :swift_bridge_tool) $(locations :rustsrc)
mv SwiftBridgeCore.h $(location SwiftBridgeCore.h)
mv gen/gen.h $(location gen.h)
""",
    tools = [":swift_bridge_tool"],
)

# Run the tool again to generate the swift code. (A single run generates both,
# but this partitions the rule's output filetypes. There's probably a better
# way...)
# This time we also need to prepend 'import bridge' on the swift files so they
# can find the 'bridge' Objective-C module defined below as "objc_lib"
genrule(
    name = "gen_swift",
    srcs = [":rustsrc"],
    outs = [
        "SwiftBridgeCore.swift",
        "gen.swift",
    ],
    cmd = """
$(location :swift_bridge_tool) $(locations :rustsrc)
echo 'import bridge' | cat - SwiftBridgeCore.swift > \
        $(location SwiftBridgeCore.swift)
echo 'import bridge' | cat - gen/gen.swift > $(location gen.swift)
""",
    tools = [":swift_bridge_tool"],
)

# Generate an Objective-C library with the C headers, that also depends on our
# Rust library via :lib_shim.
objc_library(
    name = "objc_lib",
    hdrs = [":gen_hdrs"],
    module_name = "bridge",
    deps = [":lib_shim"],
)

# Finally, we make our top-level swift library with the generated swift that
# depends on the Objective-C library.
swift_library(
    name = "RustLib",
    srcs = [
        ":gen_swift",

        # Since swift-bridge generates symbols for the Rust-facing Swift
        # library, we include its sources here so the linker can find them.
        # Ideally we'd tell swift-bridge to only generate the symbols needed
        # for Swift to call Rust.
        "//Sources/SwiftLib:SwiftLibSources"
    ],
    module_name = "RustLib",
    visibility = ["//visibility:public"],
    deps = [":objc_lib"],
)
