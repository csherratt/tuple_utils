

def to_type(off):
    return chr(ord('A') + off)

for i in range(0, 17):
    for j in range(0, 17):
        if i + j > 16:
            continue
        rhs = [to_type(x) for x in range(0, i)]
        lhs = [to_type(x + i) for x in range(0, j)]
        print "merge_impl!{[%s] [%s]}" % (", ".join(rhs), ", ".join(lhs))