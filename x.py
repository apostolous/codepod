import argparse
import tomllib
import subprocess
from typing import TypedDict

class CompileTable(TypedDict):
  path: str
  command: str

def compile(ct: CompileTable):
  command = ct["command"]
  path = ct["path"]
  subprocess.call(command, cwd=path, shell=True)

def load_toml():
  with open("compile.toml", "rb") as f:
    return tomllib.load(f)

def main():
  print("CodePods Compiler Service")

  print("Loading compile.toml...")
  compile_data = load_toml()

  for attr in compile_data.items():
    compile(attr[1])


if __name__ == "__main__":
  main()