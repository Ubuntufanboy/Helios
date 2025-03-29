start:
    LDA #$00 ; clearing registers
    TAX ; Transfer the 0 in A to X
    TAY ; Transfer the 0 in A to Y

    LDA #$01 ; Load 1 into A
    STX $80 ; Store value in 80
    DBG $80 ; Debug 80
