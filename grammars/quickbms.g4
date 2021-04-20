grammar quickbms;

// Case insensitivity schenanigans
// https://github.com/antlr/antlr4/blob/master/doc/case-insensitive-lexing.md
fragment A: [aA]; // match either an 'a' or 'A'
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

// The actual grammar
script: statement;
statement: print_statement;
expression: STRING;

print_statement: PRINT expression;
//print_statement: P R I N T;

STRING: '"' [a-zA-Z0-9]* '"';
PRINT: P R I N T;
WS: [ \t\r\n]+ -> skip;