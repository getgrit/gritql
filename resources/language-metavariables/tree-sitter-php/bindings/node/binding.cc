#include "tree_sitter/parser.h"
#include <node.h>
#include "nan.h"

using namespace v8;

extern "C" TSLanguage * tree_sitter_php();
extern "C" TSLanguage * tree_sitter_php_only();

namespace {

NAN_METHOD(New) {}

void Init(Local<Object> exports, Local<Object> module) {
  Local<FunctionTemplate> php_tpl = Nan::New<FunctionTemplate>(New);
  php_tpl->SetClassName(Nan::New("Language").ToLocalChecked());
  php_tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<Function> php_constructor = Nan::GetFunction(php_tpl).ToLocalChecked();
  Local<Object> php_instance = php_constructor->NewInstance(Nan::GetCurrentContext()).ToLocalChecked();
  Nan::SetInternalFieldPointer(php_instance, 0, tree_sitter_php());
  Nan::Set(php_instance, Nan::New("name").ToLocalChecked(), Nan::New("php").ToLocalChecked());

  Local<FunctionTemplate> php_only_tpl = Nan::New<FunctionTemplate>(New);
  php_only_tpl->SetClassName(Nan::New("Language").ToLocalChecked());
  php_only_tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<Function> php_only_constructor = Nan::GetFunction(php_only_tpl).ToLocalChecked();
  Local<Object> php_only_instance = php_only_constructor->NewInstance(Nan::GetCurrentContext()).ToLocalChecked();
  Nan::SetInternalFieldPointer(php_only_instance, 0, tree_sitter_php_only());
  Nan::Set(php_only_instance, Nan::New("name").ToLocalChecked(), Nan::New("php_only").ToLocalChecked());

  Nan::Set(exports, Nan::New("php").ToLocalChecked(), php_instance);
  Nan::Set(exports, Nan::New("php_only").ToLocalChecked(), php_only_instance);
}

NODE_MODULE(tree_sitter_php_binding, Init)

}  // namespace
