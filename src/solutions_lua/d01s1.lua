function(input)
    function is_numeral(test_char)
        numerals = {"1", "2", "3", "4", "5", "6", "7", "8", "9"}
        for i = 1,9 do
            if test_char == numerals[i] then
                return i
            end
        end
        return -1
    end

    function get_first_numeral(str)
        for i = 1, string.len(str) do
            test_char = string.sub(str, i, i)
            test_num = is_numeral(test_char)
            if test_num ~= -1 then
                return test_num
            end
        end
        return 0
    end

    accum = 0
    for input_line_num,line_str in pairs(input) do
        print()
        print(line_str)
        first_num = get_first_numeral(line_str)
        last_num = get_first_numeral(string.reverse(line_str))
        new_num = (first_num * 10) + last_num
        print(new_num)
        accum = accum + new_num
    end
    return accum
end
