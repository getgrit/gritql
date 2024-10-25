#!/bin/bash

# Check if repo name and repo path were supplied
if [ "$#" -lt 3 ] || [ "$#" -gt 5 ]; then
    echo "Usage: $0 <repo_name> <repo_path> [md|json] [test_filter] [skip_build]"
    exit 1
fi

# Set the repo name and path
repo_name=$1
repo_path=$2

echo "Benchmarking $repo_name at $repo_path"

# Check the output format
output_format=${3:-md}

if [ "$output_format" != "md" ] && [ "$output_format" != "json" ]; then
    echo "Invalid output format. Use 'md' or 'json'"
    exit 1
fi

test_filter=${4:-main}

echo "Running these tests: $test_filter"

skip_build=${5:-false}


# Run yarn build release
if [ "$skip_build" == "false" ]; then
    echo "Running yarn build release"
    yarn build

    # Check if yarn build was successful
    if [ $? -ne 0 ]; then
        echo "Yarn build failed"
        exit 1
    fi
else
    echo "Skipping bin build"
fi

# Calculate the bin path relative to the repo path
script_dir="$(dirname "$(readlink -f "$0")")"
fixtures_dir="${script_dir}/../fixtures"
bin_path="${script_dir}/../../../../../target/release/grit"

# Create a temp directory for the repo
temp_repo_path=$(mktemp -d)

# Run hyperfine against the yarn build
if [ "$output_format" == "md" ]; then
    # Extract last part of repo_path as filename
    filename=$(basename "$(dirname "$repo_path")")-$(basename "$repo_path")
    output_file="$script_dir/$filename.md"
    hyperfine_option="--export-markdown $output_file  --show-output --warmup 3"
else
    output_file="$script_dir/$repo_name.json"
    hyperfine_option="--export-json $output_file --warmup 10"
fi

export GRIT_DOWNLOADS_DISABLED=true

# benchmark names
bench_names=(
  "main_console_log_hidden"
  "main_console_log"
  "main_console_log_jsonl_rewrite"
  "main_console_log_jsonl_rewrite_hidden"
  "main_console_log_rewrite"
  "main_react_to_hooks"
  "main_quick_patterns_test"
  "optimize_filename"
  "stdlib_patterns_test"
)

commands=(
  "cd ${temp_repo_path} && ${bin_path} apply --force --dry-run --output=none ${fixtures_dir}/simple_patterns/console_log.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force --dry-run ${fixtures_dir}/simple_patterns/console_log.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force --dry-run --jsonl ${fixtures_dir}/simple_patterns/console_log_rewrite.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force --dry-run --jsonl --min-visibility hidden ${fixtures_dir}/simple_patterns/console_log_rewrite.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force ${fixtures_dir}/simple_patterns/console_log_rewrite.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force ${fixtures_dir}/simple_patterns/react_to_hooks.grit ${temp_repo_path}"
  "cd ${temp_repo_path} && ${bin_path} apply --force ${fixtures_dir}/simple_patterns/optimize_filename.grit ${temp_repo_path}"
  "cd ${fixtures_dir}/patterns_test && ${bin_path} patterns test"
  "cd ${temp_repo_path} && ${bin_path} patterns test --exclude ai"
)

# Filter bench_names and commands to only include requested commands
filtered_bench_names=()
filtered_commands=()

for i in "${!bench_names[@]}"; do
    if [[ "${bench_names[$i]}" == *"$test_filter"* ]]; then
        filtered_bench_names+=("${bench_names[$i]}")
        filtered_commands+=("${commands[$i]}")
    fi
done

# Check if the command names length matches with commands length
if [ ${#filtered_bench_names[@]} -ne ${#filtered_commands[@]} ]; then
    echo "Error: Length of command names does not match the length of commands"
    exit 1
fi

hyperfine \
  --prepare "rm -rf ${temp_repo_path} && cp -r ${repo_path} ${temp_repo_path} && cd ${temp_repo_path} && rm -rf .grit/.gritmodules" \
  $hyperfine_option \
  "${filtered_commands[@]}"

# Check if hyperfine command was successful
if [ $? -ne 0 ]; then
    echo "Hyperfine command failed, running again with output enabled"
    hyperfine \
      --prepare "rm -rf ${temp_repo_path} && cp -r ${repo_path} ${temp_repo_path} && cd ${temp_repo_path} && rm -rf .grit/.gritmodules" \
      --show-output \
      "${filtered_commands[@]}"
    exit 1
fi

# Analyze JSON output as a test
if [ "$output_format" == "json" ]; then
    # NOTE: JSON results are in *seconds*
    our_times=$(jq -r '.results[].mean' "$output_file")
    echo "$our_times"
    index=0

    for time in $our_times; do
      echo "${filtered_bench_names[$index]}: $time"
      metric_name="grit.marzano.bench.$repo_name.${filtered_bench_names[$index]}"
      $script_dir/../../../../ops/grafana/report.sh "$metric_name" "$time" || true
      ((index++))
    done

    echo "Checking for slow commands"

    slow_commands=$(jq -r '.results[] | select(.mean > 2) | "\(.command) - \(.mean)s"' "$output_file")

    if [ -n "$slow_commands" ] && [[ "$slow_commands" != *"patterns test --exclude ai"* ]]; then
        echo "The following commands are too slow:"
        echo "$slow_commands"
        exit 1
    fi
    echo "The CLI is fast enough!"
fi

# Clean up the temporary repository directory
rm -rf "${temp_repo_path}"

exit 0
