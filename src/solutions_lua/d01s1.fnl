(let [input _G.input]
  (fn char_as_int [c]
    (case c
      "1" 1
      "2" 2
      "3" 3
      "4" 4
      "5" 5
      "6" 6
      "7" 7
      "8" 8
      "9" 9
      _ 0))

  (fn split_at [str i]
    (values (string.sub str 1 i) (string.sub str (+ i 1) -1)))

  (fn get_first_digit [str]
    (let [(lft rgt) (split_at str 1)]
      (local x (char_as_int lft))
      (case x
        0 (get_first_digit rgt)
        _ x)))

  (fn get_line_value [str]
    (let [lft (get_first_digit str)
          rgt (get_first_digit (string.reverse str))]
      (+ (* lft 10) rgt)))

  (var line_values [])
  (each [_ line (pairs input)]
    (table.insert line_values (get_line_value line)))
  (accumulate [sum 0 i n (ipairs line_values)] (+ sum n)))
