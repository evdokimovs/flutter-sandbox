deps:
	flutter pub get

fmt:
	flutter format $(if $(call eq,$(check),yes),-n --set-exit-if-changed,) .

lint:
	flutter analyze .

test:
	flutter test

.PHONY:
	deps fmt lint
