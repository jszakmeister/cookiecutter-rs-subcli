import subprocess

subprocess.run(["git", "init"], check=True)
subprocess.run(["git", "add", "."], check=True)
