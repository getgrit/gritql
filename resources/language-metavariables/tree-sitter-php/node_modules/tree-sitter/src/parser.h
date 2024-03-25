#ifndef NODE_TREE_SITTER_PARSER_H_
#define NODE_TREE_SITTER_PARSER_H_

#include <v8.h>
#include <nan.h>
#include <node_object_wrap.h>
#include <tree_sitter/api.h>

namespace node_tree_sitter {

class Parser : public Nan::ObjectWrap {
 public:
  static void Init(v8::Local<v8::Object> exports);

  TSParser *parser_;

 private:
  explicit Parser();
  ~Parser();

  static void New(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void SetLanguage(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void GetLogger(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void SetLogger(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void Parse(const Nan::FunctionCallbackInfo<v8::Value> &);
  static void PrintDotGraphs(const Nan::FunctionCallbackInfo<v8::Value> &);

  static Nan::Persistent<v8::Function> constructor;
};

}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_PARSER_H_
