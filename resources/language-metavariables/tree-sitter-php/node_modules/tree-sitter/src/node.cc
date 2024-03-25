#include "./node.h"
#include <nan.h>
#include <tree_sitter/api.h>
#include <vector>
#include <v8.h>
#include "./util.h"
#include "./conversions.h"
#include "./tree.h"
#include "./tree_cursor.h"

namespace node_tree_sitter {
namespace node_methods {

using std::vector;
using namespace v8;

static const uint32_t FIELD_COUNT_PER_NODE = 6;

static uint32_t *transfer_buffer = nullptr;
static uint32_t transfer_buffer_length = 0;
static Nan::Persistent<Object> module_exports;
static TSTreeCursor scratch_cursor = {nullptr, nullptr, {0, 0}};

static inline void setup_transfer_buffer(uint32_t node_count) {
  uint32_t new_length = node_count * FIELD_COUNT_PER_NODE;
  if (new_length > transfer_buffer_length) {
    if (transfer_buffer) {
      free(transfer_buffer);
    }
    transfer_buffer_length = new_length;
    transfer_buffer = static_cast<uint32_t *>(malloc(transfer_buffer_length * sizeof(uint32_t)));

    #if defined(_MSC_VER) && NODE_RUNTIME_ELECTRON && NODE_MODULE_VERSION >= 89
      auto nodeBuffer = node::Buffer::New(Isolate::GetCurrent(), (char *)transfer_buffer, transfer_buffer_length * sizeof(uint32_t), [](char *data, void *hint) {}, nullptr)
        .ToLocalChecked()
        .As<v8::TypedArray>();
      v8::Local<v8::ArrayBuffer> js_transfer_buffer = nodeBuffer.As<v8::TypedArray>()->Buffer();
    #elif V8_MAJOR_VERSION < 8 || (V8_MAJOR_VERSION == 8 && V8_MINOR_VERSION < 4) || (defined(_MSC_VER) && NODE_RUNTIME_ELECTRON)
      auto js_transfer_buffer = ArrayBuffer::New(Isolate::GetCurrent(), transfer_buffer, transfer_buffer_length * sizeof(uint32_t));
    #else
      auto backing_store = ArrayBuffer::NewBackingStore(transfer_buffer, transfer_buffer_length * sizeof(uint32_t), BackingStore::EmptyDeleter, nullptr);
      auto js_transfer_buffer = ArrayBuffer::New(Isolate::GetCurrent(), std::move(backing_store));
    #endif

    Nan::Set(
      Nan::New(module_exports),
      Nan::New("nodeTransferArray").ToLocalChecked(),
      Uint32Array::New(js_transfer_buffer, 0, transfer_buffer_length)
    );
  }
}

static inline bool operator<=(const TSPoint &left, const TSPoint &right) {
  if (left.row < right.row) return true;
  if (left.row > right.row) return false;
  return left.column <= right.column;
}

static void MarshalNodes(const Nan::FunctionCallbackInfo<Value> &info,
                         const Tree *tree, const TSNode *nodes, uint32_t node_count) {
  info.GetReturnValue().Set(GetMarshalNodes(info, tree, nodes, node_count));
}

void MarshalNode(const Nan::FunctionCallbackInfo<Value> &info, const Tree *tree, TSNode node) {
  info.GetReturnValue().Set(GetMarshalNode(info, tree, node));
}

Local<Value> GetMarshalNodes(const Nan::FunctionCallbackInfo<Value> &info,
                         const Tree *tree, const TSNode *nodes, uint32_t node_count) {
  auto result = Nan::New<Array>();
  setup_transfer_buffer(node_count);
  uint32_t *p = transfer_buffer;
  for (unsigned i = 0; i < node_count; i++) {
    TSNode node = nodes[i];
    const auto &cache_entry = tree->cached_nodes_.find(node.id);
    if (cache_entry == tree->cached_nodes_.end()) {
      MarshalNodeId(node.id, p);
      p += 2;
      *(p++) = node.context[0];
      *(p++) = node.context[1];
      *(p++) = node.context[2];
      *(p++) = node.context[3];
      if (node.id) {
        Nan::Set(result, i, Nan::New(ts_node_symbol(node)));
      } else {
        Nan::Set(result, i, Nan::Null());
      }
    } else {
      Nan::Set(result, i, Nan::New(cache_entry->second->node));
    }
  }
  return result;
}

Local<Value> GetMarshalNode(const Nan::FunctionCallbackInfo<Value> &info, const Tree *tree, TSNode node) {
  const auto &cache_entry = tree->cached_nodes_.find(node.id);
  if (cache_entry == tree->cached_nodes_.end()) {
    setup_transfer_buffer(1);
    uint32_t *p = transfer_buffer;
    MarshalNodeId(node.id, p);
    p += 2;
    *(p++) = node.context[0];
    *(p++) = node.context[1];
    *(p++) = node.context[2];
    *(p++) = node.context[3];
    if (node.id) {
      return Nan::New(ts_node_symbol(node));
    }
  } else {
    return Nan::New(cache_entry->second->node);
  }
  return Nan::Null();
}

void MarshalNullNode() {
  memset(transfer_buffer, 0, FIELD_COUNT_PER_NODE * sizeof(transfer_buffer[0]));
}

TSNode UnmarshalNode(const Tree *tree) {
  TSNode result = {{0, 0, 0, 0}, nullptr, nullptr};
  result.tree = tree->tree_;
  if (!result.tree) {
    Nan::ThrowTypeError("Argument must be a tree");
    return result;
  }

  result.id = UnmarshalNodeId(&transfer_buffer[0]);
  result.context[0] = transfer_buffer[2];
  result.context[1] = transfer_buffer[3];
  result.context[2] = transfer_buffer[4];
  result.context[3] = transfer_buffer[5];
  return result;
}

static void ToString(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    const char *string = ts_node_string(node);
    info.GetReturnValue().Set(Nan::New(string).ToLocalChecked());
    free((char *)string);
  }
}

static void IsMissing(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    bool result = ts_node_is_missing(node);
    info.GetReturnValue().Set(Nan::New<Boolean>(result));
  }
}

