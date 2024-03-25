#include "./query.h"
#include <string>
#include <vector>
#include <v8.h>
#include <nan.h>
#include "./node.h"
#include "./language.h"
#include "./logger.h"
#include "./util.h"
#include "./conversions.h"

namespace node_tree_sitter {

using std::vector;
using namespace v8;
using node_methods::UnmarshalNodeId;

const char *query_error_names[] = {
  "TSQueryErrorNone",
  "TSQueryErrorSyntax",
  "TSQueryErrorNodeType",
  "TSQueryErrorField",
  "TSQueryErrorCapture",
  "TSQueryErrorStructure",
};

TSQueryCursor *Query::ts_query_cursor;
Nan::Persistent<Function> Query::constructor;
Nan::Persistent<FunctionTemplate> Query::constructor_template;

void Query::Init(Local<Object> exports) {
  ts_query_cursor = ts_query_cursor_new();

  Local<FunctionTemplate> tpl = Nan::New<FunctionTemplate>(New);
  tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<String> class_name = Nan::New("Query").ToLocalChecked();
  tpl->SetClassName(class_name);

  FunctionPair methods[] = {
    {"_matches", Matches},
    {"_captures", Captures},
    {"_getPredicates", GetPredicates},
  };

  for (size_t i = 0; i < length_of_array(methods); i++) {
    Nan::SetPrototypeMethod(tpl, methods[i].name, methods[i].callback);
  }

  Local<Function> ctor = Nan::GetFunction(tpl).ToLocalChecked();

  constructor_template.Reset(tpl);
  constructor.Reset(ctor);
  Nan::Set(exports, class_name, ctor);
}

Query::Query(TSQuery *query) : query_(query) {}

Query::~Query() {
  ts_query_delete(query_);
}

Local<Value> Query::NewInstance(TSQuery *query) {
  if (query) {
    Local<Object> self;
    MaybeLocal<Object> maybe_self = Nan::NewInstance(Nan::New(constructor));
    if (maybe_self.ToLocal(&self)) {
      (new Query(query))->Wrap(self);
      return self;
    }
  }
  return Nan::Null();
}

Query *Query::UnwrapQuery(const Local<Value> &value) {
  if (!value->IsObject()) return nullptr;
  Local<Object> js_query = Local<Object>::Cast(value);
  if (!Nan::New(constructor_template)->HasInstance(js_query)) return nullptr;
  return ObjectWrap::Unwrap<Query>(js_query);
}

void Query::New(const Nan::FunctionCallbackInfo<Value> &info) {
  if (!info.IsConstructCall()) {
    Local<Object> self;
    MaybeLocal<Object> maybe_self = Nan::New(constructor)->NewInstance(Nan::GetCurrentContext());
    if (maybe_self.ToLocal(&self)) {
      info.GetReturnValue().Set(self);
    } else {
      info.GetReturnValue().Set(Nan::Null());
    }
    return;
  }

  const TSLanguage *language = language_methods::UnwrapLanguage(info[0]);
  const char *source;
  uint32_t source_len;
  uint32_t error_offset = 0;
  TSQueryError error_type = TSQueryErrorNone;
  TSQuery *query;

  if (language == nullptr) {
    Nan::ThrowError("Missing language argument");
    return;
  }

  if (info[1]->IsString()) {
    auto string = Nan::To<String> (info[1]).ToLocalChecked();
    Nan::Utf8String utf8_string(string);
    source = *utf8_string;
    source_len = utf8_string.length();
    query = ts_query_new(language, source, source_len, &error_offset, &error_type);
  }
  else if (node::Buffer::HasInstance(info[1])) {
    source = node::Buffer::Data(info[1]);
    source_len = node::Buffer::Length(info[1]);
    query = ts_query_new(language, source, source_len, &error_offset, &error_type);
  }
  else {
    Nan::ThrowError("Missing source argument");
    return;
  }

  if (error_offset > 0) {
    const char *error_name = query_error_names[error_type];
    std::string message = "Query error of type ";
    message += error_name;
    message += " at position ";
    message += std::to_string(error_offset);
    Nan::ThrowError(message.c_str());
    return;
  }

  auto self = info.This();

  Query *query_wrapper = new Query(query);
  query_wrapper->Wrap(self);

  auto init =
    Nan::To<Function>(
      Nan::Get(self, Nan::New<String>("_init").ToLocalChecked()).ToLocalChecked()
    ).ToLocalChecked();
  Nan::Call(init, self, 0, nullptr);

  info.GetReturnValue().Set(self);
}

void Query::GetPredicates(const Nan::FunctionCallbackInfo<Value> &info) {
  Query *query = Query::UnwrapQuery(info.This());
  auto ts_query = query->query_;

  auto pattern_len = ts_query_pattern_count(ts_query);

  Local<Array> js_predicates = Nan::New<Array>();

  for (size_t pattern_index = 0; pattern_index < pattern_len; pattern_index++) {
    uint32_t predicates_len;
    const TSQueryPredicateStep *predicates = ts_query_predicates_for_pattern(
        ts_query, pattern_index, &predicates_len);

    Local<Array> js_pattern_predicates = Nan::New<Array>();

    if (predicates_len > 0) {
      Local<Array> js_predicate = Nan::New<Array>();

      size_t a_index = 0;
      size_t p_index = 0;
      for (size_t i = 0; i < predicates_len; i++) {
        const TSQueryPredicateStep predicate = predicates[i];
        uint32_t len;
        switch (predicate.type) {
          case TSQueryPredicateStepTypeCapture:
            Nan::Set(js_predicate, p_index++, Nan::New(TSQueryPredicateStepTypeCapture));
            Nan::Set(js_predicate, p_index++,
                Nan::New<String>(
                  ts_query_capture_name_for_id(ts_query, predicate.value_id, &len)
                ).ToLocalChecked());
            break;
          case TSQueryPredicateStepTypeString:
            Nan::Set(js_predicate, p_index++, Nan::New(TSQueryPredicateStepTypeString));
            Nan::Set(js_predicate, p_index++,
                Nan::New<String>(
                  ts_query_string_value_for_id(ts_query, predicate.value_id, &len)
                ).ToLocalChecked());
            break;
          case TSQueryPredicateStepTypeDone:
            Nan::Set(js_pattern_predicates, a_index++, js_predicate);
            js_predicate = Nan::New<Array>();
            p_index = 0;
            break;
        }
      }
    }

    Nan::Set(js_predicates, pattern_index, js_pattern_predicates);
  }

  info.GetReturnValue().Set(js_predicates);
}

void Query::Matches(const Nan::FunctionCallbackInfo<Value> &info) {
  Query *query = Query::UnwrapQuery(info.This());
  const Tree *tree = Tree::UnwrapTree(info[0]);
  uint32_t start_row    = Nan::To<uint32_t>(info[1]).ToChecked();
  uint32_t start_column = Nan::To<uint32_t>(info[2]).ToChecked() << 1;
  uint32_t end_row      = Nan::To<uint32_t>(info[3]).ToChecked();
  uint32_t end_column   = Nan::To<uint32_t>(info[4]).ToChecked() << 1;

  if (query == nullptr) {
    Nan::ThrowError("Missing argument query");
    return;
  }

  if (tree == nullptr) {
    Nan::ThrowError("Missing argument tree");
    return;
  }

  TSQuery *ts_query = query->query_;
  TSNode rootNode = node_methods::UnmarshalNode(tree);
  TSPoint start_point = {start_row, start_column};
  TSPoint end_point = {end_row, end_column};
  ts_query_cursor_set_point_range(ts_query_cursor, start_point, end_point);
  ts_query_cursor_exec(ts_query_cursor, ts_query, rootNode);

  Local<Array> js_matches = Nan::New<Array>();
  unsigned index = 0;
  vector<TSNode> nodes;
  TSQueryMatch match;

  while (ts_query_cursor_next_match(ts_query_cursor, &match)) {
    Nan::Set(js_matches, index++, Nan::New(match.pattern_index));

    for (uint16_t i = 0; i < match.capture_count; i++) {
      const TSQueryCapture &capture = match.captures[i];

      uint32_t capture_name_len = 0;
      const char *capture_name = ts_query_capture_name_for_id(
          ts_query, capture.index, &capture_name_len);

      TSNode node = capture.node;
      nodes.push_back(node);

      Local<Value> js_capture = Nan::New(capture_name).ToLocalChecked();
      Nan::Set(js_matches, index++, js_capture);
    }
  }

  auto js_nodes = node_methods::GetMarshalNodes(info, tree, nodes.data(), nodes.size());

  auto result = Nan::New<Array>();
  Nan::Set(result, 0, js_matches);
  Nan::Set(result, 1, js_nodes);
  info.GetReturnValue().Set(result);
}

void Query::Captures(const Nan::FunctionCallbackInfo<Value> &info) {
  Query *query = Query::UnwrapQuery(info.This());
  const Tree *tree = Tree::UnwrapTree(info[0]);
  uint32_t start_row    = Nan::To<uint32_t>(info[1]).ToChecked();
  uint32_t start_column = Nan::To<uint32_t>(info[2]).ToChecked() << 1;
  uint32_t end_row      = Nan::To<uint32_t>(info[3]).ToChecked();
  uint32_t end_column   = Nan::To<uint32_t>(info[4]).ToChecked() << 1;

  if (query == nullptr) {
    Nan::ThrowError("Missing argument query");
    return;
  }

  if (tree == nullptr) {
    Nan::ThrowError("Missing argument tree");
    return;
  }

  TSQuery *ts_query = query->query_;
  TSNode rootNode = node_methods::UnmarshalNode(tree);
  TSPoint start_point = {start_row, start_column};
  TSPoint end_point = {end_row, end_column};
  ts_query_cursor_set_point_range(ts_query_cursor, start_point, end_point);
  ts_query_cursor_exec(ts_query_cursor, ts_query, rootNode);

  Local<Array> js_matches = Nan::New<Array>();
  unsigned index = 0;
  vector<TSNode> nodes;
  TSQueryMatch match;
  uint32_t capture_index;

  while (ts_query_cursor_next_capture(
    ts_query_cursor,
    &match,
    &capture_index
  )) {

    Nan::Set(js_matches, index++, Nan::New(match.pattern_index));
    Nan::Set(js_matches, index++, Nan::New(capture_index));

    for (uint16_t i = 0; i < match.capture_count; i++) {
      const TSQueryCapture &capture = match.captures[i];

      uint32_t capture_name_len = 0;
      const char *capture_name = ts_query_capture_name_for_id(
          ts_query, capture.index, &capture_name_len);

      TSNode node = capture.node;
      nodes.push_back(node);

      Local<Value> js_capture = Nan::New(capture_name).ToLocalChecked();
      Nan::Set(js_matches, index++, js_capture);
    }
  }

  auto js_nodes = node_methods::GetMarshalNodes(info, tree, nodes.data(), nodes.size());

  auto result = Nan::New<Array>();
  Nan::Set(result, 0, js_matches);
  Nan::Set(result, 1, js_nodes);
  info.GetReturnValue().Set(result);
}


}  // namespace node_tree_sitter
