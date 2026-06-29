agent-check:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
    cargo xtask registry-check
    cargo xtask conductor-check
    cargo xtask safety-check
    just cli-contract-check
    just context-integrity-check
    cargo xtask precompile-check

precompile-check:
    python3 tools/precompile_static_check.py
    cargo xtask precompile-check

inspect-example:
    cargo run -p apfs-cli -- inspect --json fixtures/example-non-apfs.bin

cursor-example:
    cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 1500 --xid 70

volume-example:
    cargo run -p apfs-cli -- volumes --json fixtures/synthetic-volume-superblock.img

read-object-example:
    cargo run -p apfs-cli -- read-object --json fixtures/synthetic-mapped-object-read.img --oid 1500 --xid 70

ls-example:
    cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img

cat-example:
    cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt

stat-example:
    cargo run -p apfs-cli -- stat --json fixtures/synthetic-file-preview.img --name hello.txt

extract-example:
    rm -rf target/synthetic-extract && cargo run -p apfs-cli -- extract --json fixtures/synthetic-file-preview.img --name hello.txt --dest target/synthetic-extract

registry-check:
    cargo xtask registry-check

conductor-check:
    cargo xtask conductor-check

safety-check:
    cargo xtask safety-check
    just cli-contract-check
    just context-integrity-check

fixture-template-check:
    cargo xtask fixture-manifest-check fixtures/manifests/macos-minimal-apfs-001.template.json

real-fixture-create:
    ./tools/macos/create_real_apfs_fixture.sh

real-fixture-feedback inspect_json manifest_json out_dir:
    cargo xtask real-fixture-feedback {{inspect_json}} {{manifest_json}} {{out_dir}}

promote-feedback-example:
    cargo xtask promote-feedback fixtures/feedback/sample-real-fixture-feedback.json target/promoted-feedback-sample

remaining-elements:
    cat REMAINING_ELEMENTS.md
synthetic-oracle-check:
    python3 tools/synthetic_fixture_oracle.py

traceability:
    python3 tools/traceability_matrix.py

loop-dashboard:
    python3 tools/loop_dashboard.py

release-preflight:
    python3 tools/release_preflight.py --write-manifest

diagnostics-bundle sample="fixtures/diagnostics/sample-inspect.json":
    rm -rf target/diagnostics-bundle
    cargo xtask diagnostics-bundle --out target/diagnostics-bundle {{sample}}

rust-api-map:
    cargo xtask rust-api-map

next-loop-plan:
    cargo xtask next-loop-plan

windows-readiness-check:
    cargo xtask windows-readiness-check

fuzz-scaffold-check:
    cargo xtask fuzz-scaffold-check

doctor-example:
    cargo run -p apfs-cli -- doctor --json fixtures/synthetic-file-preview.img

diagnostics-export-example:
    rm -rf target/redacted-diagnostics
    cargo run -p apfs-cli -- diagnostics-export --json fixtures/synthetic-file-preview.img --out target/redacted-diagnostics

cli-contract:
    python3 tools/cli_contract_snapshot.py

api-surface:
    python3 tools/api_surface_snapshot.py

source-metrics:
    python3 tools/source_metrics.py

safety-case-check:
    python3 tools/safety_case_check.py

cli-contract-check:
    python3 tools/cli_contract_check.py

context-integrity-check:
    python3 tools/context_integrity_check.py

mount-plan-example:
    cargo run -p apfs-cli -- mount-plan --json fixtures/synthetic-file-preview.img --mountpoint X:

diagnostics-example:
    cargo run -p apfs-cli -- diagnostics-bundle --json fixtures/synthetic-file-preview.img

feature-readiness:
    python3 tools/feature_readiness_snapshot.py

version-check:
    python3 tools/version_consistency_check.py


handoff-check:
    python3 tools/handoff_readiness_check.py
    python3 tools/release_scaffold_check.py

cargo-triage-sample:
    python3 tools/cargo_error_to_tracks.py fixtures/feedback/sample-cargo-error.log target/cargo-triage-sample

release-scaffold-check:
    python3 tools/release_scaffold_check.py

config-sanity:
    python3 tools/config_sanity_check.py

local-env-doctor:
    mkdir -p target
    python3 tools/local_env_doctor.py --json target/local-env-doctor.json

handoff-status:
    python3 tools/handoff_status.py --write

repo-manifest:
    python3 tools/repo_manifest.py --write

known-risk-check:
    python3 tools/known_risk_check.py

handoff-candidate-check:
    python3 tools/config_sanity_check.py
    python3 tools/known_risk_check.py
    python3 tools/local_env_doctor.py --json target/local-env-doctor.json
    python3 tools/handoff_status.py --write
    python3 tools/repo_manifest.py --write
    python3 tools/precompile_static_check.py

