(module
    (import "js" "log" (func $log (param i32)))
    (import "js" "log" (func $log_i64 (param i64)))
    (import "js" "file" (memory $file 1))

    ;; Working area.
    (memory $scratch 1)

    ;; Read a byte out of the file.
    (func $read_byte (param $offset i32) (result i32)
        ;; Load from the offset
        (i32.load (local.get $offset))

        ;; Extract the byte.
        (i32.and (i32.const 0xff)))

    ;; Determine the total length of the file.
    (func $file_length (result i32)
        (local $i i32)
        (loop $search
            (call $read_byte (local.get $i))
            (i32.ne (i32.const 0))
            (if (then
                (local.set $i (i32.add (local.get $i) (i32.const 1)))
                (br $search))))
        (local.get $i))

    ;; Find the first offset of `target`, starting from `start`. Will return the index, or `-1` if it's not found.
    (func $find_offset (param $start i32) (param $target i32) (result i32)
        ;; Index.
        (local $i i32)
        ;; Return value.
        (local $ret i32)
        ;; Current byte.
        (local $byte i32)

        ;; Initialise index to start offset.
        (local.set $i (local.get $start))
        ;; Initialise return value to `-1` to indicate failure.
        (local.set $ret (i32.const -1))

        (block $break
            (loop $search
                ;; Read the current byte.
                (local.set $byte
                    (call $read_byte (local.get $i)))

                ;; If reading `0`, reached end of file.
                (i32.eq (local.get $byte) (i32.const 0))
                (br_if $break)

                (if
                  ;; Check if the target is found.
                  (i32.eq (local.get $byte) (local.get $target))
                  (then
                    ;; Update the return value.
                    (local.set $ret (local.get $i)))
                  (else
                    ;; Increment the index, and continue searching.
                    (local.set $i (i32.add (local.get $i) (i32.const 1)))
                    (br $search)))

            ))

        ;; Add the return value to the stack.
        (local.get $ret))

    ;; Calculate the height and width of the grid.
    (func $calculate_bounds (param $scratch_offset i32)
    )

    ;; Convert the provided byte into a digit.
    (func $byte_to_digit (param $byte i32) (result i32)
        (i32.sub
            (local.get $byte)
            (i32.const 0x30)))

    ;; If the provided byte is a digit, return it. Otherwise return -1.
    (func $try_byte_to_digit (param $byte i32) (result i32)
        ;; Greater or equal to b'0'
        (i32.ge_u (local.get $byte) (i32.const 0x30))
        ;; Greater or equal to b'9'
        (i32.le_u (local.get $byte) (i32.const 0x39))

        (if (result i32) (i32.and)
            (then
              (call $byte_to_digit (local.get $byte)))
            (else
                (i32.const -1))))

    ;; Log the provided number, and return it.
    (func $dbg (param $num i32) (result i32)
        (call $log (local.get $num))
        (local.get $num))
    (func $dbg_i64 (param $num i64) (result i64)
        (call $log_i64 (local.get $num))
        (local.get $num))

    ;; Parse a number left to right.
    (func $parse_left (param $start i32) (result i32)
        (local $result i32)
        (local $i i32)
        (local $current_digit i32)

        (local.set $result (i32.const 0))
        (local.set $i (local.get $start))

        (block $done
            (loop $step
                ;; Fetch the current byte.
                (call $read_byte (local.get $i))

                ;; Attempt to convert it to a digit.
                (local.tee $current_digit (call $try_byte_to_digit))

                ;; If not a valid byte, must be finished.
                (br_if $done (i32.eq (i32.const -1)))

                ;; Update the current result.
                (local.set $result
                    (i32.add
                        (i32.mul (local.get $result) (i32.const 10))
                        (local.get $current_digit)))

                ;; Increment the counter and continue looping.
                (local.set $i (i32.add (local.get $i) (i32.const 1)))
                (br $step)))

        (local.get $result))

    ;; Starting at the provided offset, advance until non-whitespace is encountered, and return the new offset. If the end-of-line is encountered, `-1` will be returned.
    (func $skip_whitespace (param $offset i32) (result i32)
        (local $i i32)
        (local $byte i32)
        
        (local.set $i (local.get $offset))
        
        (block $done
            (loop $step
                ;; Read the current byte.
                (local.set $byte (call $read_byte (local.get $i)))

                ;; Check if it's b'\n'.
                (i32.eq (local.get $byte) (i32.const 0x0a))
                (if (then
                    ;; End of line reached, indicate with -1.
                    (local.set $i (i32.const -1))
                    (br $done)))

                ;; Check if it's b' '.
                (i32.eq (local.get $byte) (i32.const 0x20))
                (if (then
                    ;; Increment counter.
                    (local.set $i (i32.add (local.get $i) (i32.const 1)))
                    (br $step)))))

        (local.get $i))

    ;; Part 1 solution.
    (func $part1 (result i64)
        (local $answer i64)

        (local $line_length i32)
        (local $line_count i32)
        (local $signs_offset i32)
        (local $row_count i32)

        (local $row_i i32)
        (local $column_i i32)
        (local $column_sign i32)
        (local $column_total i64)
        (local $current_num i64)

        (local.set $answer (i64.const 0))

        ;; Find the line length (add 1 to account for new line).
        (local.set $line_length
            (i32.add
                (call $find_offset (i32.const 0) (i32.const 0x0a))
                (i32.const 1)))

        ;; Calculate the number of lines in the file.
        (local.set $line_count
            (i32.div_u
                (call $file_length)
                (local.get $line_length)))

        (local.set $row_count
            (i32.sub
                (local.get $line_count)
                (i32.const 1)))

        ;; Calculate offset to signs.
        (local.set $signs_offset
            (i32.mul
                (local.get $line_length)
                (local.get $row_count)))

        (local.set $column_i (i32.const 0))
        (loop $column_loop
            ;; Calculate the index of the sign.
            (i32.add
                (local.get $signs_offset)
                (local.get $column_i))

            ;; Extract the sign
            (local.set $column_sign (call $read_byte))

            ;; Reset the column count.
            (local.set $column_total
                (if 
                    (result i64)
                    (i32.eq
                        (local.get $column_sign)
                        (i32.const 0x2a)) ;; b'*'
                    (then (i64.const 1))
                    (else (i64.const 0))))

            ;; Reset row counter
            (local.set $row_i (i32.const 0))
            (loop $row_loop
                ;; Calculate starting offset.
                (i32.add
                    (i32.mul
                        (local.get $line_length)
                        (local.get $row_i))
                    (local.get $column_i))

                ;; Skip any starting whitespace
                (call $skip_whitespace)

                ;; Parse number.
                (local.set $current_num
                    (i64.extend_i32_u (call $parse_left)))

                (local.set $column_total
                    (if (result i64)
                        (i32.eq
                            (local.get $column_sign)
                            (i32.const 0x2a)) ;; b'*'
                        (then (i64.mul
                            (local.get $column_total)
                            (local.get $current_num)))
                        (else (i64.add
                            (local.get $column_total)
                            (local.get $current_num)))
                    ))

                (local.tee $row_i
                    (i32.add
                        (local.get $row_i)
                        (i32.const 1)))
                (i32.lt_u (local.get $row_count))
                (br_if $row_loop))

            ;; Update the answer.
            (local.set $answer
                (i64.add
                    (local.get $answer)
                    (local.get $column_total)))

            ;; Calculate the offset pointing to after the sign.
            (i32.add
                (local.get $column_i)
                (local.get $signs_offset))
            (i32.add (i32.const 1))

            ;; Advance until next non-whitespace.
            (call $skip_whitespace)

            ;; Save the new offset (includes offset to signs).
            (local.tee $column_i)

            (if (i32.ne (i32.const -1))
                (then
                    ;; Update to remove offset to signs.
                    (local.set $column_i
                        (i32.sub
                            (local.get $column_i)
                            (local.get $signs_offset)))
                    (br $column_loop))))

        (local.get $answer))

    (export "part1" (func $part1)))
