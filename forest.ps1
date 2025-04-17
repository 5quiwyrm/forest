$file = Get-Item "./target/release/forest.exe"

if ($file -ne $null) {
    "./target/release/forest.exe $args" | iex
} else {
    "cargo run --release -- $args" | iex
}

