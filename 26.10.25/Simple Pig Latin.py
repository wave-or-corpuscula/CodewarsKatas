"""
Move the first letter of each word to the end of it, then add "ay" to the end of the word. 
Leave punctuation marks untouched.
"""

def words_wierdinator(text: str):
    wierd_text = []
    for word in text.split(" "):
        if word.isalpha():
            wierd_text.append(word[1:] + word[0] + "ay")
        else:
            wierd_text.append(word)
    return " ".join(wierd_text)


if __name__ == "__main__":
    text = "somem text for processing yess !"
    result = words_wierdinator(text)
    print(result)

    
