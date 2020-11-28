#!/bin/bash
set -ex

jsonnet_version=0.17.0
# XXX: Use thread-safe adn fmt-supported version for now: https://github.com/google/go-jsonnet/commit/35acb29ff856a4bb3e992bb74cac53e0accc656a
gojsonnet_version=35acb29ff856a4bb3e992bb74cac53e0accc656a

rm -rf vendor/go-jsonnet
mkdir -p vendor/go-jsonnet
curl -sSfL https://github.com/google/go-jsonnet/archive/${gojsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet --gzip --strip-components=1 --file=-
(cd vendor/go-jsonnet && go mod vendor)

curl -sSfL https://github.com/google/jsonnet/archive/v${jsonnet_version}.tar.gz | tar -x --directory=vendor/go-jsonnet/cpp-jsonnet --gzip --strip-components=1 --file=- jsonnet-${jsonnet_version}/include/libjsonnet.h jsonnet-${jsonnet_version}/include/libjsonnet_fmt.h
