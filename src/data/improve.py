seen = set()
unique_lines = []

with open("latin.txt", "r") as src_file:
    lines = src_file.read().splitlines()

for line in lines:
    if not line:
        continue
    word = line.split(",")[0]
    if word not in seen:
        seen.add(word)
        unique_lines.append(line)

unique_lines.sort(
    key=lambda x: (
        len(x.split(",")[0]),           # word length
        int(x.split(",")[1]),           # frequency
        int(x.split(",")[2])            # score
    ),
    reverse=True
)

with open("latin.txt", "w") as output_file:
    output_file.write("\n".join(unique_lines))

