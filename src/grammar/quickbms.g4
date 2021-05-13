grammar quickbms;

@tokenfactory {
	pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

// Case insensitivity schenanigans
// https://github.com/antlr/antlr4/blob/master/doc/case-insensitive-lexing.md
fragment A:[aA]; // match either an 'a' or 'A'
fragment B: [bB];
fragment C: [cC];
fragment D: [dD];
fragment E: [eE];
fragment F: [fF];
fragment G: [gG];
fragment H: [hH];
fragment I: [iI];
fragment J: [jJ];
fragment K: [kK];
fragment L: [lL];
fragment M: [mM];
fragment N: [nN];
fragment O: [oO];
fragment P: [pP];
fragment Q: [qQ];
fragment R: [rR];
fragment S: [sS];
fragment T: [tT];
fragment U: [uU];
fragment V: [vV];
fragment W: [wW];
fragment X: [xX];
fragment Y: [yY];
fragment Z: [zZ];

// based on: https://github.com/antlr/grammars-v4/blob/f69c762c6d67eb5324d55c82129748454adb145c/c/C.g4#L830
fragment CHARACTER:
	~["\\\r\n]
	//| ESCAPE_SEQUENCE // TODO: implement this
	| '\\\n'
	| '\\\r\n';

// Grammar rules
script: top_statement*;
top_statement: statement | function_definition;
function_definition:
	START_FUNCTION function statement* END_FUNCTION;
statement:
	PRINT expression						# print_statement
	| SET variable type_name? expression	# set_statement
	| CALL_FUNCTION function				# function_call_statement;
expression:
	STRING_LITERAL		# string_literal
	| INTEGER_LITERAL	# integer_literal;
type_name: LONG # long_type;
variable: ID;
function: ID;

// Tokens
STRING_LITERAL: '"' CHARACTER*? '"';
INTEGER_LITERAL: [0-9]+;
PRINT: P R I N T;
SET: S E T;
LONG: L O N G;
START_FUNCTION: S T A R T F U N C T I O N;
END_FUNCTION: E N D F U N C T I O N;
CALL_FUNCTION: C A L L F U N C T I O N;
ID: [a-zA-Z0-9_]+;
WS: [ \t\r\n]+ -> skip;
LINE_COMMENT_NUMBER_SIGN: '#' .+? ('\n' | EOF) -> skip;
LINE_COMMENT_DOUBLE_SLASH: '//' .+? ('\n' | EOF) -> skip;
BLOCK_COMMENT: '/*' .+? '*/' -> skip;
