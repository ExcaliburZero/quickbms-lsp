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
      $.endian_statement,
      $.idstring_statement,
      $.if_statement,
      $.goto_statement,
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
    endian_statement: $ => seq(
      $.endian,
      field("type", $._endian_type),
      field("value", optional($._expression)),
    ),
    idstring_statement: $ => seq(
      $.idstring,
      field("filenum", optional($._expression)),
      field("magic", $.string_literal),
    ),
    if_statement: $ => seq(
      $.if,
      field("left_expression", $._expression),
      field("comparison", $.comparison),
      field("right_expression", $._expression),
      field("body", repeat($._statement)),
      field("else_clauses", repeat(choice(
        $.elif_statement,
        $.else_statement
      ))),
      $.endif
    ),
    elif_statement: $ => seq(
      $.elif,
      field("left_expression", $._expression),
      field("comparison", $.comparison),
      field("right_expression", $._expression),
      field("body", repeat($._statement)),
    ),
    else_statement: $ => seq(
      $.else,
      field("body", repeat($._statement)),
    ),
    goto_statement: $ => seq(
      $.goto,
      field("offset", $._expression),
      field("file_num", optional($._expression)),
      field("type", optional($._goto_type)),
    ),
    comparison: $ => choice(
      "<",
      ">",
      "!=",
      "<>",
      "!==",
      "==",
      "=",
      "==="
    ),
    _goto_type: $ => choice(
      $.seek_set,
      $.seek_cur,
      $.seek_end,
    ),
    type: $ => choice(
      $.long
    ),
    _endian_type: $ => choice(
      $.little,
      $.intel,
      $.big,
      $.network,
      $.swap,
      $.change,
      $.invert,
      $.guess,
      $.guess16,
      $.guess64,
      $.guess24,
      $.save,
      $.store
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
    endian: $ => /[Ee][Nn][Dd][Ii][Aa][Nn]/,
    little: $ => /[Ll][Ii][Tt][Tt][Ll][Ee]/,
    intel: $ => /[Ii][Nn][Tt][Ee][Ll]/,
    big: $ => /[Bb][Ii][Gg]/,
    network: $ => /[Nn][Ee][Tt][Ww][Oo][Rr][Kk]/,
    swap: $ => /[Ss][Ww][Aa][Pp]/,
    change: $ => /[Cc][Hh][Aa][Nn][Gg][Ee]/,
    invert: $ => /[Ii][Nn][Vv][Ee][Rr][Tt]/,
    guess: $ => /[Gg][Uu][Ee][Ss][Ss]/,
    guess16: $ => /[Gg][Uu][Ee][Ss][Ss]16/,
    guess64: $ => /[Gg][Uu][Ee][Ss][Ss]64/,
    guess24: $ => /[Gg][Uu][Ee][Ss][Ss]24/,
    save: $ => /[Ss][Aa][Vv][Ee]/,
    store: $ => /[Ss][Tt][Oo][Rr][Ee]/,
    idstring: $ => /[Ii][Dd][Ss][Tt][Rr][Ii][Nn][Gg]/,
    if: $ => /[Ii][Ff]/,
    elif: $ => /[Ee][Ll][Ii][Ff]/,
    else: $ => /[Ee][Ll][Ss][Ee]/,
    endif: $ => /[Ee][Nn][Dd][Ii][Ff]/,
    goto: $ => /[Gg][Oo][Tt][Oo]/,
    seek_set: $ => /[Ss][Ee][Ee][Kk]_[Ss][Ee][Tt]/,
    seek_cur: $ => /[Ss][Ee][Ee][Kk]_[Cc][Uu][Rr]/,
    seek_end: $ => /[Ss][Ee][Ee][Kk]_[Ee][Nn][Dd]/,

    identifier: $ => /[a-zA-Z_]+[a-zA-Z0-9_\-]*/,
    integer_literal: $ => /((0x)?[0-9a-fA-F]+)|(\-?[0-9]+)/,
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

