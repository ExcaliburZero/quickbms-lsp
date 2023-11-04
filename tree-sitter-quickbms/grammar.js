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
      $.for_statement,
      $.break_statement,
      $.continue_statement,
      $.goto_statement,
      $.cleanexit_statement,
      $.findloc_statement,
      $.get_statement,
      $.math_statement,
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
    for_statement: $ => seq(
      $.for,
      optional(seq(
        field("variable", $._expression),
        field("operation", $.operation),
        field("value", $._expression),
        field("comparison", $.comparison),
        field("right_expression", $._expression),
      )),
      field("body", repeat($._statement)),
      $.next_statement,
    ),
    next_statement: $ => seq(
      $.next,
      optional(seq(
        field("variable", $._expression),
        optional(seq(
          field("operation", $.operation),
          field("value", $._expression),
        )),
      )),
    ),
    break_statement: $ => seq(
      $.break,
      field("label", optional($._expression)),
    ),
    continue_statement: $ => seq(
      $.continue,
      field("label", optional($._expression)),
    ),
    goto_statement: $ => seq(
      $.goto,
      field("offset", $._expression),
      field("file_num", optional($._expression)),
      field("type", optional($._goto_type)),
    ),
    cleanexit_statement: $ => seq(
      $.cleanexit
    ),
    findloc_statement: $ => seq(
      $.findloc,
      field("variable", $._expression),
      field("type", $.type),
      field("string", $._expression),
      optional(seq(
        field("file_num", $._expression),
        optional(seq(
          field("error_value", $._expression),
          optional(seq(
            field("end_off", $._expression),
          )),
        )),
      )),
    ),
    get_statement: $ => seq(
      $.get,
      field("variable", $._expression),
      field("type", $.type),
      field("file_num", optional($._expression)),
    ),
    math_statement: $ => seq(
      $.math,
      field("variable", $._expression),
      field("operation", $.operation),
      field("expression", $._expression),
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
    operation: $ => choice(
      "=",
      "+=",
      "/=",
      "*",
      "<<",
      "u<<",
      "~",
      "n",
      "a",
      "u/",
      "v",
    ),
    _goto_type: $ => choice(
      $.seek_set,
      $.seek_cur,
      $.seek_end,
    ),
    type: $ => choice(
      $.long,
      $.asize,
      $.string,
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
    asize: $ => /[Aa][Ss][Ii][Zz][Ee]/,
    string: $ => /[Ss][Tt][Rr][Ii][Nn][Gg]/,
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
    for: $ => /[Ff][Oo][Rr]/,
    next: $ => /[Nn][Ee][Xx][Tt]/,
    break: $ => /[Bb][Rr][Ee][Aa][Kk]/,
    continue: $ => /[Cc][Oo][Nn][Tt][Ii][Nn][Uu][Ee]/,
    goto: $ => /[Gg][Oo][Tt][Oo]/,
    seek_set: $ => /[Ss][Ee][Ee][Kk]_[Ss][Ee][Tt]/,
    seek_cur: $ => /[Ss][Ee][Ee][Kk]_[Cc][Uu][Rr]/,
    seek_end: $ => /[Ss][Ee][Ee][Kk]_[Ee][Nn][Dd]/,
    cleanexit: $ => /[Cc][Ll][Ee][Aa][Nn][Ee][Xx][Ii][Tt]/,
    findloc: $ => /[Ff][Ii][Nn][Dd][Ll][Oo][Cc]/,
    get: $ => /[Gg][Ee][Tt]/,
    math: $ => /[Mm][Aa][Tt][Hh]/,

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

