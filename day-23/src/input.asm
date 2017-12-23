                set b 57
                set c b
                jnz a 2         # to not_debug
                jnz 1 5         # to debug
not_debug:      mul b 100
                sub b -100000
                set c b
                sub c -17000
debug:          set f 1
                set d 2
loop_b:         set e 2
loop_a:         set g d
                mul g e
                sub g b
                jnz g 2         # to skip_reset_f
                set f 0
skip_reset_f:   sub e -1
                set g e
                sub g b
                jnz g -8        # to loop_a
                sub d -1
                set g d
                sub g b
                jnz g -13       # to loop_b
                jnz f 2         # to skip_inc_h
                sub h -1
skip_inc_h:     set g b
                sub g c
                jnz g 2         # to continue
                jnz 1 3         # to END
continue:       sub b -17
                jnz 1 -23       # to debug
END:
