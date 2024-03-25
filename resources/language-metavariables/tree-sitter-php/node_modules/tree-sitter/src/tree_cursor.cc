#include "./tree_cursor.h"
#include <nan.h>
#include <tree_sitter/api.h>
#include <v8.h>
#include "./util.h"
#include "./conversions.h"
#include "./node.h"
#include "./tree.h"

namespace node_tree_sitter {

using namespace v8;

Nan::Persistent<Function> TreeCursor::constructor;

void TreeCursor::Init(v8::Local<v8::Object> exports) {
  Local<FunctionTemplate> tpl = Nan::New<FunctionTemplate>(New);
  Local<String> class_name = Nan::New("TreeCursor").ToLocalChecked();
  tpl->SetClassName(class_name);
  tpl->InstanceTemplate()->SetInternalFieldCount(1);

  GetterPair getters[] = {
    {"startIndex", StartIndex},
    {"endIndex", EndIndex},
    {"nodeType", NodeType},
    {"nodeIsNamed", NodeIsNamed},
    {"nodeIsMissing", NodeIsMissing},
    {"currentFieldName", CurrentFieldName},
  };

  FunctionPair methods[] = {
    {"startPosition", StartPosition},
    {"endPosition", EndPosition},
    {"gotoParent", GotoParent},
    {"gotoFirstChild", GotoFirstChild},
    {"gotoFirstChildForIndex", GotoFirstChildForIndex},
    {"gotoNextSibling", GotoNextSibling},
    {"currentNode", CurrentNode},
    {"reset", Reset},
  };

  for (size_t i = 0; i < length_of_array(getters); i++) {
    Nan::SetAccessor(
      tpl->InstanceTemplate(),
      Nan::New(getters[i].name).ToLocalChecked(),
      getters[i].callback);
  }

  for (size_t i = 0; i < length_of_array(methods); i++) {
    Nan::SetPrototypeMethod(tpl, methods[i].name, methods[i].callback);
  }

  Local<Function> constructor_local = Nan::GetFunction(tpl).ToLocalChecked();
  Nan::Set(exports, class_name, constructor_local);
  constructor.Reset(Nan::Persistent<Function>(constructor_local));
}

Local<Value> TreeCursor::NewInstance(TSTreeCursor cursor) {
  Local<Object> self;
  MaybeLocal<Object> maybe_self = Nan::New(constructor)->NewInstance(Nan::GetCurrentContext());
  if (maybe_self.ToLocal(&self)) {
    (new TreeCursor(cursor))->Wrap(self);
    return self;
  } else {
    return Nan::Null();
  }
}

TreeCursor::TreeCursor(TSTreeCursor cursor) : cursor_(cursor) {}

TreeCursor::~TreeCursor() { ts_tree_cursor_delete(&cursor_); }

void TreeCursor::New(const Nan::FunctionCallbackInfo<Value> &info) {
  info.GetReturnValue().Set(Nan::Null());
}

void TreeCursor::GotoParent(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  bool result = ts_tree_cursor_goto_parent(&cursor->cursor_);
  info.GetReturnValue().Set(Nan::New(result));
}

void TreeCursor::GotoFirstChild(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  bool result = ts_tree_cursor_goto_first_child(&cursor->cursor_);
  info.GetReturnValue().Set(Nan::New(result));
}

void TreeCursor::GotoFirstChildForIndex(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  auto maybe_index = Nan::To<uint32_t>(info[0]);
  if (maybe_index.IsNothing()) {
    Nan::ThrowTypeError("Argument must be an integer");
    return;
  }
  uint32_t goal_byte = maybe_index.FromJust() * 2;
  int64_t child_index = ts_tree_cursor_goto_first_child_for_byte(&cursor->cursor_, goal_byte);
  if (child_index < 0) {
    info.GetReturnValue().Set(Nan::Null());
  } else {
    info.GetReturnValue().Set(Nan::New(static_cast<uint32_t>(child_index)));
  }
}

void TreeCursor::GotoNextSibling(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  bool result = ts_tree_cursor_goto_next_sibling(&cursor->cursor_);
  info.GetReturnValue().Set(Nan::New(result));
}

void TreeCursor::StartPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  TransferPoint(ts_node_start_point(node));
}

void TreeCursor::EndPosition(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  TransferPoint(ts_node_end_point(node));
}

void TreeCursor::CurrentNode(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  Local<String> key = Nan::New<String>("tree").ToLocalChecked();
  const Tree *tree = Tree::UnwrapTree(Nan::Get(info.This(), key).ToLocalChecked());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  node_methods::MarshalNode(info, tree, node);
}

void TreeCursor::Reset(const Nan::FunctionCallbackInfo<Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  Local<String> key = Nan::New<String>("tree").ToLocalChecked();
  const Tree *tree = Tree::UnwrapTree(Nan::Get(info.This(), key).ToLocalChecked());
  TSNode node = node_methods::UnmarshalNode(tree);
  ts_tree_cursor_reset(&cursor->cursor_, node);
}

void TreeCursor::NodeType(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  info.GetReturnValue().Set(Nan::New(ts_node_type(node)).ToLocalChecked());
}

void TreeCursor::NodeIsNamed(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);

  info.GetReturnValue().Set(Nan::New(ts_node_is_named(node)));
}

void TreeCursor::NodeIsMissing(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);

  info.GetReturnValue().Set(Nan::New(ts_node_is_missing(node)));
}

void TreeCursor::CurrentFieldName(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  const char *field_name = ts_tree_cursor_current_field_name(&cursor->cursor_);
  if (field_name) {
    info.GetReturnValue().Set(Nan::New(field_name).ToLocalChecked());
  }
}

void TreeCursor::StartIndex(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  info.GetReturnValue().Set(ByteCountToJS(ts_node_start_byte(node)));
}

void TreeCursor::EndIndex(v8::Local<v8::String> prop, const Nan::PropertyCallbackInfo<v8::Value> &info) {
  TreeCursor *cursor = Nan::ObjectWrap::Unwrap<TreeCursor>(info.This());
  TSNode node = ts_tree_cursor_current_node(&cursor->cursor_);
  info.GetReturnValue().Set(ByteCountToJS(ts_node_end_byte(node)));
}

}
