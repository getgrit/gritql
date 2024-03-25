#include "./tree.h"
#include <string>
#include <v8.h>
#include <nan.h>
#include "./node.h"
#include "./logger.h"
#include "./util.h"
#include "./conversions.h"

namespace node_tree_sitter {

using namespace v8;
using node_methods::UnmarshalNodeId;

Nan::Persistent<Function> Tree::constructor;
Nan::Persistent<FunctionTemplate> Tree::constructor_template;

void Tree::Init(Local<Object> exports) {
  Local<FunctionTemplate> tpl = Nan::New<FunctionTemplate>(New);
  tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<String> class_name = Nan::New("Tree").ToLocalChecked();
  tpl->SetClassName(class_name);

  FunctionPair methods[] = {
    {"edit", Edit},
    {"rootNode", RootNode},
    {"printDotGraph", PrintDotGraph},
    {"getChangedRanges", GetChangedRanges},
    {"getEditedRange", GetEditedRange},
    {"_cacheNode", CacheNode},
    {"_cacheNodes", CacheNodes},
  };

  for (size_t i = 0; i < length_of_array(methods); i++) {
    Nan::SetPrototypeMethod(tpl, methods[i].name, methods[i].callback);
  }

  Local<Function> ctor = Nan::GetFunction(tpl).ToLocalChecked();

  constructor_template.Reset(tpl);
  constructor.Reset(ctor);
  Nan::Set(exports, class_name, ctor);
}

Tree::Tree(TSTree *tree) : tree_(tree) {}

Tree::~Tree() {
  ts_tree_delete(tree_);
  for (auto &entry : cached_nodes_) {
    entry.second->tree = nullptr;
  }
}

Local<Value> Tree::NewInstance(TSTree *tree) {
  if (tree) {
    Local<Object> self;
    MaybeLocal<Object> maybe_self = Nan::NewInstance(Nan::New(constructor));
    if (maybe_self.ToLocal(&self)) {
      (new Tree(tree))->Wrap(self);
      return self;
    }
  }
  return Nan::Null();
}

const Tree *Tree::UnwrapTree(const Local<Value> &value) {
  if (!value->IsObject()) return nullptr;
  Local<Object> js_tree = Local<Object>::Cast(value);
  if (!Nan::New(constructor_template)->HasInstance(js_tree)) return nullptr;
  return ObjectWrap::Unwrap<Tree>(js_tree);
}

void Tree::New(const Nan::FunctionCallbackInfo<Value> &info) {}

#define read_number_from_js(out, value, name)        \
  maybe_number = Nan::To<uint32_t>(value);           \
  if (maybe_number.IsNothing()) {                    \
    Nan::ThrowTypeError(name " must be an integer"); \
    return;                                          \
  }                                                  \
  *(out) = maybe_number.FromJust();

#define read_byte_count_from_js(out, value, name)   \
  read_number_from_js(out, value, name);            \
  (*out) *= 2

void Tree::Edit(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());

  TSInputEdit edit;
  Nan::Maybe<uint32_t> maybe_number = Nan::Nothing<uint32_t>();
  read_number_from_js(&edit.start_point.row, info[0], "startPosition.row");
  read_byte_count_from_js(&edit.start_point.column, info[1], "startPosition.column");
  read_number_from_js(&edit.old_end_point.row, info[2], "oldEndPosition.row");
  read_byte_count_from_js(&edit.old_end_point.column, info[3], "oldEndPosition.column");
  read_number_from_js(&edit.new_end_point.row, info[4], "newEndPosition.row");
  read_byte_count_from_js(&edit.new_end_point.column, info[5], "newEndPosition.column");
  read_byte_count_from_js(&edit.start_byte, info[6], "startIndex");
  read_byte_count_from_js(&edit.old_end_byte, info[7], "oldEndIndex");
  read_byte_count_from_js(&edit.new_end_byte, info[8], "newEndIndex");

  ts_tree_edit(tree->tree_, &edit);

  for (auto &entry : tree->cached_nodes_) {
    Local<Object> js_node = Nan::New(entry.second->node);
    TSNode node;
    node.id = entry.first;
    for (unsigned i = 0; i < 4; i++) {
      Local<Value> node_field;
      if (Nan::Get(js_node, i + 2).ToLocal(&node_field)) {
        node.context[i] = Nan::To<uint32_t>(node_field).FromMaybe(0);
      }
    }

    ts_node_edit(&node, &edit);

    for (unsigned i = 0; i < 4; i++) {
      Nan::Set(js_node, i + 2, Nan::New(node.context[i]));
    }
  }

  info.GetReturnValue().Set(info.This());
}

