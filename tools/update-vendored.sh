#!/bin/bash
set -ex

jsonnet_version=0.17.0

rm -rf vendor/go-jsonnet
mkdir -p vendor/go-jsonnet
curl -sSfL https://github.com/google/go-jsonnet/archive/v${jsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet --gzip --strip-components=1 --file=-
(cd vendor/go-jsonnet && go mod vendor)

# Need google/jsonnet/include/libjsonnet.h until https://github.com/google/go-jsonnet/pull/482
curl -sSfL https://github.com/google/jsonnet/archive/v${jsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet/cpp-jsonnet --gzip --strip-components=1 --file=- jsonnet-${jsonnet_version}/include/libjsonnet.h
