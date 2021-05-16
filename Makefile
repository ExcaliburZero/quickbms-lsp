.PHONY: test grammar

grammar:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && tree-sitter generate

test:
	export PATH=$$PATH:./node_modules/.bin && cd tree-sitter-quickbms && ../scripts/test_grammar_against_examples.sh