# Copyright (c) 2022 David Chan
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

import json

output_data = {
    "sysroot_src": "/Users/davidchan/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library",
    "crates": []
}
for i in range(31):
    output_data['crates'].append({
        "root_module": f"2022/day-{i}/part-1.rs",
            "edition": "2021",
            "deps": []
            })
    output_data['crates'].append({
        "root_module": f"2022/day-{i}/part-2.rs",
            "edition": "2021",
            "deps": []
            })

with open('rust-project.json', 'w') as f:
    json.dump(output_data, f, indent=4)
