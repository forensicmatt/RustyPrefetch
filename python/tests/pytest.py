import sys
sys.path.append("../target/release")
import pyrpf

prefetch = sys.argv[1]
with open(prefetch,"rb") as pf_fh:
    print pyrpf.as_json(pf_fh)
