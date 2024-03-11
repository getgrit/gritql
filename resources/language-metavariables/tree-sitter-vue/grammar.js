module.exports = grammar({
  name: "vue",

  externals: $ => [
    $._text_fragment,
    $._interpolation_text,
    $._start_tag_name,
    $._template_start_tag_name,
    $._script_start_tag_name,
    $._style_start_tag_name,
    $._end_tag_name,
    $.erroneous_end_tag_name,
    "/>",
    $._implicit_end_tag,
    $.raw_text,
    $.comment,
  ],

  extras: $ => [/\s+/],

  rules: {
    component: $ => repeat(
      choice(
        $.comment,
        $.element,
        $.template_element,
        $.script_element,
        $.style_element,
      ),
    ),

    _node: $ => choice(
      $.comment,
      $.text,
      $.interpolation,
      $.element,
      $.template_element,
      $.script_element,
      $.style_element,
      $.erroneous_end_tag,
    ),

    element: $ => choice(
      seq(
        $.start_tag,
        repeat($._node),
        choice($.end_tag, $._implicit_end_tag),
      ),
      $.self_closing_tag,
    ),

    template_element: $ => seq(
      alias($.template_start_tag, $.start_tag),
      repeat($._node),
      $.end_tag,
    ),

    script_element: $ => seq(
      alias($.script_start_tag, $.start_tag),
      optional($.raw_text),
      $.end_tag,
    ),

    style_element: $ => seq(
      alias($.style_start_tag, $.start_tag),
      optional($.raw_text),
      $.end_tag,
    ),

    start_tag: $ => seq(
      "<",
      alias($._start_tag_name, $.tag_name),
      repeat(choice($.attribute, $.directive_attribute)),
      ">",
    ),

    template_start_tag: $ => seq(
      "<",
      alias($._template_start_tag_name, $.tag_name),
      repeat(choice($.attribute, $.directive_attribute)),
      ">",
    ),

    script_start_tag: $ => seq(
      "<",
      alias($._script_start_tag_name, $.tag_name),
      repeat(choice($.attribute, $.directive_attribute)),
      ">",
    ),

    style_start_tag: $ => seq(
      "<",
      alias($._style_start_tag_name, $.tag_name),
      repeat(choice($.attribute, $.directive_attribute)),
      ">",
    ),

    self_closing_tag: $ => seq(
      "<",
      alias($._start_tag_name, $.tag_name),
      repeat(choice($.attribute, $.directive_attribute)),
      "/>",
    ),

    end_tag: $ => seq(
      "</",
      alias($._end_tag_name, $.tag_name),
      ">",
    ),

    erroneous_end_tag: $ => seq(
      "</",
      $.erroneous_end_tag_name,
      ">",
    ),

    attribute: $ => seq(
      $.attribute_name,
      optional(seq(
        "=",
        choice(
          $.attribute_value,
          $.quoted_attribute_value,
        ),
      )),
    ),

    attribute_name: $ => /[^<>"'=/\s]+/,

    attribute_value: $ => /[^<>"'=\s]+/,

    quoted_attribute_value: $ =>
      choice(
        seq("'", optional(alias(/[^']+/, $.attribute_value)), "'"),
        seq('"', optional(alias(/[^"]+/, $.attribute_value)), '"'),
      ),

    text: $ => choice($._text_fragment, "{{"),

    interpolation: $ => seq(
      "{{",
      optional(alias($._interpolation_text, $.raw_text)),
      "}}",
    ),

    directive_attribute: $ =>
      seq(
        choice(
          seq(
            $.directive_name,
            optional(seq(
              token.immediate(prec(1, ":")),
              choice($.directive_argument, $.directive_dynamic_argument),
            )),
          ),
          seq(
            alias($.directive_shorthand, $.directive_name),
            choice($.directive_argument, $.directive_dynamic_argument),
          ),
        ),
        optional($.directive_modifiers),
        optional(seq("=", choice($.attribute_value, $.quoted_attribute_value))),
      ),
    directive_name: $ => token(prec(1, /v-[^<>'"=/\s:.]+/)),
    directive_shorthand: $ => token(prec(1, choice(":", "@", "#"))),
    directive_argument: $ => token.immediate(/[^<>"'/=\s.]+/),
    directive_dynamic_argument: $ => seq(
      token.immediate(prec(1, "[")),
      optional($.directive_dynamic_argument_value),
      token.immediate("]"),
    ),
    directive_dynamic_argument_value: $ => token.immediate(/[^<>"'/=\s\]]+/),
    directive_modifiers: $ => repeat1(seq(token.immediate(prec(1, ".")), $.directive_modifier)),
    directive_modifier: $ => token.immediate(/[^<>"'/=\s.]+/),
  },
});
