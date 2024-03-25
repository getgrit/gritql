#ifndef NODE_TREE_SITTER_QUERY_H_
#define NODE_TREE_SITTER_QUERY_H_

#include <v8.h>
#include <nan.h>
#include <node_object_wrap.h>
#include <unordered_map>
#include <tree_sitter/api.h>

namespace node_tree_sitter {

class Query : public Nan::ObjectWrap {
 public:
  static void Init(v8::Local<v8::Object> exports);
  static v8::Local<v8::Value> NewInstance(TSQuery *);
  static Query *UnwrapQuery(const v8::Local<v8::Value> &);

  TSQuery *query_;

 private:
  explicit Query(TSQuery *);
  ~Query();

  static void New(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void Matches(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void Captures(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void GetPredicates(const Nan::FunctionCallbackInfo<v8::Value> &);

  static TSQueryCursor *ts_query_cursor;
  static Nan::Persistent<v8::Function> constructor;
  static Nan::Persistent<v8::FunctionTemplate> constructor_template;
};

}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_QUERY_H_
