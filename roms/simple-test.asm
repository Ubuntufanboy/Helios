; Helios Demo Program
; This program draws a pattern on the screen and plays some sounds

.org $0000           ; Start at address 0

start:
    LDA #$00         ; Clear the X and Y registers
    TAX
    TAY
    
    ; Initialize pattern variables
    LDA #$01         ; Color 1 (Red)
    STA $80          ; Store at zero page address $80
    LDA #$06         ; Delay counter for animation
    STA $81

main_loop:
    JSR draw_pattern ; Draw the pattern
    JSR play_sound   ; Play a sound
    JSR delay        ; Wait a bit
    JMP main_loop    ; Loop forever

; Draw a colorful pattern
draw_pattern:
    LDA #$00         ; Start at the top of the screen
    STA $82          ; Y position
    
y_loop:
    LDA #$00         ; Start at the left of the screen
    STA $83          ; X position
    
x_loop:
    ; Calculate color based on X, Y, and time
    LDA $83          ; X
    AND #$07         ; Mod 8
    ADC $82          ; Add Y
    AND #$07         ; Mod 8
    ADC $80          ; Add time-based color
    AND #$07         ; Ensure color is 0-7
    
    ; Calculate memory address for pixel
    LDA $82          ; Y position
    STA $84          ; Temp storage
    LDA #$00
    STA $85
    
    ; Multiply Y by 256 to get row offset
    LDA $84
    STA $85          ; High byte = Y (since we're multiplying by 256)
    
    ; Add X to get final position
    LDA $83          ; X position
    ADC #$00         ; Add low byte of row offset
    STA $84          ; Low byte of address
    
    ; Set the pixel
    LDA $80          ; Use the current color
    EOR $84          ; XOR with X position
    EOR $85          ; XOR with Y position
    AND #$07         ; Ensure in range 0-7
    
    ; Write to display memory (Start at $F000)
    LDX $85          ; High byte
    LDY $84          ; Low byte
    STA $F000, Y     ; Write to display memory
    
    ; Increment X position
    LDA $83
    ADC #$04         ; Skip every 4 pixels for a grid pattern
    STA $83
    CMP #$FF         ; Check if we've reached the end of the row
    BNE x_loop
    
    ; Increment Y position
    LDA $82
    ADC #$04         ; Skip every 4 rows for a grid pattern
    STA $82
    CMP #$FF         ; Check if we've reached the end of the screen
    BNE y_loop
    
    ; Update the pattern color for next frame
    LDA $80
    ADC #$01
    AND #$07         ; Keep in range 0-7
    STA $80
    
    RTS

; Play a sound on a random channel
play_sound:
    ; Choose a channel (0-3)
    LDA $80
    AND #$03
    STA $86          ; Channel number
    
    ; Choose a note (0-63)
    LDA $80
    ADC $82
    AND #$3F         ; Notes 0-63
    STA $87          ; Note value
    
    ; Combine channel and note
    LDA $86          ; Channel (0-3)
    ;ASL A            ; Shift left 6 bits
    ;ASL A
    ;ASL A
    ;ASL A
    ;ASL A
    ;ASL A
    ORA $87          ; OR with note value
    
    ; Play the sound
    SND #$00         ; Send to audio system
    
    RTS

; Delay routine
delay:
    LDX #$00         ; Initialize counter
delay_loop:
    LDY #$00         ; Inner loop counter
inner_delay:
    INY              ; Increment Y
    CPY #$00         ; Check if Y wrapped to 0
    BNE inner_delay  ; If not, continue inner loop
    
    INX              ; Increment X
    CPX $81          ; Compare with delay value
    BNE delay_loop   ; If not equal, continue outer loop
    
    RTS

.org $FFFC          ; Reset vector location
.word start         ; Reset vector
.word start         ; IRQ vector
