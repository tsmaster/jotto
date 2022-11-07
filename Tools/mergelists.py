import glob

dirs = [#"../Data/*.txt",
        #"../Data/MobyWords/files/*.TXT"
        #"../Data/MobyWords/files/COMMON.TXT",
        #"../Data/MobyWords/files/CROSSWD.TXT",
        #"../Data/MobyWords/files/CRSWD-D.TXT",
        #"../Data/MobyWords/files/FREQ.TXT",
    "../Data/words_alpha.txt",
        ]

def make_anag(w):
    letters = [w[x] for x in range(len(w))]
    letters.sort()
    return ''.join(letters)

def has_dups(w):
    for idx_1 in range(len(w)):
        c1 = w[idx_1]
        for idx_2 in range(idx_1 + 1, len(w)):
            c2 = w[idx_2]
            if c1 == c2:
                return True
    return False

worddict = {}


for d in dirs:
    for g in glob.glob(d):
        print(g)
        with open(g) as fileobj:
            try:
                for line in fileobj.readlines():
                    for w in line.split():
                        w = w.strip().upper()
                        if not w.isalpha():
                            continue
                        if len(w) == 5:
                            if has_dups(w):
                                continue
                            w_an = make_anag(w)
                            #print("word:", w)
                            if not (w_an in worddict):
                                print("new word:", w)
                                old_list = worddict.get(w_an, [])
                                new_list = old_list + [w]
                                new_list.sort()
                                worddict[w_an] = new_list
            except:
                print("error")

word_key_list = list(worddict.keys())

words = []

for wk in word_key_list:
    words.append(worddict[wk][0])

words.sort()



with open("outwords.txt", "wt") as outfile:
    for w in words:
        outfile.write(w)
        outfile.write('\n')
        
                              
