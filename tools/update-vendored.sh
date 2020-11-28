#!/bin/bash
set -ex

jsonnet_version=0.17.0
# XXX: Use thread-safe version for now: https://github.com/google/go-jsonnet/commit/57e6137c936549b3df96181d5741328f01946faf
gojsonnet_version=57e6137c936549b3df96181d5741328f01946faf

rm -rf vendor/go-jsonnet
mkdir -p vendor/go-jsonnet
curl -sSfL https://github.com/google/go-jsonnet/archive/${gojsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet --gzip --strip-components=1 --file=-
(cd vendor/go-jsonnet && go mod vendor)

curl -sSfL https://github.com/google/jsonnet/archive/v${jsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet/cpp-jsonnet --gzip --strip-components=1 --file=- jsonnet-${jsonnet_version}/include/libjsonnet.h