static void HasChanges(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    bool result = ts_node_has_changes(node);
    info.GetReturnValue().Set(Nan::New<Boolean>(result));
  }
}

static void HasError(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    bool result = ts_node_has_error(node);
    info.GetReturnValue().Set(Nan::New<Boolean>(result));
  }
}

static void FirstNamedChildForIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    Nan::Maybe<uint32_t> byte = ByteCountFromJS(info[1]);
    if (byte.IsJust()) {
      MarshalNode(info, tree, ts_node_first_named_child_for_byte(node, byte.FromJust()));
      return;
    }
  }
  MarshalNullNode();
}

static void FirstChildForIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id && info.Length() > 1) {
    Nan::Maybe<uint32_t> byte = ByteCountFromJS(info[1]);
    if (byte.IsJust()) {
      MarshalNode(info, tree, ts_node_first_child_for_byte(node, byte.FromJust()));
      return;
    }
  }
  MarshalNullNode();
}

static void NamedDescendantForIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    Nan::Maybe<uint32_t> maybe_min = ByteCountFromJS(info[1]);
    Nan::Maybe<uint32_t> maybe_max = ByteCountFromJS(info[2]);
    if (maybe_min.IsJust() && maybe_max.IsJust()) {
      uint32_t min = maybe_min.FromJust();
      uint32_t max = maybe_max.FromJust();
      MarshalNode(info, tree, ts_node_named_descendant_for_byte_range(node, min, max));
      return;
    }
  }
  MarshalNullNode();
}

static void DescendantForIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    Nan::Maybe<uint32_t> maybe_min = ByteCountFromJS(info[1]);
    Nan::Maybe<uint32_t> maybe_max = ByteCountFromJS(info[2]);
    if (maybe_min.IsJust() && maybe_max.IsJust()) {
      uint32_t min = maybe_min.FromJust();
      uint32_t max = maybe_max.FromJust();
      MarshalNode(info, tree, ts_node_descendant_for_byte_range(node, min, max));
      return;
    }
  }
  MarshalNullNode();
}

static void NamedDescendantForPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    Nan::Maybe<TSPoint> maybe_min = PointFromJS(info[1]);
    Nan::Maybe<TSPoint> maybe_max = PointFromJS(info[2]);
    if (maybe_min.IsJust() && maybe_max.IsJust()) {
      TSPoint min = maybe_min.FromJust();
      TSPoint max = maybe_max.FromJust();
      MarshalNode(info, tree, ts_node_named_descendant_for_point_range(node, min, max));
      return;
    }
  }
  MarshalNullNode();
}

