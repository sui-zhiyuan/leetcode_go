import os
from pathlib import Path

dirs = os.listdir("./")
for dir in dirs:
    if not dir.startswith("p") and not dir.startswith("l"):
        continue
    if not os.path.isdir(dir):
        raise Exception("dir is not a directory", dir)

    if dir.startswith("p"):
        newDir = dir.replace("p", "l", 1)
        print("renaming", dir, "to", newDir)
        os.rename(dir, newDir)
        dir = newDir

    for subDir in os.listdir("./" + dir):
        if not subDir.startswith("p") and not subDir.startswith("l"):
            continue
        if not os.path.isdir(dir + "/" + subDir):
            raise Exception("subDir is not a directory", dir, subDir)

        oldSub = subDir
        newSub = subDir
        if oldSub.startswith("p"):
            newSub = oldSub.replace("p", "l", 1)
            print("renaming", dir + "/" + oldSub, "to", dir + "/" + newSub)
            os.rename(dir + "/" + oldSub, dir + "/" + newSub)
        elif oldSub.startswith("l"):
            oldSub = newSub.replace("l", "p", 1)

        for file in os.listdir("./" + dir + "/" + newSub):
            filePath = dir + "/" + newSub + "/" + file
            if not os.path.isfile(filePath):
                raise Exception ("path is not a file", filePath)
            text = Path(filePath).read_text()
            if text.startswith("package " + oldSub):
                text = text.replace("package " + oldSub, "package " + newSub, 1)
                print("changing package name in", filePath)
                Path(filePath).write_text(text)
            if text.find("github.com/sui-zhiyuan/leetcode_go/define") >= 0:
                text = text.replace(
                    "github.com/sui-zhiyuan/leetcode_go/define",
                    "github.com/sui-zhiyuan/leetcode_go/golang/common",
                    1,
                )
                text = text.replace("define.", "common.")
                print("changing define package path", filePath)
                Path(filePath).write_text(text)
