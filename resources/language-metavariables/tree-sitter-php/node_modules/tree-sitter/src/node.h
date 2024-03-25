#ifndef NODE_TREE_SITTER_NODE_H_
#define NODE_TREE_SITTER_NODE_H_

#include <nan.h>
#include <v8.h>
#include <node_object_wrap.h>
#include <tree_sitter/api.h>
#include "./tree.h"

using namespace v8;

namespace node_tree_sitter {
namespace node_methods {

void Init(v8::Local<v8::Object>);
void MarshalNode(const Nan::FunctionCallbackInfo<v8::Value> &info, const Tree *, TSNode);
Local<Value> GetMarshalNode(const Nan::FunctionCallbackInfo<Value> &info, const Tree *tree, TSNode node);
Local<Value> GetMarshalNodes(const Nan::FunctionCallbackInfo<Value> &info, const Tree *tree, const TSNode *nodes, uint32_t node_count);
TSNode UnmarshalNode(const Tree *tree);

static inline const void *UnmarshalNodeId(const uint32_t *buffer) {
  const void *result;
  memcpy(&result, buffer, sizeof(result));
  return result;
}

static inline void MarshalNodeId(const void *id, uint32_t *buffer) {
  memset(buffer, 0, sizeof(uint64_t));
  memcpy(buffer, &id, sizeof(id));
}

}  // namespace node_methods
}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_NODE_H_
