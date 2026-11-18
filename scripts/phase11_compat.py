"""Compare two compiled CLIs on the recorded foundation corpus plus deterministic semantic cases."""
from pathlib import Path
import json
import subprocess
import sys
import tempfile

root=Path(__file__).resolve().parents[1]
base,new=map(lambda s:Path(s).resolve(),sys.argv[1:])
# Explicit foundation inventory; new-only examples have their own oracle.
names=("hello","hello_mn","scalar","scalar_mn","functions","functions_mn","matmul","matmul_mn",
       "mixed_syntax","negative/shape_mismatch","negative/shape_mismatch_mn")
cases=[(root/"examples"/(name+".mn")).read_bytes() for name in names]
cases.extend([
    b"fn main(){let a=a;}",b"fn main(){return;print(1);}",b"fn main(){print(1/0);}",
    b"fn main(){let b:bool=1;}",b"fn bad(){print(missing);}fn main(){}",b"fn main(){print(f());}fn f()->i64{return 9;}",
    b"fn main(){main();}",b"fn main(){print(1);print(9223372036854775807+1);}",b"fn main(){\xff}",b"fn main(){print((1);}",
    "функц main(){хувьсагч café=3; хэвлэх(cafe\u0301);}".encode()
])
for i in range(20):
    cases.append(f"функц main(){{let a=[[{i}.0,1.0]];хэвлэх(a@[[2.0],[1.0]]);}}".encode())
with tempfile.TemporaryDirectory() as d:
    p=Path(d)/"case.mn";comparisons=0
    for case in cases:
        p.write_bytes(case)
        for cmd in ("check","run"):
            for lang in ("en","mn"):
                def observe(binary):
                    r=subprocess.run([binary,cmd,p,"--lang",lang],capture_output=True,timeout=10)
                    return (r.returncode,r.stdout,r.stderr)
                if observe(base)!=observe(new):raise SystemExit(f"Mismatch: {case!r} {cmd} {lang}")
                comparisons+=1
print(json.dumps(dict(case_count=len(cases),paired_comparisons=comparisons,attributes=["exit","stdout","stderr"])))
