/*

This is the linearization algorithm for converting a list of rewrite effects into a unified rewrite.

The rewrites are represented as a mapping:

## Data structure sketch

Binding -> (StructureToInline, UniqueVariableIdentifier -> BindingOrPattern)

- Binding represents a location in code (see signature) and
- StructureToInline represents a structure to inline at that location.
  This is represented as a Pattern, but only allowing, for now:
   - a `someTextWith $uniqueVariableIdentifiers001`, or
   - a constructed list containing other StructureToInline items
- UniqueVariableIdentifier represents a unique variable identifier that is used in the StructureToInline.
- BindingOrPattern represents either a Binding or a Pattern, depending on whether we take the `binding` or `assigned` of a variable.
  The pattern needs to be a StructureToInline as well.

## Algorithm

We resolve each binding idependently. The final result is a mapping from Ranges to Strings.

For each binding, we resolve the inner bindings recursively:
1. We take the code range of the binding.
2. We look in the global mapping for any bindings that overlap with this range.
3. We resolve the inner bindings recursively.
4. We replace the binding with the resolved inner bindings.

The algorithm incrementally constructs a data structure that represents the final result, the mapping from Ranges to Strings.

The algorithm also lends itself to incrementality.
When we have a new effect, we can invalidate only the effects which it contain (i.e., their Range contains the Range of) the new effect.
We do not care about ranges which are contained in the new effect -- see the note about conflicting rewrites below.

This is just a sketch. There are still decisions to be made regarding the data structures and exact algorithm considering borrowing contraints.

## What about conflicting rewrites?

We assume that outer rewrites (i.e., rewrites that have a larger range) take precedence over inner rewrites.

Considering that we do not have partial list rewrites, we can assume effect Ranges are either nested or disjoint. They cannot partially overlap.

## How do we obtain the input data structure

The input data structure is computed on the fly during rewriting.
Regular variables are replaced with the UniqueVariableIdentifiers, and we remember the Binding or Pattern from `binding` or `assigned` at the time of the effect.

## Does this allow incremental rewrites?

Yes. We might not implement it at first but the algorithm is incremental by design.

We can incrementally compute the rewrite output, including only computing particular code ranges.
This is useful for future functionality, such as `guess`.

## What are downsides of this approach?

The RHS of rewrite effects are not AST nodes, they are just strings.
This means we do not have a good way of checking that the RHS is valid code

## Example

Say we want to rewrite the following React class to its hooks equivalent:

```
class Foo extends Component {
  bar() { }
  render() {
    return <div>Hello</div>;
  }
}
```

We want to rewrite it to:
```
const Foo = function() {
  bar() { }
  return <div>Hello</div>;
}
```

Let's say we have the following pattern:
```
`class $className extends Component { $body }` => `const $className = function() { $body }` where {
    $body <: contains `render() { $renderBody }` => $renderBody
}
```

We will have the following two effects:
- theBindingMatchingTheEntireClass -> (
    `const $className001 = function() { $body001 }`,
    {
        $className001 -> theBindingOfTheFooIdentifier,
        $body001 -> theBindingOfTheClassBody
    }
  )
- theBindingOfTheRenderMethod -> (
    `$renderBody`,
    {
        $x -> theBindingOfTheRenderBody
    }
  )
)

Note that the second effect is nested inside the first effect.

The rewrite is only preserved because it is inside the range of the $body variable on the RHS of the first effect.
More precisely, we see the second effect because it is inside the range of `theBindingOfTheClassBody`, which is part of the first effect.

See the "What about conflicting rewrites?" section above.

*/
