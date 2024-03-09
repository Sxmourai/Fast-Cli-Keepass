import os
print("Creating new release")
targets = "x86_64-unknown-linux-gnu x86_64-pc-windows-gnu".split(" ")

for target in targets:
    os.system(f"rustup target add {target}")
    print(f"Building target {target}")
    os.system(f"cargo b --release --target {target}")
    print(f"Creating archive...")
    extension = ".exe" if "windows" in target else "" 
    path = f"target/{target}/release/fast-cli-keepass{extension}"
    os.system(f"tar czf target/{target}.tar.gz {path}")
    print(f"Done ! Get your archive at: target/{target}.tar.gz")

print(f"Creating source code archive")
os.system("tar czf target/source_code.tar.gz --directory=src .")
print(f"Done ! Get your archive at: target/source_code.tar.gz")
