# TRUFFLEYARD

# forensic automation tool using the SANS Windows Forensic Analysis Poster (for now)

## HOW TO USE
for now, this works only if the image is mounted in filesystem (read-only) using losetup
1. sudo losetup --find --partscan --show  --read-only /path/to/image.dd
2. sudo mount /mountpoint_losetup /target_mountpoint

tool uses the target_mountpoint as argument for now (will be changed eventually)


