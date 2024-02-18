.PHONY: init build run

setup:
	cd $(CURDIR)/frontend && npm i && cd $(CURDIR)/backend && cargo fetch
build:
	cd $(CURDIR)/frontend && npm run build \
	&& cd $(CURDIR)/backend && cargo build \
	&& cp $(CURDIR)/backend/target/debug/backend $(CURDIR)/build \
	&& cp $(CURDIR)/backend/.env $(CURDIR)/build \
	&& sed -i '' 's/env=development/env=production/g' $(CURDIR)/build/.env
run:
	cd $(CURDIR)/frontend && npm run dev & cd $(CURDIR)/backend && cargo watch -c -w src -x run
run-be:
	cd $(CURDIR)/backend && cargo watch -c -w src -x run