static void DescendantForPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    Nan::Maybe<TSPoint> maybe_min = PointFromJS(info[1]);
    Nan::Maybe<TSPoint> maybe_max = PointFromJS(info[2]);
    if (maybe_min.IsJust() && maybe_max.IsJust()) {
      TSPoint min = maybe_min.FromJust();
      TSPoint max = maybe_max.FromJust();
      MarshalNode(info, tree, ts_node_descendant_for_point_range(node, min, max));
      return;
    }
  }
  MarshalNullNode();
}

static void Type(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    const char *result = ts_node_type(node);
    info.GetReturnValue().Set(Nan::New(result).ToLocalChecked());
  }
}

static void TypeId(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    TSSymbol result = ts_node_symbol(node);
    info.GetReturnValue().Set(Nan::New(result));
  }
}

static void IsNamed(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    bool result = ts_node_is_named(node);
    info.GetReturnValue().Set(Nan::New(result));
  }
}

static void StartIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    int32_t result = ts_node_start_byte(node) / 2;
    info.GetReturnValue().Set(Nan::New<Integer>(result));
  }
}

static void EndIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    int32_t result = ts_node_end_byte(node) / 2;
    info.GetReturnValue().Set(Nan::New<Integer>(result));
  }
}

static void StartPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    TransferPoint(ts_node_start_point(node));
  }
}

static void EndPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    TransferPoint(ts_node_end_point(node));
  }
}

static void Child(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    if (!info[1]->IsUint32()) {
      Nan::ThrowTypeError("Second argument must be an integer");
      return;
    }
    uint32_t index = Nan::To<uint32_t>(info[1]).FromJust();
    MarshalNode(info, tree, ts_node_child(node, index));
    return;
  }
  MarshalNullNode();
}

static void NamedChild(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    if (!info[1]->IsUint32()) {
      Nan::ThrowTypeError("Second argument must be an integer");
      return;
    }
    uint32_t index = Nan::To<uint32_t>(info[1]).FromJust();
    MarshalNode(info, tree, ts_node_named_child(node, index));
    return;
  }
  MarshalNullNode();
}

static void ChildCount(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    info.GetReturnValue().Set(Nan::New(ts_node_child_count(node)));
  }
}

static void NamedChildCount(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    info.GetReturnValue().Set(Nan::New(ts_node_named_child_count(node)));
  }
}

static void FirstChild(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_child(node, 0));
    return;
  }
  MarshalNullNode();
}

static void FirstNamedChild(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_named_child(node, 0));
    return;
  }
  MarshalNullNode();
}

static void LastChild(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    uint32_t child_count = ts_node_child_count(node);
    if (child_count > 0) {
      MarshalNode(info, tree, ts_node_child(node, child_count - 1));
      return;
    }
  }
  MarshalNullNode();
}

static void LastNamedChild(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    uint32_t child_count = ts_node_named_child_count(node);
    if (child_count > 0) {
      MarshalNode(info, tree, ts_node_named_child(node, child_count - 1));
      return;
    }
  }
  MarshalNullNode();
}

static void Parent(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_parent(node));
    return;
  }
  MarshalNullNode();
}

static void NextSibling(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_next_sibling(node));
    return;
  }
  MarshalNullNode();
}

static void NextNamedSibling(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_next_named_sibling(node));
    return;
  }
  MarshalNullNode();
}

static void PreviousSibling(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_prev_sibling(node));
    return;
  }
  MarshalNullNode();
}

static void PreviousNamedSibling(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (node.id) {
    MarshalNode(info, tree, ts_node_prev_named_sibling(node));
    return;
  }
  MarshalNullNode();
}

struct SymbolSet {
  std::basic_string<TSSymbol> symbols;
  void add(TSSymbol symbol) { symbols += symbol; }
  bool contains(TSSymbol symbol) { return symbols.find(symbol) != symbols.npos; }
};

