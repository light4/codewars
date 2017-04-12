def validBraces(string):
    data = ['{}', '()', '[]']
    if not any(braces in string for braces in data):
        return False
    for braces in data:
        if braces in string:
            break

    if string == braces:
        return True
    else:
        return validBraces(string.replace(braces, ''))
