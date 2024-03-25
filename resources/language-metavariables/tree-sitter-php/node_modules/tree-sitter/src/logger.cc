#include "./logger.h"
#include <string>
#include <v8.h>
#include <nan.h>
#include <tree_sitter/api.h>
#include "./util.h"

namespace node_tree_sitter {

using namespace v8;
using std::string;

void Logger::Log(void *payload, TSLogType type, const char *message_str) {
  Logger *debugger = (Logger *)payload;
  Local<Function> fn = Nan::New(debugger->func);
  if (!fn->IsFunction())
    return;

  string message(message_str);
  string param_sep = " ";
  size_t param_sep_pos = message.find(param_sep, 0);

  Local<String> type_name = Nan::New((type == TSLogTypeParse) ? "parse" : "lex").ToLocalChecked();
  Local<String> name = Nan::New(message.substr(0, param_sep_pos)).ToLocalChecked();
  Local<Object> params = Nan::New<Object>();

  while (param_sep_pos != string::npos) {
    size_t key_pos = param_sep_pos + param_sep.size();
    size_t value_sep_pos = message.find(":", key_pos);

    if (value_sep_pos == string::npos)
      break;

    size_t val_pos = value_sep_pos + 1;
    param_sep = ", ";
    param_sep_pos = message.find(param_sep, value_sep_pos);

    string key = message.substr(key_pos, (value_sep_pos - key_pos));
    string value = message.substr(val_pos, (param_sep_pos - val_pos));
    Nan::Set(params, Nan::New(key).ToLocalChecked(), Nan::New(value).ToLocalChecked());
  }

  Local<Value> argv[3] = { name, params, type_name };
  TryCatch try_catch(Isolate::GetCurrent());
  Nan::Call(fn, GetGlobal(fn), 3, argv);
  if (try_catch.HasCaught()) {
    Local<Value> log_argv[2] = {
      Nan::New("Error in debug callback:").ToLocalChecked(),
      try_catch.Exception()
    };

    Local<Object> console = Local<Object>::Cast(Nan::Get(GetGlobal(fn), Nan::New("console").ToLocalChecked()).ToLocalChecked());
    Local<Function> error_fn = Local<Function>::Cast(Nan::Get(console, Nan::New("error").ToLocalChecked()).ToLocalChecked());
    Nan::Call(error_fn, console, 2, log_argv);
  }
}

TSLogger Logger::Make(Local<Function> func) {
  TSLogger result;
  Logger *logger = new Logger();
  logger->func.Reset(Nan::Persistent<Function>(func));
  result.payload = (void *)logger;
  result.log = Log;
  return result;
}

}  // namespace node_tree_sitter
