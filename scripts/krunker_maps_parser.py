import time
import re
import json

# Written by dasbardgoet: 2022.06.11
# How to use:
# - Download krunker source code and prettyfy it (this url should download it, if the server is still running)
# - Save the prettified source code in [./data/krunker_source_code_prettified.js]
# - Run the script
# - On error, good luck fixing it
# - Visit [./src/gamestate/maps.rs] and add/update the missing maps there

# getting maps data out of prettified source code

map_regex = r"""e\.exports = JSON\.parse\('({"name":".+)'\);"""
start_time = time.time()

with open("./data/krunker_source_code_prettified.js", "r", encoding="utf-8") as f:
    source_code = f.read()

map_re_compiled = re.compile(map_regex)

x = map_re_compiled.findall(source_code)
maps = [json.loads(_map) for _map in x]
print(f"Found maps:")
for x in maps:
    print(f"map name: {x['name']}")

with open("./maps/maps_minified.json", 'w+') as f:
    dumped_str = json.dumps(maps, indent=4)
    f.write(dumped_str)
print("written to ./maps/maps_minified.json")

# writing data to jsons in maps folder
# Small script which was used to extract all the maps from the maps_minified.json and put them into their own file
# maps = json.loads(open("./maps/maps_minified.json", "r", encoding="utf8").read())

def get_idx(num, length):
    ss = str(num)
    ss_len = len(ss)
    if length > ss_len:
        ss = "0" * (length - ss_len) + ss
    return ss

i = 0
for m in maps:
    print("MAP:", m['name'], "idx:", i)
    open(f"./maps/jsons/{get_idx(i, 2)}_{m['name']}.json",'w+').write(json.dumps(m, indent=4))
    i+=1
print("Updated maps: ./maps/jsons/IDX_MAPNAME.json")

end_time = time.time()
print(f"Done in: {end_time-start_time} seconds")