void Tree::RootNode(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  node_methods::MarshalNode(info, tree, ts_tree_root_node(tree->tree_));
}

void Tree::GetChangedRanges(const Nan::FunctionCallbackInfo<Value> &info) {
  const Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  const Tree *other_tree = UnwrapTree(info[0]);
  if (!other_tree) {
    Nan::ThrowTypeError("Argument must be a tree");
    return;
  }

  uint32_t range_count;
  TSRange *ranges = ts_tree_get_changed_ranges(tree->tree_, other_tree->tree_, &range_count);

  Local<Array> result = Nan::New<Array>();
  for (size_t i = 0; i < range_count; i++) {
    Nan::Set(result, i, RangeToJS(ranges[i]));
  }

  free(ranges);

  info.GetReturnValue().Set(result);
}

void Tree::GetEditedRange(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  TSNode root = ts_tree_root_node(tree->tree_);
  if (!ts_node_has_changes(root)) return;
  TSRange result = {
    ts_node_start_point(root),
    ts_node_end_point(root),
    ts_node_start_byte(root),
    ts_node_end_byte(root),
  };

  TSTreeCursor cursor = ts_tree_cursor_new(root);

  while (true) {
    if (!ts_tree_cursor_goto_first_child(&cursor)) break;
    while (true) {
      TSNode node = ts_tree_cursor_current_node(&cursor);
      if (ts_node_has_changes(node)) {
        result.start_byte = ts_node_start_byte(node);
        result.start_point = ts_node_start_point(node);
        break;
      } else if (!ts_tree_cursor_goto_next_sibling(&cursor)) {
        break;
      }
    }
  }

  while (ts_tree_cursor_goto_parent(&cursor)) {}

  while (true) {
    if (!ts_tree_cursor_goto_first_child(&cursor)) break;
    while (true) {
      TSNode node = ts_tree_cursor_current_node(&cursor);
      if (ts_node_has_changes(node)) {
        result.end_byte = ts_node_end_byte(node);
        result.end_point = ts_node_end_point(node);
      }

      if (!ts_tree_cursor_goto_next_sibling(&cursor)) {
        break;
      }
    }
  }

  ts_tree_cursor_delete(&cursor);
  info.GetReturnValue().Set(RangeToJS(result));
}

void Tree::PrintDotGraph(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  ts_tree_print_dot_graph(tree->tree_, stderr);
  info.GetReturnValue().Set(info.This());
}

static void FinalizeNode(const v8::WeakCallbackInfo<Tree::NodeCacheEntry> &info) {
  Tree::NodeCacheEntry *cache_entry = info.GetParameter();
  assert(!cache_entry->node.IsEmpty());
  cache_entry->node.Reset();
  if (cache_entry->tree) {
    assert(cache_entry->tree->cached_nodes_.count(cache_entry->key));
    cache_entry->tree->cached_nodes_.erase(cache_entry->key);
  }
  delete cache_entry;
}

static void CacheNodeForTree(Tree *tree, Isolate *isolate, Local<Object> js_node) {
  Local<Value> js_node_field1, js_node_field2;
  if (!Nan::Get(js_node, 0).ToLocal(&js_node_field1)) return;
  if (!Nan::Get(js_node, 1).ToLocal(&js_node_field2)) return;
  uint32_t key_parts[2] = {
    Nan::To<uint32_t>(js_node_field1).FromMaybe(0),
    Nan::To<uint32_t>(js_node_field2).FromMaybe(0)
  };
  const void *key = UnmarshalNodeId(key_parts);

  auto cache_entry = new Tree::NodeCacheEntry{tree, key, {}};
  cache_entry->node.Reset(isolate, js_node);
  cache_entry->node.SetWeak(cache_entry, &FinalizeNode, Nan::WeakCallbackType::kParameter);

  assert(!tree->cached_nodes_.count(key));

  tree->cached_nodes_[key] = cache_entry;
}

void Tree::CacheNode(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  Isolate *isolate = info.GetIsolate();
  Local<Object> js_node = Local<Object>::Cast(info[0]);

  CacheNodeForTree(tree, isolate, js_node);
}

void Tree::CacheNodes(const Nan::FunctionCallbackInfo<Value> &info) {
  Tree *tree = ObjectWrap::Unwrap<Tree>(info.This());
  Isolate *isolate = info.GetIsolate();
  Local<Array> js_nodes = Local<Array>::Cast(info[0]);
  uint32_t length = js_nodes->Length();

  for (uint32_t i = 0; i < length; i++) {
    auto js_node = Local<Object>::Cast(Nan::Get(js_nodes, i).ToLocalChecked());
    CacheNodeForTree(tree, isolate, js_node);
  }
}

}  // namespace node_tree_sitter
