import re
import sys

defines = dict()

def preProcessFile(path):
    file = open(path, "r")
    lines = file.readlines()

    reading = True
    while reading:
        # Remove comments
        for i,line in enumerate(lines):
            lines[i] = re.sub(";;.*$", "", line)

        # Remove trash
        for i,line in enumerate(lines):
            line = line.replace("\t", " ")
            line = line.replace("\n", "")
            line = re.sub(" +$", "", line)
            lines[i] = re.sub(" +", " ", line)

        reading = False
        for i,line in enumerate(lines):
            if re.search("^#inc ", line):
                incPath = line.split(" ")[1]
                incFile = open(incPath, "r")
                incLines = incFile.readlines()
                lines[i] = ""
                lines[i:i] = incLines;
                incFile.close()
                reading = True
                break;
    for i,line in enumerate(lines):
        if re.search("^#var ", line):
            name = line.split(" ")[1]
            value = line.split(" ")[2]
            defines[name] = value
            lines[i] = ""

    code = [['','NOI.noa']]
    for i,line in enumerate(lines):
        if line != "":
            split = line.split(" ");
            if len(split) <= 3:
                code.append(line.split(" "))
            else:
                print("Error at %d: %s" % (i, line))
                print(split)
                exit(1)
    return code

def getPositions(code):
    positions = dict()
    for i,step in enumerate(code):
        if step[0] != "":
            positions[step[0].replace(":", "")] = i;
    return positions

def assemble(code):
    for i,step in enumerate(code):
        if re.match("^0x[0-9A-Fa-f][0-9A-Fa-f]", step[1]):
            value = int(step[1], base = 16)
            value = "{0:08b}".format(value)
            code[i][1] = value;
        if re.match("^[0-9]", step[1]):
            value = int(step[1])
            value = "{0:08b}".format(value)
            code[i][1] = value;
        else:
            match step[1].split(".")[0]:
                case "NOI":
                    insByte = 0b00000000
                case "JUM":
                    insByte = 0b00001000
                case "LDD":
                    insByte = 0b01000000
                case "STD":
                    insByte = 0b01001000
                case "LDA":
                    insByte = 0b01010000
                case "STA":
                    insByte = 0b01011000
                case "ADD":
                    insByte = 0b10000000
                case "SUB":
                    insByte = 0b10001000
                case "NAN":
                    insByte = 0b10010000
                case "SHL":
                    insByte = 0b10100000
                case "SHR":
                    insByte = 0b10101000
                case "EQU":
                    insByte = 0b10110000
                case "GRE":
                    insByte = 0b10111000
                #case _:
                #    exit(2)
            match step[1].split(".")[1]:
                case "noa":
                    insByte |= 0b000
                case "num":
                    insByte |= 0b001
                case "ram":
                    insByte |= 0b010
                case "rom":
                    insByte |= 0b011
                case "ptr":
                    insByte |= 0b100
                case "prr":
                    insByte |= 0b101
                case "inp":
                    insByte |= 0b110
                case "out":
                    insByte |= 0b111
            code[i][1] = "{0:08b}".format(insByte);

        if len(step) == 3:
            if re.match("^0x[0-9A-Fa-f]{2}$", step[2]):
                value = int(step[2], base = 16)
                value = "{0:08b}".format(value)
                code[i][2] = value;
            elif re.match("^0x[0-9A-Fa-f]{4}$", step[2]):
                value = int(step[2], base = 16)
                value = "{0:016b}".format(value)
                code[i][2] = value[:len(value)//2]
                code[i].append(value[len(value)//2:])
            elif re.match("^[0-9]+$", step[2]):
                value = int(step[2])
                if value > 255:
                    sys.exit("Decimal number too large (>255).")
                value = "{0:08b}".format(value)
                code[i][2] = value
            elif re.match("^:[A-Za-z][A-Za-z0-9_\-]*$", step[2]):
                code[i].append("")
            elif re.match("^:[A-Za-z][A-Za-z0-9_\-]*@[01]$", step[2]):
                pass
            elif re.match("^[A-Za-z][A-Za-z0-9_]+$", step[2]):
                value = int(defines[step[2]], base = 16)
                value = "{0:08b}".format(value)
                code[i][2] = value;
            else:
                sys.exit(f"Unrecognized format for value {code[i]}")
        code[i] = step[1:]
    return code

def countTo(pos, code):
    val = 0
    for i in range(0, pos):
        val += len(code[i])
    return val

def positionReplace(code, positions):
    for i,line in enumerate(code):
        if len(line) == 3 and re.match("^:[A-Za-z][A-Za-z0-9\-_]*$", line[1]):
            print(line[1])
            ref = positions[line[1].replace(":", "")]
            ref = countTo(ref, code)
            ref = "{0:016b}".format(ref)
            code[i][1] = ref[:len(ref)//2]
            code[i][2] = ref[len(ref)//2:]
        elif len(line) == 2 and re.match("^:[A-Za-z][A-Za-z0-9\-_]*(@[01])?$", line[1]):
            ref = positions[line[1].replace(":", "").split("@")[0]]
            ref = countTo(ref, code)
            ref = "{0:016b}".format(ref)
            if line[1].split("@")[1] == "1":
                code[i][1] = ref[:len(ref)//2]
            else:
                code[i][1] = ref[len(ref)//2:]

    return code

def codeToBin(code, path):
    asm = []
    for line in code:
        for subline in line:
            asm.append(int(subline, base = 2))
    restsize = 65536 - len(asm);
    for i in range(0, restsize):
        asm.append(0)
    asmBytes = bytearray(asm)
    binfile = open(path, "wb")
    binfile.write(asmBytes)
    binfile.close()

code = preProcessFile(sys.argv[1])
print(code)
positions = getPositions(code)
code = assemble(code)
code = positionReplace(code, positions)
codeToBin(code, "result.bin")
