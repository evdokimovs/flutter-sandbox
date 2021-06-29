# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)
deps:
	flutter pub get

fmt:
	flutter format $(if $(call eq,$(check),yes),-n --set-exit-if-changed,) .

lint:
	flutter analyze .

test:
	flutter test

build:
	flutter build $(if $(call eq,$(platform),ios),ios,apk)

release:
	-git tag -d latest
	git tag latest
	git push latest

.PHONY: deps build fmt lint test
