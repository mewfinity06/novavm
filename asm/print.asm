; Write to standard out from DATA_SECTION_START+0..DATA_SECTION_START+20
SYSCALL WRITE $1 $0 $20
; Write to standard out from DATA_SECTION_START+22..DATA_SECTION_START+59
SYSCALL WRITE $1 $22 $59

[[DATA]] "This is a print line!\n"
[[DATA]] "Let's try some escaped \"characters\"\n"
