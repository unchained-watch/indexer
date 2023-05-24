.PHONY: anvil
anvil:
	scripts/anvil.sh run

.PHONY: deploy
deploy:
	scripts/anvil.sh deploy

.PHONY: counter_increment
counter_increment:
	scripts/anvil.sh counter_increment

.PHONY: counter_decrement
counter_decrement:
	scripts/anvil.sh counter_decrement

.PHONY: rust_core
rust_core:
	scripts/anvil.sh rust_core