.PHONY: regression-test update-regression-tests test-grammar grammar update-grammar-tests

regression-test:
	cargo build --release && pytest regression_tests/test.py

update-regression-tests:
	cargo build --release && python regression_tests/test.py

grammar:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter generate

test-grammar:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter test

update-grammar-tests:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter test --update