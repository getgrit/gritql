#include <node.h>
#include <v8.h>
#include "./language.h"
#include "./node.h"
#include "./parser.h"
#include "./query.h"
#include "./tree.h"
#include "./tree_cursor.h"
#include "./conversions.h"

namespace node_tree_sitter {

using namespace v8;

void InitAll(Local<Object> exports, Local<Value> m_, void* v_) {
  InitConversions(exports);
  node_methods::Init(exports);
  language_methods::Init(exports);
  Parser::Init(exports);
  Query::Init(exports);
  Tree::Init(exports);
  TreeCursor::Init(exports);
}

NODE_MODULE(tree_sitter_runtime_binding, InitAll)

}  // namespace node_tree_sitter
