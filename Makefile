.PHONY: test grammar

grammar:
	cd tree-sitter-quickbms && tree-sitter generate

test:
	cd tree-sitter-quickbms && ../scripts/test_grammar_against_examples.sh