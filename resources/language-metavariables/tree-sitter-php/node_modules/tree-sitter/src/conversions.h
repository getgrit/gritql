#ifndef NODE_TREE_SITTER_CONVERSIONS_H_
#define NODE_TREE_SITTER_CONVERSIONS_H_

#include <nan.h>
#include <v8.h>
#include <tree_sitter/api.h>

namespace node_tree_sitter {

void InitConversions(v8::Local<v8::Object> exports);
v8::Local<v8::Object> RangeToJS(const TSRange &);
v8::Local<v8::Object> PointToJS(const TSPoint &);
void TransferPoint(const TSPoint &);
v8::Local<v8::Number> ByteCountToJS(uint32_t);
Nan::Maybe<TSPoint> PointFromJS(const v8::Local<v8::Value> &);
Nan::Maybe<uint32_t> ByteCountFromJS(const v8::Local<v8::Value> &);
Nan::Maybe<TSRange> RangeFromJS(const v8::Local<v8::Value> &);

extern Nan::Persistent<v8::String> row_key;
extern Nan::Persistent<v8::String> column_key;
extern Nan::Persistent<v8::String> start_key;
extern Nan::Persistent<v8::String> end_key;

}  // namespace node_tree_sitter

#endif  // NODE_TREE_SITTER_CONVERSIONS_H_