bool symbol_set_from_js(SymbolSet *symbols, const Local<Value> &value, const TSLanguage *language) {
  if (!value->IsArray()) {
    Nan::ThrowTypeError("Argument must be a string or array of strings");
    return false;
  }

  unsigned symbol_count = ts_language_symbol_count(language);

  Local<Array> js_types = Local<Array>::Cast(value);
  for (unsigned i = 0, n = js_types->Length(); i < n; i++) {
    Local<Value> js_node_type_value;
    if (Nan::Get(js_types, i).ToLocal(&js_node_type_value)) {
      Local<String> js_node_type;
      if (Nan::To<String>(js_node_type_value).ToLocal(&js_node_type)) {
        auto length = js_node_type->Utf8Length(
          #if NODE_MAJOR_VERSION >= 12
            Isolate::GetCurrent()
          #endif
        );

        std::string node_type(length, '\0');
        js_node_type->WriteUtf8(

          // Nan doesn't wrap this functionality
          #if NODE_MAJOR_VERSION >= 12
            Isolate::GetCurrent(),
          #endif

          &node_type[0]
        );

        if (node_type == "ERROR") {
          symbols->add(static_cast<TSSymbol>(-1));
        } else {
          for (TSSymbol j = 0; j < symbol_count; j++) {
            if (node_type == ts_language_symbol_name(language, j)) {
              symbols->add(j);
            }
          }
        }

        continue;
      }
    }

    Nan::ThrowTypeError("Argument must be a string or array of strings");
    return false;
  }

  return true;
}

static void Children(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (!node.id) return;

  vector<TSNode> result;
  ts_tree_cursor_reset(&scratch_cursor, node);
  if (ts_tree_cursor_goto_first_child(&scratch_cursor)) {
    do {
      TSNode child = ts_tree_cursor_current_node(&scratch_cursor);
      result.push_back(child);
    } while (ts_tree_cursor_goto_next_sibling(&scratch_cursor));
  }

  MarshalNodes(info, tree, result.data(), result.size());
}

static void NamedChildren(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (!node.id) return;

  vector<TSNode> result;
  ts_tree_cursor_reset(&scratch_cursor, node);
  if (ts_tree_cursor_goto_first_child(&scratch_cursor)) {
    do {
      TSNode child = ts_tree_cursor_current_node(&scratch_cursor);
      if (ts_node_is_named(child)) {
        result.push_back(child);
      }
    } while (ts_tree_cursor_goto_next_sibling(&scratch_cursor));
  }

  MarshalNodes(info, tree, result.data(), result.size());
}

static void DescendantsOfType(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (!node.id) return;

  SymbolSet symbols;
  if (!symbol_set_from_js(&symbols, info[1], ts_tree_language(node.tree))) return;

  TSPoint start_point = {0, 0};
  TSPoint end_point = {UINT32_MAX, UINT32_MAX};

  if (info.Length() > 2 && info[2]->IsObject()) {
    auto maybe_start_point = PointFromJS(info[2]);
    if (maybe_start_point.IsNothing()) return;
    start_point = maybe_start_point.FromJust();
  }

  if (info.Length() > 3 && info[3]->IsObject()) {
    auto maybe_end_point = PointFromJS(info[3]);
    if (maybe_end_point.IsNothing()) return;
    end_point = maybe_end_point.FromJust();
  }

  vector<TSNode> found;
  ts_tree_cursor_reset(&scratch_cursor, node);
  auto already_visited_children = false;
  while (true) {
    TSNode descendant = ts_tree_cursor_current_node(&scratch_cursor);

    if (!already_visited_children) {
      if (ts_node_end_point(descendant) <= start_point) {
        if (ts_tree_cursor_goto_next_sibling(&scratch_cursor)) {
          already_visited_children = false;
        } else {
          if (!ts_tree_cursor_goto_parent(&scratch_cursor)) break;
          already_visited_children = true;
        }
        continue;
      }

      if (end_point <= ts_node_start_point(descendant)) break;

      if (symbols.contains(ts_node_symbol(descendant))) {
        found.push_back(descendant);
      }

      if (ts_tree_cursor_goto_first_child(&scratch_cursor)) {
        already_visited_children = false;
      } else if (ts_tree_cursor_goto_next_sibling(&scratch_cursor)) {
        already_visited_children = false;
      } else {
        if (!ts_tree_cursor_goto_parent(&scratch_cursor)) break;
        already_visited_children = true;
      }
    } else {
      if (ts_tree_cursor_goto_next_sibling(&scratch_cursor)) {
        already_visited_children = false;
      } else {
        if (!ts_tree_cursor_goto_parent(&scratch_cursor)) break;
      }
    }
  }

  MarshalNodes(info, tree, found.data(), found.size());
}

