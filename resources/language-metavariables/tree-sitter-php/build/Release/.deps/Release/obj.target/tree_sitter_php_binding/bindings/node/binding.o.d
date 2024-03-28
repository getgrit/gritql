cmd_Release/obj.target/tree_sitter_php_binding/bindings/node/binding.o := c++ -o Release/obj.target/tree_sitter_php_binding/bindings/node/binding.o ../bindings/node/binding.cc '-DNODE_GYP_MODULE_NAME=tree_sitter_php_binding' '-DUSING_UV_SHARED=1' '-DUSING_V8_SHARED=1' '-DV8_DEPRECATION_WARNINGS=1' '-DV8_DEPRECATION_WARNINGS' '-DV8_IMMINENT_DEPRECATION_WARNINGS' '-D_GLIBCXX_USE_CXX11_ABI=1' '-D_DARWIN_USE_64_BIT_INODE=1' '-D_LARGEFILE_SOURCE' '-D_FILE_OFFSET_BITS=64' '-DOPENSSL_NO_PINSHARED' '-DOPENSSL_THREADS' '-DBUILDING_NODE_EXTENSION' -I/Users/james/Library/Caches/node-gyp/18.12.1/include/node -I/Users/james/Library/Caches/node-gyp/18.12.1/src -I/Users/james/Library/Caches/node-gyp/18.12.1/deps/openssl/config -I/Users/james/Library/Caches/node-gyp/18.12.1/deps/openssl/openssl/include -I/Users/james/Library/Caches/node-gyp/18.12.1/deps/uv/include -I/Users/james/Library/Caches/node-gyp/18.12.1/deps/zlib -I/Users/james/Library/Caches/node-gyp/18.12.1/deps/v8/include -I../../nan -I../php/src  -O3 -gdwarf-2 -mmacosx-version-min=10.15 -arch x86_64 -Wall -Wendif-labels -W -Wno-unused-parameter -std=gnu++17 -stdlib=libc++ -fno-rtti -fno-exceptions -fno-strict-aliasing -MMD -MF ./Release/.deps/Release/obj.target/tree_sitter_php_binding/bindings/node/binding.o.d.raw   -c
Release/obj.target/tree_sitter_php_binding/bindings/node/binding.o: \
  ../bindings/node/binding.cc ../php/src/tree_sitter/parser.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/node.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/cppgc/common.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8config.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-array-buffer.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-local-handle.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-internal.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-version.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-object.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-maybe.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-persistent-handle.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-weak-callback-info.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-primitive.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-data.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-value.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-traced-handle.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-container.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-context.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-snapshot.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-date.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-debug.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-script.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-message.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-exception.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-extension.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-external.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-function.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-function-callback.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-template.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-memory-span.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-initialization.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-callbacks.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-isolate.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-embedder-heap.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-microtask.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-statistics.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-promise.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-unwinder.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-embedder-state-scope.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-platform.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-json.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-locker.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-microtask-queue.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-primitive-object.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-proxy.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-regexp.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-typed-array.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-value-serializer.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-wasm.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_version.h \
  ../../nan/nan.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/errno.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/version.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/unix.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/threadpool.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/darwin.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_buffer.h \
  /Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_object_wrap.h \
  ../../nan/nan_callbacks.h ../../nan/nan_callbacks_12_inl.h \
  ../../nan/nan_maybe_43_inl.h ../../nan/nan_converters.h \
  ../../nan/nan_converters_43_inl.h ../../nan/nan_new.h \
  ../../nan/nan_implementation_12_inl.h \
  ../../nan/nan_persistent_12_inl.h ../../nan/nan_weak.h \
  ../../nan/nan_object_wrap.h ../../nan/nan_private.h \
  ../../nan/nan_typedarray_contents.h ../../nan/nan_json.h \
  ../../nan/nan_scriptorigin.h
../bindings/node/binding.cc:
../php/src/tree_sitter/parser.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/node.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/cppgc/common.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8config.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-array-buffer.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-local-handle.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-internal.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-version.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-object.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-maybe.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-persistent-handle.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-weak-callback-info.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-primitive.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-data.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-value.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-traced-handle.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-container.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-context.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-snapshot.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-date.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-debug.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-script.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-message.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-exception.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-extension.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-external.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-function.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-function-callback.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-template.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-memory-span.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-initialization.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-callbacks.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-isolate.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-embedder-heap.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-microtask.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-statistics.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-promise.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-unwinder.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-embedder-state-scope.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-platform.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-json.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-locker.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-microtask-queue.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-primitive-object.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-proxy.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-regexp.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-typed-array.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-value-serializer.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/v8-wasm.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_version.h:
../../nan/nan.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/errno.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/version.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/unix.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/threadpool.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/uv/darwin.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_buffer.h:
/Users/james/Library/Caches/node-gyp/18.12.1/include/node/node_object_wrap.h:
../../nan/nan_callbacks.h:
../../nan/nan_callbacks_12_inl.h:
../../nan/nan_maybe_43_inl.h:
../../nan/nan_converters.h:
../../nan/nan_converters_43_inl.h:
../../nan/nan_new.h:
../../nan/nan_implementation_12_inl.h:
../../nan/nan_persistent_12_inl.h:
../../nan/nan_weak.h:
../../nan/nan_object_wrap.h:
../../nan/nan_private.h:
../../nan/nan_typedarray_contents.h:
../../nan/nan_json.h:
../../nan/nan_scriptorigin.h:
