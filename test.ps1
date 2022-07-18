Remove-Item -Confirm:$false -Recurse 'coverage'
cargo test
llvm-profdata merge -sparse coverage/leaprs-*.profraw -o coverage/leaprs.profdata
llvm-cov export target\debug\deps\leaprs-7372d08029cc2e95.exe -instr-profile=".\coverage\leaprs.profdata" --ignore-filename-regex='\\.cargo\\registry' --Xdemangler=rustfilt -format=lcov | Out-File -Encoding UTF8 -FilePath .\coverage\lcov.info