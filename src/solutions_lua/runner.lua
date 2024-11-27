function(day, solution, input)
    local filedir = "src/solutions_lua/"
    local fileend = ".fnl"
    local padded_day = day
    if day < 10 then
        padded_day = "0" .. day
    end
    filename = filedir .. "d" .. padded_day .. "s" .. solution .. fileend;
    
    _G.input = input

    return require("fennel").install().dofile(filename)
end