local-compile-loop:
    python3 tools/local_compile_loop.py --out-dir target/local-compile-loop

cargo-workspace-audit:
    python3 tools/cargo_workspace_audit.py

macos-fixture-dry-run:
    python3 tools/macos_fixture_dry_run.py

winfsp-callback-matrix:
    python3 tools/winfsp_callback_matrix.py

production-gap-report:
    python3 tools/production_gap_report.py

current-env-inventory:
    python3 tools/current_environment_inventory.py

current-env-remaining:
    python3 tools/current_env_remaining.py

dependency-graph:
    python3 tools/cargo_dependency_graph.py

negative-fixtures:
    python3 tools/synthetic_negative_fixture_generator.py

test-matrix:
    python3 tools/test_matrix_generator.py

archive-audit:
    python3 tools/handoff_archive_audit.py

# v0.25.0 current-environment checks
apfs-offset-audit:
    python3 tools/apfs_offset_audit.py

golden-outputs:
    python3 tools/golden_output_generator.py

dependency-policy-audit:
    python3 tools/dependency_license_policy_check.py

backlog-issue-export:
    python3 tools/backlog_issue_export.py

current-env-selftest:
    python3 tools/current_env_selftest.py

markdown-link-audit:
    python3 tools/markdown_link_audit.py

shell-script-audit:
    python3 tools/shell_script_static_check.py

documentation-index-audit:
    python3 tools/documentation_index_audit.py

fixture-coverage-report:
    python3 tools/fixture_coverage_report.py

cargoless-smoke:
    python3 tools/cargoless_smoke_suite.py

current-env-completion:
    python3 tools/current_env_completion_report.py


tool-matrix:
    python3 tools/tool_capability_matrix.py

rust-static-lint:
    python3 tools/rust_static_lint.py

local-command-plan:
    python3 tools/local_command_plan.py

package-integrity:
    python3 tools/package_integrity_audit.py

mvp-blockers:
    python3 tools/mvp_blocker_tasklist.py

agent-brief:
    python3 tools/agent_handoff_brief.py


# v0.26.0 final current-environment closure checks
source-debt-report:
    python3 tools/source_debt_report.py

production-claim-guard:
    python3 tools/production_claim_guard.py

handoff-manifest-verify:
    python3 tools/handoff_manifest_verify.py

mvp-blocker-dag:
    python3 tools/mvp_blocker_dag.py

local-migration-commands:
    python3 tools/local_migration_commands.py

current-env-final-report:
    python3 tools/current_env_final_report.py

quality-gate-check:
    python3 tools/quality_gate_check.py

docs-site-check:
    python3 tools/docs_site_static_check.py

test-scaffold-audit:
    python3 tools/test_scaffold_audit.py

ci-quality-gate-audit:
    python3 tools/ci_quality_gate_audit.py

test-strategy-audit:
    python3 tools/test_strategy_audit.py

docs-site-audit:
    python3 tools/docs_site_audit.py

profiling-plan-audit:
    python3 tools/profiling_plan_audit.py

current-env-tool-inventory-extended:
    python3 tools/current_env_tool_inventory_extended.py

# v0.27.0 strict quality/docs gates
quality-gate-check:
    python3 tools/quality_gate_check.py

docs-site-check:
    python3 tools/docs_site_check.py

testing-infra-report:
    python3 tools/testing_infrastructure_report.py

docs-site-build:
    cd docs-site && npm install && npm run build

strict-local-quality:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo nextest run --workspace --all-features
    cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90 --summary-only
    cargo test -p apfs-types --test property_nx_superblock
    cargo test -p apfs-core --test integration_inspect
    cargo test -p apfs-cli --test e2e_cli

# v0.28.0 quality assurance audits
workflow-policy-audit:
    python3 tools/github_workflow_policy_audit.py

docs-package-audit:
    python3 tools/docs_package_audit.py

test-inventory-report:
    python3 tools/test_inventory_report.py

hypothesis-strategy-audit:
    python3 tools/hypothesis_strategy_audit.py

profiling-budget-check:
    python3 tools/profiling_budget_check.py

quality-gate-evidence:
    python3 tools/quality_gate_evidence.py

quality-docs-test-meta-check:
    python3 tools/docs_package_audit.py
    python3 tools/github_workflow_policy_audit.py
    python3 tools/test_inventory_report.py
    python3 tools/hypothesis_strategy_audit.py
    python3 tools/profiling_budget_check.py
    python3 tools/quality_gate_evidence.py

bleeding-edge-audit:
    python3 tools/bleeding_edge_repo_audit.py

workflow-hardening-audit:
    python3 tools/github_actions_hardening_audit.py
    python3 tools/action_pinning_audit.py
