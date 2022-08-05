--Registers func to be called after Clink's edit prompt ends (it is called after the onendedit event). The function receives a string argument containing the input text from the edit prompt. The function returns up to two values. If the first is not nil then it's a string that replaces the edit prompt text. If the second is not nil and is false then it stops further onfilterinput handlers from running.
--Starting in v1.3.13 func may return a table of strings, and each is executed as a command line.
--Note: Be very careful if you replace the text; this has the potential to interfere with or even ruin the user's ability to enter command lines for CMD to process.
clink.onfilterinput(function (line)
    --	clink.print("\x1b[35m>\x1b[m \x1b[32m"..line.."\x1b[m")

    --过滤特殊的cmd内置命令
    local commands = {"cd","clear","exit"}
    for i,v in ipairs(commands) do
        if v == string.sub(line,1, string.len(v)) then
            return line,true
        end
    end
    return "C:\\Users\\bigtear\\Documents\\GitHub\\reprint\\r.exe "..line,false
end)