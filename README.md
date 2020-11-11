## Sort Markdown Translations

English is not my mother tongue, and I often find myself annotating the new words I encounter on Bear, the note-taking app I’m using.
I don’t spend time sorting the lines properly as I write them down, so the whole file turns out to be an almost useless mess.
On top of that, Bear is not great at synchronizing notes between phone and laptop, so I have to write two separate sections.
Long story short, that’s what I get:

```md
### Mobile
Plait - treccia (per i capelli)
Uncanny - sorprendente
Grimace - smorfia / fare una smorfia (to grimace)
Snoop - ficcanaso
To crease - piegare/piega
Alcove - nicchia
Frenzy - frenesia
Foolhardy - temerario

### Laptop
To ditch - abbandonare, scavare un fosso
Shed - capannone, capanno
To shed - liberarsi, spargere (to shed some light on <topic>)
Bog - palude
To scorch - bruciare

---
To redeem - redimere (liberare da una condizione di asservimento/negativa), salvare
Mollify - placare, ammansire

---

### Sorted
Alas - ahimè
Attainment - raggiungimento, conseguimento, conquista

Baffled - sconcertato
Barf - vomitare
Beam - raggio

Charter - carta, statuto

Endangered - in pericolo, minacciato
Enraged - infuriato
```

This script sorts the lines and groups them by their first letter (excluding the particle `to` for verbs), making it easier for me to look up (and hopefully learn some of) the words I previously encountered:

```md
### Mobile


### Laptop


---

### A
alas - ahimè
alcove - nicchia
attainment - raggiungimento, conseguimento, conquista

### B
baffled - sconcertato
barf - vomitare
beam - raggio
bog - palude

### C
charter - carta, statuto
to crease - piegare/piega

### D
to ditch - abbandonare, scavare un fosso

### E
endangered - in pericolo, minacciato
enraged - infuriato

### F
foolhardy - temerario
frenzy - frenesia

### G
grimace - smorfia / fare una smorfia (to grimace)

### M
mollify - placare, ammansire

### P
plait - treccia (per i capelli)

### R
to redeem - redimere (liberare da una condizione di asservimento/negativa), salvare

### S
to scorch - bruciare
shed - capannone, capanno
to shed - liberarsi, spargere (to shed some light on <topic>)
snoop - ficcanaso

### U
uncanny - sorprendente
```

Usage:

```sh
sort_md_translations <input_file> <output_file>
```

I didn’t bother giving the script a professional-looking: no fancy libraries to parse the cli arguments and it’s happy to crash with an unfriendly error message when I don’t use it properly.
It does its job, and that’s all I need for now.
