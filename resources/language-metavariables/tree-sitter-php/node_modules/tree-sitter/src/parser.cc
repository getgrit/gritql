#include "./parser.h"
#include <string>
#include <vector>
#include <climits>
#include <v8.h>
#include <nan.h>
#include "./conversions.h"
#include "./language.h"
#include "./logger.h"
#include "./tree.h"
#include "./util.h"
#include <cmath>

namespace node_tree_sitter {

using namespace v8;
using std::vector;
using std::pair;

Nan::Persistent<Function> Parser::constructor;

class CallbackInput {
 public:
  CallbackInput(v8::Local<v8::Function> callback, v8::Local<v8::Value> js_buffer_size)
    : callback(callback),
      byte_offset(0),
      partial_string_offset(0) {
    uint32_t buffer_size = Nan::To<uint32_t>(js_buffer_size).FromMaybe(0);
    if (buffer_size == 0) buffer_size = 32 * 1024;
    buffer.resize(buffer_size);
  }

  TSInput Input() {
    TSInput result;
    result.payload = (void *)this;
    result.encoding = TSInputEncodingUTF16;
    result.read = Read;
    return result;
  }

 private:
  static const char * Read(void *payload, uint32_t byte, TSPoint position, uint32_t *bytes_read) {
    CallbackInput *reader = (CallbackInput *)payload;

    if (byte != reader->byte_offset) {
      reader->byte_offset = byte;
      reader->partial_string_offset = 0;
      reader->partial_string.Reset();
    }

    *bytes_read = 0;
    Local<String> result;
    uint32_t start = 0;
    if (reader->partial_string_offset) {
      result = Nan::New(reader->partial_string);
      start = reader->partial_string_offset;
    } else {
      Local<Function> callback = Nan::New(reader->callback);
      uint32_t utf16_unit = byte / 2;
      Local<Value> argv[2] = { Nan::New<Number>(utf16_unit), PointToJS(position) };
      TryCatch try_catch(Isolate::GetCurrent());
      auto maybe_result_value = Nan::Call(callback, GetGlobal(callback), 2, argv);
      if (try_catch.HasCaught()) return nullptr;

      Local<Value> result_value;
      if (!maybe_result_value.ToLocal(&result_value)) return nullptr;
      if (!result_value->IsString()) return nullptr;
      if (!Nan::To<String>(result_value).ToLocal(&result)) return nullptr;
    }

    int utf16_units_read = result->Write(

      // Nan doesn't wrap this functionality
      #if NODE_MAJOR_VERSION >= 12
        Isolate::GetCurrent(),
      #endif

      reader->buffer.data(),
      start,
      reader->buffer.size(),
      String::NO_NULL_TERMINATION
    );
    int end = start + utf16_units_read;
    *bytes_read = 2 * utf16_units_read;

    reader->byte_offset += *bytes_read;

    if (end < result->Length()) {
      reader->partial_string_offset = end;
      reader->partial_string.Reset(result);
    } else {
      reader->partial_string_offset = 0;
      reader->partial_string.Reset();
    }

    return (const char *)reader->buffer.data();
  }

