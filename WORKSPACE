load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "com_github_buildbuddy_io_rules_xcodeproj",
    sha256 = "1e2f40eaee520093343528ac9a4a9180b0500cdd83b1e5e2a95abc8c541686e2",
    url = "https://github.com/buildbuddy-io/rules_xcodeproj/releases/download/1.1.0/release.tar.gz",
)

load(
    "@com_github_buildbuddy_io_rules_xcodeproj//xcodeproj:repositories.bzl",
    "xcodeproj_rules_dependencies",
)

xcodeproj_rules_dependencies()

load(
    "@build_bazel_rules_apple//apple:repositories.bzl",
    "apple_rules_dependencies",
)

apple_rules_dependencies()

load(
    "@build_bazel_rules_swift//swift:repositories.bzl",
    "swift_rules_dependencies",
)

swift_rules_dependencies()

load(
    "@build_bazel_rules_swift//swift:extras.bzl",
    "swift_rules_extra_dependencies",
)

swift_rules_extra_dependencies()

load(
    "@build_bazel_apple_support//lib:repositories.bzl",
    "apple_support_dependencies",
)

apple_support_dependencies()

load(
    "@build_bazel_rules_apple//apple:apple.bzl",
    "provisioning_profile_repository",
)

provisioning_profile_repository(
    name = "local_provisioning_profiles",
)

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    url = "https://github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz",
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    extra_target_triples = [
        "aarch64-apple-ios-sim",
        "aarch64-apple-ios",
        "x86_64-apple-ios",
    ],
)

local_repository(
    name = "com_acme",
    path = "fake_third_party/com_acme",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

# crate repository for the RustLib core of the app

# NOTE: don't forget to add the dependency to the rust_library target!
# e.g. deps = [ ... , "@crate_index_rustlib//:swift-bridge", ]
crates_repository(
    name = "crate_index_rustlib",
    cargo_lockfile = "//Sources/RustLib:Cargo.lock",
    lockfile = "//Sources/RustLib:Cargo.Bazel.lock",
    isolated = False,  # cache results of the previous invocation to
                       # ${HOME}/.cargo so using it is fast
    packages = {
        "swift-bridge": crate.spec(
            version = "0.1.48",
        ),
        "swift-bridge-build": crate.spec(
            version = "0.1.48",
        ),
    },

    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index_rustlib//:defs.bzl", "crate_repositories")

crate_repositories()
