
# Small script which was used to extract all the maps from the maps_minified.json and put them into their own file

import json

maps = json.loads(open("./maps/maps_minified.json", "r", encoding="utf8").read())

i = 0
for m in maps:
    print("MAP:", m['name'], "idx:", i)
    # open(f"./maps/jsons/{m['name']}.json",'w').write(json.dumps(m, indent=4))
    i+=1

# _ current maps
# MAP: Burg idx: 0
# MAP: Littletown idx: 1
# MAP: Sandstorm idx: 2
# MAP: Subzero idx: 3
# MAP: Undergrowth idx: 4
# MAP: Shipyard idx: 5
# MAP: Freight idx: 6
# MAP: Lostworld idx: 7
# MAP: Citadel idx: 8
# MAP: Oasis idx: 9
# MAP: Kanji idx: 10
# MAP: Industry idx: 11
# MAP: Lumber idx: 12
# MAP: Evacuation idx: 13
# MAP: Site idx: 14
# MAP: SkyTemple idx: 15
# MAP: Lagoon idx: 16
# MAP: Bureau idx: 17
# MAP: Tortuga idx: 18