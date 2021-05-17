module.exports = grammar({
  name: 'quickbms',

  extras: $ => [
    /\s|\\\r?\n/,
    $.comment,
  ],

  rules: {
    source_file: $ => repeat($._top_statement),
    _top_statement: $ => choice(
      $._statement,
      $.function_declaration
    ),
    function_declaration: $ => seq(
      $.startfunction,
      field("name", $.identifier),
      field("body", repeat($._statement)),
      $.endfunction,
    ),
    _statement: $ => choice(
      $.print_statement,
      $.set_statement,
      $.function_call_statement,
    ),
    set_statement: $ => seq(
      $.set,
      field("variable", $.identifier),
      field("type", optional($.type)),
      field("value", $._expression),
    ),
    print_statement: $ => seq(
      $.print,
      field("value", $._expression)
    ),
    function_call_statement: $ => seq(
      $.callfunction,
      field("name", $.identifier),
    ),
    type: $ => choice(
      $.long
    ),
    _expression: $ => choice(
      $.string_literal,
      $.integer_literal,
      $.identifier,
    ),

    print: $ => /[Pp][Rr][Ii][Nn][Tt]/,
    set: $ => /[Ss][Ee][Tt]/,
    startfunction: $ => /[Ss][Tt][Aa][Rr][Tt][Ff][Uu][Nn][Cc][Tt][Ii][Oo][Nn]/,
    endfunction: $ => /[Ee][Nn][Dd][Ff][Uu][Nn][Cc][Tt][Ii][Oo][Nn]/,
    callfunction: $ => /[Cc][Aa][Ll][Ll][Ff][Uu][Nn][Cc][Tt][Ii][Oo][Nn]/,
    long: $ => /[Ll][Oo][Nn][Gg]/,

    identifier: $ => /[a-zA-Z]+[a-zA-Z0-9]*/,
    integer_literal: $ => /(0x)?[0-9a-fA-F]+/,
    string_literal: $ => seq(
      '"',
      repeat(token.immediate(prec(1, /[^\\"\n]+/))),
      '"',
    ),

    comment: $ => token(choice(
      seq('#', /(\\(.|\r?\n)|[^\\\n])*/),
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/'
      )
    )),
  }
});

