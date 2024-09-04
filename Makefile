.PHONY: checks
checks:
	cargo check
	cargo test -- --nocapture
	cargo clippy -- \
		-W clippy::pedantic \
		-W clippy::cast-possible-truncation \
		-W clippy::cast-sign-loss \
		-A clippy::redundant_closure_for_method_calls \
		-A clippy::single_match_else \
		-A clippy::uninlined-format-args \
		-A clippy::missing_errors_doc
	cargo fmt --check

.PHONY: clippy_nursery
clippy_nursery:
	cargo clippy -- -W clippy::nursery

.PHONY: clippy_cargo
clippy_cargo:
	cargo clippy -- -W clippy::cargo

.PHONY: install
install:
	cargo install --path .

# Release =====================================================================

.PHONY: release
release:
	$(MAKE) -s version_tag
	$(MAKE) -s tags_push
	$(MAKE) -s crate_publish

.PHONY: version_check
version_check:
	cargo semver-checks

.PHONY: version_tag
version_tag:
	git tag v$$(cargo pkgid | awk -F'#' '{print $$2}')

.PHONY: tags_push
tags_push:
	git push --tags origin

.PHONY: crate_publish
crate_publish:
	cargo publish
