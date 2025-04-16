import json

common_letters = ["e", "t", "a", "o", "i", "n", "s", "h", "r", "d", "l", "u"]
rare_letters = ["q", "x", "z", "j", "k"]

def isVowel(char):
    if char == "a" or char == "e" or char == "i" or char == "o" or char == "u":
        return True
    else:
        return False


def score_word(word):
    score = 100

    common_bonus = 0
    rare_penalty = 0
    vowel_count = 0
    consonant_count = 0
    for char in word:
        if char in common_letters:
            common_bonus += 2

        if char in rare_letters:
            rare_penalty += 2

        if isVowel(char):
            vowel_count += 1
        else:
            consonant_count += 1

    score -= rare_penalty

    # Penalize a imbalance of consonants and vowels
    ratio = vowel_count / (vowel_count + consonant_count)
    if ratio < 0.3 or ratio > 0.7:
        score -= 15

    # Penalize if vowels and consonants dont alternate & penalize for repeating letters
    alternation_penalty = 0
    repeat_penalty = 0
    for i in range(len(word) - 2):
        if (
            isVowel(word[i])
            and isVowel(word[i + 1])
            or (not isVowel(word[i]) and not isVowel(word[i + 1]))
        ):
            alternation_penalty += 1

        if word[i] == word[i + 1]:
            repeat_penalty += 1

    score -= alternation_penalty * 3
    score -= repeat_penalty * 5

    return score


def get_frequency(frequency):
    if frequency == "very frequent":
        return 9
    elif frequency == "frequent":
        return 8
    elif frequency == "common":
        return 7
    elif frequency == "lesser":
        return 6
    elif frequency == "uncommon":
        return 5
    elif frequency == "very rare":
        return 4
    elif frequency == "inscription":
        return 3
    elif frequency == "graffiti":
        return 2
    elif frequency == "Pliny (only in Pliny Natural History)":
        return 1
    else:
        return 0

with open("latin.json", "r") as file:
    data = json.load(file)


with open("latin.txt", "w") as output_file:
    for word in data:
        frequency = get_frequency(word["info"]["freq"])
    
        for part in word["parts"]:
            if ' ' in part or '-' in part or '|' in part or 'zzz' in part or '/' in part or '(' in part or ')' in part:
                continue

            score = score_word(part)

            output_file.write(f"{part},{frequency},{score}\n")
    
