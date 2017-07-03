def mix(s1, s2):
    map1 = {}
    map2 = {}
    result = []
    for c in s1:
        if "a" <= c <= "z":
            map1[c] = map1.get(c, 0) + 1
    for c in s2:
        if "a" <= c <= "z":
            map2[c] = map2.get(c, 0) + 1
    for c in set(list(map1.keys()) + list(map2.keys())):
        count1 = map1.get(c, 0)
        count2 = map2.get(c, 0)
        if max(count1, count2) <= 1:
            continue
        if count1 > count2:
            result.append({"c": c, "length": count1, "order": 1})
        elif count1 == count2:
            result.append({"c": c, "length": count1, "order": 3})
        else:
            result.append({"c": c, "length": count2, "order": 2})

    result = sorted(result, key=lambda x: (-x["length"], x["order"], x["c"]))
    output = []
    for i in result:
        if i["order"] == 3:
            output.append("=:{}".format(i["length"] * i["c"]))
        else:
            output.append("{}:{}".format(i["order"], i["length"] * i["c"]))

    return "/".join(output)
