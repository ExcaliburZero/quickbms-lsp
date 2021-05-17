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

    print: $ => seq(
      $._P, $._R, $._I, $._N, $._T
    ),
    set: $ => seq(
      $._S, $._E, $._T
    ),
    startfunction: $ => seq(
      $._S, $._T, $._A, $._R, $._T, $._F, $._U, $._N, $._C, $._T, $._I, $._O, $._N
    ),
    endfunction: $ => seq(
      $._E, $._N, $._D, $._F, $._U, $._N, $._C, $._T, $._I, $._O, $._N
    ),
    callfunction: $ => seq(
      $._C, $._A, $._L, $._L, $._F, $._U, $._N, $._C, $._T, $._I, $._O, $._N
    ),
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

    // Case insensitivity support since keywords, variables, funcitons, etc. are case insensitive.
    _A: $ => /[aA]/,
    _B: $ => /[bB]/,
    _C: $ => /[cC]/,
    _D: $ => /[dD]/,
    _E: $ => /[eE]/,
    _F: $ => /[fF]/,
    _G: $ => /[gG]/,
    _H: $ => /[hH]/,
    _I: $ => /[iI]/,
    _J: $ => /[jJ]/,
    _K: $ => /[kK]/,
    _L: $ => /[lL]/,
    _M: $ => /[mM]/,
    _N: $ => /[nN]/,
    _O: $ => /[oO]/,
    _P: $ => /[pP]/,
    _Q: $ => /[qQ]/,
    _R: $ => /[rR]/,
    _S: $ => /[sS]/,
    _T: $ => /[tT]/,
    _U: $ => /[uU]/,
    _V: $ => /[vV]/,
    _W: $ => /[wW]/,
    _X: $ => /[xX]/,
    _Y: $ => /[yY]/,
    _Z: $ => /[zZ]/,
  }
});

