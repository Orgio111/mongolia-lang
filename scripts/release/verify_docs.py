"""Run the complete .mn programs printed inline in current user documentation."""
import argparse
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("binary",type=Path)
binary = parser.parse_args().binary.resolve()
examples = []
for doc in sorted(ROOT.rglob("*.md")):
    if "target" in doc.relative_to(ROOT).parts:
        continue
    programs = re.findall(r"^```(?:mn|mongolia)\s*\n(.*?)^```",doc.read_text(encoding="utf-8"),re.MULTILINE|re.DOTALL)
    for i,source in enumerate(programs):
        examples.append((doc.relative_to(ROOT).as_posix(),i,source))
llms = (ROOT/"llms.txt").read_text()
for i,source in enumerate(re.findall(r"^Valid (?:tensor )?source:\n(.*?)\n\n",llms,re.MULTILINE|re.DOTALL)):
    examples.append(("llms.txt",i,source))
expected = {("README.md",0):"[[19.0, 22.0], [43.0, 50.0]]\n",
            ("llms.txt",0):"42\n", ("llms.txt",1):"[[11.0]]\n"}
if {(doc,i) for doc,i,_ in examples} != set(expected):
    raise SystemExit("Update expected output for changed documentation examples")
with tempfile.TemporaryDirectory() as directory:
    path = Path(directory)/"documented.mn"
    for doc,i,source in examples:
        path.write_text(source,encoding="utf-8")
        for cmd,out in (("check",""),("run",expected[doc,i])):
            r = subprocess.run([binary,cmd,path],capture_output=True,text=True,encoding="utf-8",timeout=15)
            if (r.returncode,r.stdout,r.stderr) != (0,out,""):
                raise SystemExit(f"Documentation mismatch in {doc} sample {i+1}: {r}")
        print(f"PASS {doc} program {i+1}")