static void ChildNodesForFieldId(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (!node.id) return;

  auto maybe_field_id = Nan::To<uint32_t>(info[1]);
  if (!maybe_field_id.IsJust()) {
    Nan::ThrowTypeError("Second argument must be an integer");
    return;
  }
  uint32_t field_id = maybe_field_id.FromJust();

  vector<TSNode> result;
  ts_tree_cursor_reset(&scratch_cursor, node);
  if (ts_tree_cursor_goto_first_child(&scratch_cursor)) {
    do {
      TSNode child = ts_tree_cursor_current_node(&scratch_cursor);
      if (ts_tree_cursor_current_field_id(&scratch_cursor) == field_id) {
        result.push_back(child);
      }
    } while (ts_tree_cursor_goto_next_sibling(&scratch_cursor));
  }

  MarshalNodes(info, tree, result.data(), result.size());
}

static void ChildNodeForFieldId(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);

  if (node.id) {
    auto maybe_field_id = Nan::To<uint32_t>(info[1]);
    if (!maybe_field_id.IsJust()) {
      Nan::ThrowTypeError("Second argument must be an integer");
      return;
    }
    uint32_t field_id = maybe_field_id.FromJust();
    MarshalNode(info, tree, ts_node_child_by_field_id(node, field_id));
    return;
  }
  MarshalNullNode();
}

static void Closest(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  if (!node.id) return;

  SymbolSet symbols;
  if (!symbol_set_from_js(&symbols, info[1], ts_tree_language(node.tree))) return;

  for (;;) {
    TSNode parent = ts_node_parent(node);
    if (!parent.id) break;
    if (symbols.contains(ts_node_symbol(parent))) {
      MarshalNode(info, tree, parent);
      return;
    }
    node = parent;
  }

  MarshalNullNode();
}

static void Walk(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = Tree::UnwrapTree(info[0]);
  TSNode node = UnmarshalNode(tree);
  TSTreeCursor cursor = ts_tree_cursor_new(node);
  info.GetReturnValue().Set(TreeCursor::NewInstance(cursor));
}

void Init(Local<Object> exports) {
  Local<Object> result = Nan::New<Object>();

  FunctionPair methods[] = {
    {"startIndex", StartIndex},
    {"endIndex", EndIndex},
    {"type", Type},
    {"typeId", TypeId},
    {"isNamed", IsNamed},
    {"parent", Parent},
    {"child", Child},
    {"namedChild", NamedChild},
    {"children", Children},
    {"namedChildren", NamedChildren},
    {"childCount", ChildCount},
    {"namedChildCount", NamedChildCount},
    {"firstChild", FirstChild},
    {"lastChild", LastChild},
    {"firstNamedChild", FirstNamedChild},
    {"lastNamedChild", LastNamedChild},
    {"nextSibling", NextSibling},
    {"nextNamedSibling", NextNamedSibling},
    {"previousSibling", PreviousSibling},
    {"previousNamedSibling", PreviousNamedSibling},
    {"startPosition", StartPosition},
    {"endPosition", EndPosition},
    {"isMissing", IsMissing},
    {"toString", ToString},
    {"firstChildForIndex", FirstChildForIndex},
    {"firstNamedChildForIndex", FirstNamedChildForIndex},
    {"descendantForIndex", DescendantForIndex},
    {"namedDescendantForIndex", NamedDescendantForIndex},
    {"descendantForPosition", DescendantForPosition},
    {"namedDescendantForPosition", NamedDescendantForPosition},
    {"hasChanges", HasChanges},
    {"hasError", HasError},
    {"descendantsOfType", DescendantsOfType},
    {"walk", Walk},
    {"closest", Closest},
    {"childNodeForFieldId", ChildNodeForFieldId},
    {"childNodesForFieldId", ChildNodesForFieldId},
  };

  for (size_t i = 0; i < length_of_array(methods); i++) {
    Nan::Set(
      result,
      Nan::New(methods[i].name).ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(methods[i].callback)).ToLocalChecked()
    );
  }

  module_exports.Reset(exports);
  setup_transfer_buffer(1);

  Nan::Set(exports, Nan::New("NodeMethods").ToLocalChecked(), result);
}

}  // namespace node_methods
}  // namespace node_tree_sitter
