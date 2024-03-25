#ifndef NODE_TREE_SITTER_TREE_H_
#define NODE_TREE_SITTER_TREE_H_

#include <v8.h>
#include <nan.h>
#include <node_object_wrap.h>
#include <unordered_map>
#include <tree_sitter/api.h>

namespace node_tree_sitter {

class Tree : public Nan::ObjectWrap {
 public:
  static void Init(v8::Local<v8::Object> exports);
  static v8::Local<v8::Value> NewInstance(TSTree *);
  static const Tree *UnwrapTree(const v8::Local<v8::Value> &);

  struct NodeCacheEntry {
    Tree *tree;
    const void *key;
    v8::Persistent<v8::Object> node;
  };

  TSTree *tree_;
  std::unordered_map<const void *, NodeCacheEntry *> cached_nodes_;

 private:
  explicit Tree(TSTree *);
  ~Tree();

  static void New(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void Edit(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void RootNode(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void PrintDotGraph(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void GetEditedRange(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void GetChangedRanges(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void CacheNode(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void CacheNodes(const Nan::FunctionCallbackInfo<v8::Value> &);

  static Nan::Persistent<v8::Function> constructor;
  static Nan::Persistent<v8::FunctionTemplate> constructor_template;
};

}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_TREE_H_
