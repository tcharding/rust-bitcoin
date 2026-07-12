#!/usr/bin/env bash
#
# Checks semver compatibility and non-additive cargo features.
#
# Usage: check-semver.sh [BASELINE_COMMIT]
#   BASELINE_COMMIT: Git commit hash to compare against (defaults to master)

set -euo pipefail

BASELINE_COMMIT="${1:-$(git rev-parse master)}"
WORKSPACE_ROOT="$(git rev-parse --show-toplevel)"

# Check for required tools.
if ! command -v jq &> /dev/null; then
    echo "ERROR: jq is required but not installed"
    exit 1
fi

# Get all workspace package IDs from cargo metadata.
#
# Example output:
#   path+file:///home/user/rust-bitcoin/bitcoin#bitcoin@0.33.0-beta
#   path+file:///home/user/rust-bitcoin/hashes#bitcoin_hashes@0.21.0
#   path+file:///home/user/rust-bitcoin/consensus_encoding#bitcoin-consensus-encoding@1.0.0
get_workspace_packages() {
    cargo metadata --format-version 1 | jq -r '.workspace_members[]'
}

# Get package name from package ID.
#
# Example input:  path+file:///home/user/rust-bitcoin/hashes#bitcoin_hashes@0.21.0
# Example output: bitcoin_hashes
get_package_name() {
    local pkg_id="$1"
    cargo metadata --format-version 1 | \
        jq -r ".packages[] | select(.id == \"$pkg_id\") | .name"
}

# Get all feature names for a package except `default`.
#
# Example input:  path+file:///home/user/rust-bitcoin/consensus_encoding#bitcoin-consensus-encoding@1.0.0
# Example output:
#   alloc
#   std
get_package_features() {
    local pkg_id="$1"
    cargo metadata --format-version 1 | \
        jq -r ".packages[] | select(.id == \"$pkg_id\") | .features | keys[] | select(. != \"default\")" | sort | uniq
}

# Get test variants for a package.
#
# All-features and no-default-features are always tested, and alloc
# variant is tested if the package has an alloc feature.
#
# Example input (with alloc):  path+file:///home/user/rust-bitcoin/consensus_encoding#bitcoin-consensus-encoding@1.0.0
# Example output:
#   all-features
#   no-default-features
#   alloc
get_package_variants() {
    local pkg_id="$1"
    local variants=("all-features" "no-default-features")

    # Check if package has "alloc" feature
    if cargo metadata --format-version 1 | jq -e "
        .packages[] |
        select(.id == \"$pkg_id\") |
        .features | has(\"alloc\")
    " &>/dev/null; then
        variants+=("alloc")
    fi

    printf '%s\n' "${variants[@]}"
}

# Check if a package has reached 1.0.0 or higher (semver hard-fail).
#
# Example input:  path+file:///home/user/rust-bitcoin/consensus_encoding#bitcoin-consensus-encoding@1.0.0
# Returns:        0 (success) because version is 1.0.0
is_stabilized_package() {
    local pkg_id="$1"
    cargo metadata --format-version 1 | jq -e "
        .packages[] |
        select(.id == \"$pkg_id\") |
        .version | split(\".\")[0] | tonumber >= 1
    " &>/dev/null
}

main() {
    check_non_additive_features
    check_semver_breaks
}

check_non_additive_features() {
    echo "Checking for non-additive features..."

    local has_non_additive=false

    for pkg_id in $(get_workspace_packages); do
        local pkg_name=$(get_package_name "$pkg_id")

        echo "Checking $pkg_name for non-additive features..."
        local current_features=""
        for feature in $(get_package_features "$pkg_id"); do
            current_features="$current_features --current-features $feature"
        done

        # Compare no-features to all-features. Due to cargo-semver-checks
        # interface, all-features need to be explicitly listed.
        if ! cargo semver-checks \
            -p "$pkg_name" \
            --release-type minor \
            --only-explicit-features \
            --baseline-root "$WORKSPACE_ROOT" \
            --baseline-features "" \
            $current_features; then
            echo "Non-additive cargo features found in $pkg_name"
            has_non_additive=true
        fi
    done

    if [ "$has_non_additive" = true ]; then
        echo "ERROR: Non-additive cargo features detected"
        exit 1
    fi

    echo "No non-additive cargo features found"
}

check_semver_breaks() {
    echo "Checking semver against baseline: $BASELINE_COMMIT"

    local has_hard_fails=false

    for pkg_id in $(get_workspace_packages); do
        local pkg_name=$(get_package_name "$pkg_id")

        for variant in $(get_package_variants "$pkg_id"); do
            echo "Checking $pkg_name ($variant)..."

            # Convert cargo args to cargo-semver-checks equivalents.
            local features_args=""
            case "$variant" in
                all-features)
                    features_args="--all-features"
                    ;;
                no-default-features)
                    features_args="--only-explicit-features"
                    ;;
                alloc)
                    features_args="--only-explicit-features --current-features alloc"
                    ;;
            esac

            if ! cargo semver-checks \
                -p "$pkg_name" \
                --release-type minor \
                --baseline-rev "$BASELINE_COMMIT" \
                $features_args; then
                echo "Breaking changes found in $pkg_name ($variant)"
                touch semver-break

                if is_stabilized_package "$pkg_id"; then
                    has_hard_fails=true
                fi
            fi
        done
    done

    if [ "$has_hard_fails" = true ]; then
        echo "Semver break detected in a 1.0 crate; failing CI"
        exit 1
    fi
}

#
# Main script
#
main "$@"
exit 0
