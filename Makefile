LIB = lib
RUST_ANTLR_URL = https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.2/antlr4-4.8-2-SNAPSHOT-complete.jar
RUST_ANTLR_JAR = $(LIB)/antlr4_with_rust_support.jar
GRUN = java org.antlr.v4.gui.TestRig

# Add the antlr jar to the classpath, so that we can run grun
export CLASSPATH=.:$(PWD)/$(RUST_ANTLR_JAR):$(echo CLASSPATH)

.PHONY: test grammar

grammar: $(RUST_ANTLR_JAR)
	java -jar $(RUST_ANTLR_JAR) -Dlanguage=Rust src/grammar/quickbms.g4 -visitor

test: $(RUST_ANTLR_JAR)
	echo $$CLASSPATH
	java -jar $(RUST_ANTLR_JAR) src/grammar/quickbms.g4
	cd src/grammar; javac *.java
	cd src/grammar && ../../scripts/test_grammar_against_examples.sh "$(GRUN)"

$(RUST_ANTLR_JAR): $(LIB)
	curl -L $(RUST_ANTLR_URL) > $(RUST_ANTLR_JAR)

$(LIB):
	mkdir -p $(LIB)
