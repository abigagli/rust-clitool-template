rpi_cpu_target := "aarch64-unknown-linux-gnu"
rpi_default_target_host := "{{rpi_host}}"
rpi_default_target_folder := "."
default_binary := `cargo read-manifest | jq '.name'`

default:
  @just --list --justfile {% raw %}{{justfile()}}{% endraw %}
  @echo "default_binary: {% raw %}{{default_binary}}{% endraw %}"
  @echo "rpi_default_target_host: {% raw %}{{rpi_default_target_host}}{% endraw %}"
  @echo "rpi_default_target_folder: {% raw %}{{rpi_default_target_folder}}{% endraw %}"

_upload build_type_and_executable target_cpu target_host target_folder:
    /opt/local/bin/rsync -aPvz --zc lz4 target/{% raw %}{{target_cpu}}/{{build_type_and_executable}}{% endraw %} {% raw %}{{target_host}}:{{target_folder}}{% endraw %}/

upload_rpi build_type executable=default_binary target_host=rpi_default_target_host target_folder=rpi_default_target_folder: ( _upload build_type/executable rpi_cpu_target target_host target_folder)

# test_support_lib:
#   cargo test -p support_lib

############ BUILDING FOR RPI ############
build_for_rpi *ARGS:
  @echo "Building for RPI using .cargo/config.toml with args: {% raw %}{{ARGS}}{% endraw %}"
  cargo rpi {% raw %}{{ARGS}}{% endraw %}

############ BUILDING FOR WINDOWS ############
build_for_windows *ARGS:
  @echo "Building for Windows using alpine-rustx Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project rustx_crosscompiler:latest cargo build --target x86_64-pc-windows-gnu {% raw %}{{ARGS}}{% endraw %}

############ BUILDING FOR MACOS/Aarch64 ############
build_for_macos_aarch64 *ARGS:
  @echo "Building for MacOS/aarch64 using alpine-rustx Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project rustx_crosscompiler:latest cargo build --target aarch64-apple-darwin {% raw %}{{ARGS}}{% endraw %}

zigbuild_for_macos_aarch64 *ARGS:
  @echo "Building for MacOS/aarch64 using zigbuild Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project ghcr.io/rust-cross/cargo-zigbuild cargo zigbuild --target aarch64-apple-darwin {% raw %}{{ARGS}}{% endraw %}

############ BUILDING FOR MACOS/Intel ############
build_for_macos_intel *ARGS:
  @echo "Building for MacOS/Intel using alpine-rustx Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project rustx_crosscompiler:latest cargo build --target x86_64-apple-darwin {% raw %}{{ARGS}}{% endraw %}

zigbuild_for_macos_intel *ARGS:
  @echo "Building for MacOS/Intel using zigbuild Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project ghcr.io/rust-cross/cargo-zigbuild cargo zigbuild --target x86_64-apple-darwin {% raw %}{{ARGS}}{% endraw %}

############ DEPLOYING TO RPI ############
deploy_rpi build_type executable=default_binary target_host=rpi_default_target_host target_folder=rpi_default_target_folder:
  #!/usr/bin/env bash
  #set -euxo pipefail
  set -euo pipefail
  mode=$(if [ "{% raw %}{{build_type}}{% endraw %}" = "debug" ]; then echo "dev"; else echo "{% raw %}{{build_type}}{% endraw %}"; fi)
  #echo "Building with profile: $mode"
  just build_for_rpi -p {% raw %}{{executable}}{% endraw %} --profile $mode
  echo "Deploying $mode build of {% raw %}{{executable}}{% endraw %} to {% raw %}{{target_host}}:{{target_folder}}{% endraw %}"
  #just upload_rpi_release {{executable}} {{target_host}} {{target_folder}}
  just upload_rpi {% raw %}{{build_type}}{% endraw %} {% raw %}{{executable}}{% endraw %} {% raw %}{{target_host}}{% endraw %} {% raw %}{{target_folder}}{% endraw %}

############ UPGRADE DEPENDENCIES #############
check_stale_deps *ARGS:
    cargo upgrade -n --verbose {{ ARGS }}

upgrade_dependency *CRATE:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo upgrade  --incompatible allow -p {{ CRATE }}

validate_dep_upgrade *CARGO_FLAGS:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo update
    cargo build --workspace {{ CARGO_FLAGS }}
    just build_for_rpi -p iq2angles
    cargo test --workspace --no-fail-fast {{ CARGO_FLAGS }}


