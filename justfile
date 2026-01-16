rpi_cpu_target := "aarch64-unknown-linux-gnu"
macmini_cpu_target := "aarch64-apple-darwin"
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
  @echo "Building for Windows using Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project rustx_crosscompiler:latest cargo build --target x86_64-pc-windows-gnu {% raw %}{{ARGS}}{% endraw %}

############ BUILDING FOR MACMINI ############
build_for_macmini *ARGS:
  @echo "Building for MACMINI using Docker cross-compiler with args: {% raw %}{{ARGS}}{% endraw %}"
  docker run --rm -v "{% raw %}{{justfile_directory()}}{% endraw %}:/rust_project" -w /rust_project rustx_crosscompiler:latest cargo build --target aarch64-apple-darwin {% raw %}{{ARGS}}{% endraw %}

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
