

Script to generate day src files:
`for ((i=1; i <= 25; ++i)); do echo "use regex::Regex;\n\npub fn day${i}_1() {\n    let s = std::fs::read_to_string(\"src/day${i}.txt\").unwrap();\n}\n\npub fn day${i}_2() 
{\n\n}" > src/day${i}.rs; done`


Script for txt files:
`for ((i=1; i <= 25; ++i)); do touch src/day${i}.txt; done`
