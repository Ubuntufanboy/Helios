; SND Uses the bit structure CCNNNN where the first 2 bits represent channel and last 6 represent note
; Channel 0: Sine
; Channel 1: Square
; Channel 2: Triangle
; Channel 3: Noise
; Note - 0-63 + 21. 21-84

main: ; Main label isnt required but good for organization
    SND 32 ; MIDI note 32
    SND 33
    SND 34
    SND 35
    SND 36
    SND 37
    SND 38
    SND 39
