.PHONY: test grammar update-tests

grammar:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter generate

test:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter test

update-tests:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter test --update