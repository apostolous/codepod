import argparse

def compile(ver):
  print("compiling " + ver())

def admin():
  return "adm"

def client():
  return "cli"

def server():
  return "serv"

def main():
  print("CodePods Compiler Service")

  parser = argparse.ArgumentParser(prog="CodePods")

  parser.add_argument("-a", "--admin", action="store_true", required=False)
  parser.add_argument("-c", "--client", action="store_true", required=False)
  parser.add_argument("-s", "--server", action="store_true", required=False)

  args = parser.parse_args();

  if args.admin:
    compile(admin)
  
  if args.client:
    compile(client)

  if args.server:
    compile(server)


if __name__ == "__main__":
  main()