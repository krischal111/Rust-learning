from curses.ascii import SO
from youtube_dl import main


class Solution:
    def isValid(self, s: str) -> bool:
        
        brackets = {'}':'{', ')':'(', ']':'['}
        bracket_stack = []
        for char in s :
            # return True
            if s in brackets.values() :
                bracket_stack.append(s)
                print(bracket_stack)
            if s in brackets :
                print(bracket_stack)
                if brackets.len() == 0 :
                    return False
                if brackets[char] !=  bracket_stack.pop() :
                    return False
        if  len(bracket_stack) :
            return False
        else :
            return False
            
__name__: main
s = Solution()
print(s.isValid("((())){}[](]"))

