load("@build_bazel_rules_swift//swift:swift.bzl", "swift_library")

filegroup(
    name = "SwiftLibSources",
    srcs = glob(["**/*.swift"]),
    visibility = ["//visibility:public"],
)

swift_library(
    name = "SwiftLib",
    srcs = [":SwiftLibSources"],
    module_name = "SwiftLib",
    visibility = ["//visibility:public"],
)

