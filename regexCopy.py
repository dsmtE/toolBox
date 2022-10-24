import argparse
from pathlib import Path
import re
import shutil
import os

"""
example:
 python .\regexCopy.py fromPath toPath "(?P<name>.*)/(?P=name).metadata.json" -rp "\g<name>/Spine/\g<name>.metadata.json"
"""
if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("srcFolderPath", help="Source folder path", type=Path)
    ap.add_argument("destFolderPath", help="Destination folder path", type=Path)
    ap.add_argument("srcRegexFileMatch", help="Regex for matching files in provided srcfolder", type=str)
    ap.add_argument(
        "-rp",
        "--replacementPattern",
        help="Replacement pattern to be used on the relative path from srcFolder before copying the file relatively to destFolder",
        type=str,
        default=None)
    ap.add_argument("-o", '--overwrite', default=False, action='store_true')
    args = ap.parse_args()

    srcFolderPath = Path(args.srcFolderPath).resolve()
    destFolderPath = Path(args.destFolderPath).resolve()
    srcRegexFileMatch = args.srcRegexFileMatch
    replacementPattern = args.replacementPattern
    overwrite = args.overwrite

    regex = re.compile(srcRegexFileMatch)

    # TODO add TQDM progress bar
    for path in srcFolderPath.rglob("*"):
        if not path.is_file():
            continue

        srcRelativepPath = path.relative_to(srcFolderPath)
        srcRelativepPathStr = srcRelativepPath.as_posix()

        match = regex.match(srcRelativepPathStr)

        if match:
            destRelativePath = re.sub(srcRegexFileMatch, replacementPattern, srcRelativepPathStr) if replacementPattern is not None else srcRelativepPathStr
            destinationPath = destFolderPath / destRelativePath
            if os.path.isdir(destinationPath):
                raise RuntimeError(f"Dir Path are not handled currently \"{destinationPath}\"")
            print(f"Copy \"{path}\" to \"{destinationPath}\"")
            if destinationPath.exists() and overwrite:
                if destFolderPath.is_file():
                    os.remove(destinationPath)
                else:
                    raise ValueError(f"Unable to delete properly destination path \"{destFolderPath}\".")
            shutil.copy(path, destinationPath)

    # TODO Add validation(Yes, No, full preview), export full preview if needed in txt file