pub fn load_rom () -> Vec<u8> {
    vec![
        // LD SP,$fffe		; $0000  Setup Stack
        0x31, 0xfe, 0xff, 
        // OR A			; $0003  Zero the memory from $8000-$9FFF (VRAM)
        0xaf, 
        // LD HL,$9fff		; $0004
        0x21, 0xff, 0x9f,
    // Addr_0007:
	    // LD (HL-),A		; $0007
        0x32,
        // BIT 7,H		; $0008
        0xcb, 0x7c, 
        // JR NZ, Addr_0007	; $000a
        0x20, 0xfb, 
        // LD HL,$ff26		; $000c  Setup Audio
        0x21, 0x26, 0xff, 
        // LD C,$11		; $000f
        0x0e, 0x11,
        // LD A,$80		; $0011
        0x3e, 0x80, 
        // LD (HL-),A		; $0013
        0x32, 
        // LD ($FF00+C),A	; $0014
        0xe2, 
        // INC C			; $0015
        0x0c, 
        // LD A,$f3		; $0016
        0x3e, 0xf3, 
        // LD ($FF00+C),A	; $0018
        0xe2, 
        // LD (HL-),A		; $0019
        0x32, 
        // LD A,$77		; $001a
        0x3e, 0x77, 
        // LD (HL),A		; $001c
        0x77, 
        // LD A,$fc		; $001d  Setup BG palette
        0x3e, 0xfc, 
        // LD ($FF00+$47),A	; $001f
        0xe0, 0x47,
        // LD DE,$0104		; $0021  Convert and load logo data from cart into Video RAM 
        0x11, 0x04, 0x01, 
        // LD HL,$8010		; $0024
        0x21, 0x10, 0x80, 
    // Addr_0027:
        // LD A,(DE)		; $0027
        0x1a, 
        // CALL $0095		; $0028
        0xcd, 0x95, 0x00, 
        // CALL $0096		; $002b
        0xcd, 0x96, 0x00, 
        // INC DE		; $002e
        0x13, 
        // LD A,E		; $002f
        0x7b, 
        // CP $34		; $0030
        0xfe, 0x34, 
        // JR NZ, Addr_0027	; $0032
        0x20, 0xf3,
        // LD DE,$00d8		; $0034  Load 8 additional bytes into Video RAM
        0x11, 0xd8, 0x00, 
        // LD B,$08		; $0037
        0x06, 0x08,
    // Addr_0039:   
        // LD A,(DE)		; $0039 
        0x1a, 
        // INC DE		; $003a
        0x13, 
        // LD (HL+),A		; $003b
        0x22, 
        // INC HL		; $003c
        0x23, 
        // DEC B			; $003d
        0x05, 
        // JR NZ, Addr_0039	; $003e
        0x20, 0xf9, 
        // LD A,$19		; $0040  Setup background tilemap
        0x3e, 0x19, 
        // LD ($9910),A	; $0042
        0xea, 0x10, 0x99, 
        // LD HL,$992f		; $0045
        0x21, 0x2f, 0x99,
    // Addr_0048:
        // LD C,$0c		; $0048 
        0x0e, 0x0c, 
    // Addr_004A: 
        // DEC A			; $004a   
        0x3d, 
        // JR Z, Addr_0055	; $004b
        0x28, 0x08, 
        // LD (HL-),A		; $004d
        0x32, 
        // DEC C			; $004e
        0x0d, 
        // JR NZ, Addr_004A	; $004f
        0x20, 0xf9, 
        // LD L,$0f		; $0051
        0x2e, 0x0f, 
        // JR Addr_0048	; $0053
        0x18, 0xf3, 

        // ; === Scroll logo on screen, and play logo sound===
    // Addr_0055:
        // LD H,A		; $0055  Initialize scroll count, H=0
        0x67, 
        // LD A,$64		; $0056
        0x3e, 0x64, 
        // LD D,A		; $0058  set loop count, D=$64
        0x57,
        // LD ($FF00+$42),A	; $0059  Set vertical scroll register 
        0xe0, 0x42,
        // LD A,$91		; $005b
        0x3e, 0x91, 
        // LD ($FF00+$40),A	; $005d  Turn on LCD, showing Background
        0xe0, 0x40,
        // INC B			; $005f  Set B=1
        0x04, 
    // Addr_0060:
        // LD E,$02		; $0060
        0x1e, 0x02, 
    // Addr_0062: 
        // LD C,$0c		; $0062   
        0x0e, 0x0c, 
    // Addr_0064:
        // LD A,($FF00+$44)	; $0064  wait for screen frame
        0xf0, 0x44,
        // CP $90		; $0066
        0xfe, 0x90, 
        // JR NZ, Addr_0064	; $0068
        0x20, 0xfa, 
        // DEC C			; $006a
        0x0d,
        // JR NZ, Addr_0064	; $006b 
        0x20, 0xf7,
        // DEC E			; $006d
        0x1d,
        // JR NZ, Addr_0062	; $006e 
        0x20, 0xf2, 
        // LD C,$13		; $0070
        0x0e, 0x13,
        // INC H			; $0072  increment scroll count
        0x24, 
        // LD A,H		; $0073
        0x7c, 
        // LD E,$83		; $0074
        0x1e, 0x83, 
        // CP $62		; $0076  $62 counts in, play sound #1
        0xfe, 0x62, 
        // JR Z, Addr_0080	; $0078
        0x28, 0x06, 
        // LD E,$c1		; $007a
        0x1e, 0xc1,
        // CP $64		; $007c
        0xfe, 0x64,
        // JR NZ, Addr_0086	; $007e  $64 counts in, play sound #2
        0x20, 0x06, 
    // Addr_0080:
        // LD A,E		; $0080  play sound
        0x7b,
        // LD ($FF00+C),A	; $0081
        0xe2, 
        // INC C	
        0x0c, 
        // LD A,$87		; $0083
        0x3e, 0x87, 
        // LD ($FF00+C),A	; $0085
        0xe2, 
    // Addr_0086:
        // LD A,($FF00+$42)	; $0086
        0xf0, 0x42,
        // SUB B			; $0088 
        0x90, 
        // LD ($FF00+$42),A	; $0089  scroll logo up if B=1
        0xe0, 0x42, 
        // DEC D			; $008b
        0x15, 
        // JR NZ, Addr_0060	; $008c
        0x20, 0xd2,
        // DEC B			; $008e  set B=0 first time 
        0x05,
        // JR NZ, Addr_00E0	; $008f    ... next time, cause jump to "Nintendo Logo check"
        0x20, 0x4f,
        // LD D,$20		; $0091  use scrolling loop to pause
        0x16, 0x20, 
        // JR Addr_0060	; $0093
        0x18, 0xcb, 

        // ; ==== Graphic routine ====
        
        // LD C,A		; $0095  "Double up" all the bits of the graphics data
        0x4f,
        // LD B,$04		; $0096     and store in Video RAM
        0x06, 0x04, 
    // Addr_0098:
        // PUSH BC		; $0098
        0xc5, 
        // RL C			; $0099
        0xcb, 0x11, 
        // RLA			; $009b
        0x17, 
        // POP BC		; $009c
        0xc1, 
        // RL C			; $009d
        0xcb, 0x11, 
        // RLA			; $009f
        0x17, 
        // DEC B			; $00a0
        0x05, 
        // JR NZ, Addr_0098	; $00a1
        0x20, 0xf5, 
        // LD (HL+),A		; $00a3
        0x22,
        // INC HL		; $00a4
        0x23, 
        // LD (HL+),A		; $00a5
        0x22, 
        // INC HL		; $00a6
        0x23, 
    // RET			; $00a7
        0xc9, 
    // Addr_00A8:
        // Nintendo logo
        0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 
        0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 
        0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 
        0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 
        0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 
        0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
    // Addr_00D8:
        // Tile data for ® symbol
        0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
        
        // ; ===== Nintendo logo comparison routine =====

    // Addr_00E0:
        // LD HL,$0104		; $00e0	; point HL to Nintendo logo in cart
        0x21, 0x04, 0x01, 
        // LD DE,$00a8		; $00e3	; point DE to Nintendo logo in DMG rom
        0x11, 0xa8, 0x00, 
    // Addr_00E6:
        // LD A,(DE)		; $00e6
        0x1a, 
        // INC DE		; $00e7
        0x13,
        // CP (HL)		; $00e8 ;compare logo data in cart to DMG rom
        0xbe, 
        // JR NZ,$fe		; $00e9	;if not a match, lock up here
        0x20, 0xfe, 
        // INC HL		; $00eb
        0x23, 
        // LD A,L		; $00ec
        0x7d, 
        // CP $34		; $00ed	;do this for $30 bytes
        0xfe, 0x34, 
        // JR NZ, Addr_00E6	; $00ef
        0x20, 0xf5, 
        // LD B,$19		; $00f1
        0x06, 0x19, 
        // LD A,B		; $00f3
        0x78, 
    // Addr_00F4:
        // ADD (HL)		; $00f4
        0x86, 
        // INC HL		; $00f5
        0x23, 
        // DEC B			; $00f6
        0x05, 
        // JR NZ, Addr_00F4	; $00f7
        0x20, 0xfb, 
        // ADD (HL)		; $00f9
        0x86, 
        // JR NZ,$fe		; $00fa	; if $19 + bytes from $0134-$014D  don't add to $00
		//				;  ... lock up
        0x20, 0xfe, 
        // LD A,$01		; $00fc
        0x3e, 0x01, 
        // LD ($FF00+$50),A	; $00fe	;turn off DMG rom
        0xe0, 0x50, 
    ]
}