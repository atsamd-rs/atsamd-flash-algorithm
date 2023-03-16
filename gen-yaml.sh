#!/usr/bin/env -S sh -eu
#
# Provide a single argument with the path to the elf.
out_yaml=target/definition.yaml
elf=$1
sed -e 's/algorithm-test # \(.*\)$/\1/' template.yaml > "$out_yaml"
target-gen elf --update "$elf" "$out_yaml"