  Nan::Persistent<v8::Function> callback;
  std::vector<uint16_t> buffer;
  size_t byte_offset;
  Nan::Persistent<v8::String> partial_string;
  size_t partial_string_offset;
};

void Parser::Init(Local<Object> exports) {
  Local<FunctionTemplate> tpl = Nan::New<FunctionTemplate>(New);
  tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<String> class_name = Nan::New("Parser").ToLocalChecked();
  tpl->SetClassName(class_name);

  FunctionPair methods[] = {
    {"getLogger", GetLogger},
    {"setLogger", SetLogger},
    {"setLanguage", SetLanguage},
    {"printDotGraphs", PrintDotGraphs},
    {"parse", Parse},
  };

  for (size_t i = 0; i < length_of_array(methods); i++) {
    Nan::SetPrototypeMethod(tpl, methods[i].name, methods[i].callback);
  }

  constructor.Reset(Nan::Persistent<Function>(Nan::GetFunction(tpl).ToLocalChecked()));
  Nan::Set(exports, class_name, Nan::New(constructor));
  Nan::Set(exports, Nan::New("LANGUAGE_VERSION").ToLocalChecked(), Nan::New<Number>(TREE_SITTER_LANGUAGE_VERSION));
}

Parser::Parser() : parser_(ts_parser_new()) {}

Parser::~Parser() { ts_parser_delete(parser_); }

static bool handle_included_ranges(TSParser *parser, Local<Value> arg) {
  uint32_t last_included_range_end = 0;
  if (arg->IsArray()) {
    auto js_included_ranges = Local<Array>::Cast(arg);
    vector<TSRange> included_ranges;
    for (unsigned i = 0; i < js_included_ranges->Length(); i++) {
      Local<Value> range_value;
      if (!Nan::Get(js_included_ranges, i).ToLocal(&range_value)) return false;
      auto maybe_range = RangeFromJS(range_value);
      if (!maybe_range.IsJust()) return false;
      auto range = maybe_range.FromJust();
      if (range.start_byte < last_included_range_end) {
        Nan::ThrowRangeError("Overlapping ranges");
        return false;
      }
      last_included_range_end = range.end_byte;
      included_ranges.push_back(range);
    }
    ts_parser_set_included_ranges(parser, included_ranges.data(), included_ranges.size());
  } else {
    ts_parser_set_included_ranges(parser, nullptr, 0);
  }

  return true;
}

void Parser::New(const Nan::FunctionCallbackInfo<Value> &info) {
  if (info.IsConstructCall()) {
    Parser *parser = new Parser();
    parser->Wrap(info.This());
    info.GetReturnValue().Set(info.This());
  } else {
    Local<Object> self;
    MaybeLocal<Object> maybe_self = Nan::New(constructor)->NewInstance(Nan::GetCurrentContext());
    if (maybe_self.ToLocal(&self)) {
      info.GetReturnValue().Set(self);
    } else {
      info.GetReturnValue().Set(Nan::Null());
    }
  }
}

void Parser::SetLanguage(const Nan::FunctionCallbackInfo<Value> &info) {
  Parser *parser = ObjectWrap::Unwrap<Parser>(info.This());

  const TSLanguage *language = language_methods::UnwrapLanguage(info[0]);
  if (language) {
    ts_parser_set_language(parser->parser_, language);
    info.GetReturnValue().Set(info.This());
  }
}

void Parser::Parse(const Nan::FunctionCallbackInfo<Value> &info) {
  Parser *parser = ObjectWrap::Unwrap<Parser>(info.This());

  if (!info[0]->IsFunction()) {
    Nan::ThrowTypeError("Input must be a function");
    return;
  }

  Local<Function> callback = Local<Function>::Cast(info[0]);

  Local<Object> js_old_tree;
  const TSTree *old_tree = nullptr;
  if (info.Length() > 1 && !info[1]->IsNull() && !info[1]->IsUndefined() && Nan::To<Object>(info[1]).ToLocal(&js_old_tree)) {
    const Tree *tree = Tree::UnwrapTree(js_old_tree);
    if (!tree) {
      Nan::ThrowTypeError("Second argument must be a tree");
      return;
    }
    old_tree = tree->tree_;
  }

  Local<Value> buffer_size = Nan::Null();
  if (info.Length() > 2) buffer_size = info[2];

  if (!handle_included_ranges(parser->parser_, info[3])) return;

  CallbackInput callback_input(callback, buffer_size);
  TSTree *tree = ts_parser_parse(parser->parser_, old_tree, callback_input.Input());
  Local<Value> result = Tree::NewInstance(tree);
  info.GetReturnValue().Set(result);
}

void Parser::GetLogger(const Nan::FunctionCallbackInfo<Value> &info) {
  Parser *parser = ObjectWrap::Unwrap<Parser>(info.This());

  TSLogger current_logger = ts_parser_logger(parser->parser_);
  if (current_logger.payload && current_logger.log == Logger::Log) {
    Logger *logger = (Logger *)current_logger.payload;
    info.GetReturnValue().Set(Nan::New(logger->func));
  } else {
    info.GetReturnValue().Set(Nan::Null());
  }
}

void Parser::SetLogger(const Nan::FunctionCallbackInfo<Value> &info) {
  Parser *parser = ObjectWrap::Unwrap<Parser>(info.This());

  TSLogger current_logger = ts_parser_logger(parser->parser_);

  if (info[0]->IsFunction()) {
    if (current_logger.payload) delete (Logger *)current_logger.payload;
    ts_parser_set_logger(parser->parser_, Logger::Make(Local<Function>::Cast(info[0])));
  } else if (!Nan::To<bool>(info[0]).FromMaybe(true)) {
    if (current_logger.payload) delete (Logger *)current_logger.payload;
    ts_parser_set_logger(parser->parser_, { 0, 0 });
  } else {
    Nan::ThrowTypeError("Logger callback must either be a function or a falsy value");
    return;
  }

  info.GetReturnValue().Set(info.This());
}

void Parser::PrintDotGraphs(const Nan::FunctionCallbackInfo<Value> &info) {
  Parser *parser = ObjectWrap::Unwrap<Parser>(info.This());

  if (Nan::To<bool>(info[0]).FromMaybe(false)) {
    ts_parser_print_dot_graphs(parser->parser_, 2);
  } else {
    ts_parser_print_dot_graphs(parser->parser_, -1);
  }

  info.GetReturnValue().Set(info.This());
}

}  // namespace node_tree_sitter
