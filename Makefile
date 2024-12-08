.PHONY: all
all: run

.PHONY: run
run:
	@if [ -z "$(DAY)" ]; then \
		echo "Error: DAY variable not set. Usage: make run DAY=day_01"; \
		exit 1; \
	fi
	@if [ ! -d "$(DAY)" ]; then \
		echo "Error: Directory '$(DAY)' does not exist."; \
		exit 1; \
	fi
	@echo "Running Rust project in directory '$(DAY)'..."
	@cd $(DAY) && cargo run

.PHONY: new_day
new_day:
	@if [ -z "$(DAY)" ]; then \
		echo "Error: DAY variable not set. Usage: make new_day DAY=day_01"; \
		exit 1; \
	fi
	@echo "Creating new Rust project in directory '$(DAY)'..."
	@cargo new $(DAY)
	@touch $(DAY)/description.txt
	@mkdir -p $(DAY)/input
	@touch $(DAY)/input/input.txt
	@touch $(DAY)/input/example.txt
