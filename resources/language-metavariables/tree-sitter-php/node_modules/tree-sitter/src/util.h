#ifndef NODE_TREE_SITTER_UTIL_H_
#define NODE_TREE_SITTER_UTIL_H_

#include <v8.h>
#include <nan.h>

namespace node_tree_sitter {

#define length_of_array(a) (sizeof(a) / sizeof(a[0]))

struct GetterPair {
  const char *name;
  Nan::GetterCallback callback;
};

struct FunctionPair {
  const char *name;
  Nan::FunctionCallback callback;
};

bool instance_of(v8::Local<v8::Value> value, v8::Local<v8::Object> object);

v8::Local<v8::Object> GetGlobal(v8::Local<v8::Function>& callback);

}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_UTIL_H_
