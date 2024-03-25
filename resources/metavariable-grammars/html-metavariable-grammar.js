/**
 * @file HTML grammar for tree-sitter
 * @author Max Brunsfeld
 * @license MIT
 */

/* eslint-disable arrow-parens */
/* eslint-disable camelcase */
/* eslint-disable-next-line spaced-comment */
/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
    name: 'html',
  
    extras: $ => [
      $.comment,
      /\s+/,
    ],
  
    externals: $ => [
      $._start_tag_name,
      $._script_start_tag_name,
      $._style_start_tag_name,
      $._end_tag_name,
      $.erroneous_end_tag_name,
      '/>',
      $._implicit_end_tag,
      $.raw_text,
      $.comment,
    ],
  
    rules: {
      document: $ => field('nodes', repeat($._node)),
  
      doctype: $ => seq(
        '<!',
        alias($._doctype, 'doctype'),
        /[^>]+/,
        '>',
      ),
  
      _doctype: _ => /[Dd][Oo][Cc][Tt][Yy][Pp][Ee]/,
  
      _node: $ => choice(
        $.doctype,
        $.entity,
        $.text,
        $.element,
        $.script_element,
        $.style_element,
        $.erroneous_end_tag,
      ),
  
      element: $ => choice(
        field('content', $.element_content),
        field('content', $.self_closing_tag),
      ),
  
      script_element: $ => seq(
        field('start_tag', alias($.script_start_tag, $.start_tag)),
        field('body', optional($.raw_text)),
        field('end_tag', $.end_tag),
      ),
  
      style_element: $ => seq(
        field('start_tag', alias($.style_start_tag, $.start_tag)),
        field('body', optional($.raw_text)),
        field('end_tag', $.end_tag),
      ),
  
      start_tag: $ => seq(
        '<',
        field('name', choice($.grit_metavariable, alias($._start_tag_name, $.tag_name))),
        field('props', repeat($.attribute)),
        '>',
      ),
  
      script_start_tag: $ => seq(
        '<',
        field('name', alias($._script_start_tag_name, $.tag_name)),
        field('props', repeat($.attribute)),
        '>',
      ),
  
      style_start_tag: $ => seq(
        '<',
        field('name', alias($._style_start_tag_name, $.tag_name)),
        field('props', repeat($.attribute)),
        '>',
      ),

      element_content: $ => seq(
        field('start_tag', $.start_tag),
        field('body', repeat($._node)),
        field('end_tag', choice($.end_tag, $._implicit_end_tag)),
      ),
  
      self_closing_tag: $ => seq(
        '<',
        field('name', choice($.grit_metavariable, alias($._start_tag_name, $.tag_name))),
        field('props', repeat($.attribute)),
        '/>',
      ),
  
      end_tag: $ => seq(
        '</',
        field('name', choice($.grit_metavariable, alias($._end_tag_name, $.tag_name))),
        '>',
      ),
  
      erroneous_end_tag: $ => seq(
        '</',
        field('name', $.erroneous_end_tag_name),
        '>',
      ),
  
      attribute: $ => seq(
        field('name', $.attribute_name),
        optional(seq(
          '=',
          field('value', choice(
            $.attribute_value,
            $.quoted_attribute_value,
          )),
        )),
      ),
  
      attribute_name: $ => choice($.grit_metavariable, /[^<>"'/=\s]+/),
  
      attribute_value: $ => choice($.grit_metavariable, /[^<>"'=\s]+/),
  
      // An entity can be named, numeric (decimal), or numeric (hexacecimal). The
      // longest entity name is 29 characters long, and the HTML spec says that
      // no more will ever be added.
      entity: _ => /&(#([xX][0-9a-fA-F]{1,6}|[0-9]{1,5})|[A-Za-z]{1,30});?/,
  
      quoted_attribute_value: $ => choice(
        seq('\'', field('value', optional(alias(/[^']+/, $.attribute_value))), '\''),
        seq('"', field('value', optional(alias(/[^"]+/, $.attribute_value))), '"'),
      ),
  
      text: _ => /[^<>&\s]([^<>&]*[^<>&\s])?/,

      grit_metavariable: ($) => token(prec(100, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
    },
  });
  