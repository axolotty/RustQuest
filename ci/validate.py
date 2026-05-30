#!/usr/bin/env python3
"""Valide que toutes les solutions de référence passent leur test.

Interroge le serveur RustQuest (supposé lancé sur 127.0.0.1:3000) : pour chaque
niveau, récupère sa solution officielle, la soumet à /api/run, et vérifie que
le résultat est `success`. Sort en erreur (code 1) si au moins un niveau échoue.
"""

import json
import sys
import urllib.request

BASE = "http://127.0.0.1:3000"


def get(path):
    with urllib.request.urlopen(BASE + path) as r:
        return json.load(r)


def post(path, data):
    req = urllib.request.Request(
        BASE + path,
        data=json.dumps(data).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req) as r:
        return json.load(r)


def main():
    total = get("/api/levels?lang=fr")["total_count"]
    print(f"Validation de {total} niveaux...\n")

    failures = []
    for i in range(1, total + 1):
        detail = get(f"/api/levels/{i}?lang=fr")
        outcome = post("/api/run", {"level_id": i, "code": detail["solution"]})["outcome"]
        if outcome["success"]:
            print(f"  OK   {i:3d}  {detail['slug']}")
        else:
            print(f"  FAIL {i:3d}  {detail['slug']}  (stage={outcome['stage']})")
            failures.append((i, detail["slug"], outcome))

    if failures:
        print(f"\nÉCHEC : {len(failures)} niveau(x) ne passent pas.")
        for i, slug, o in failures:
            print(f"\n--- niveau {i} ({slug}) — stage={o['stage']} ---")
            if o.get("compile_output"):
                print(o["compile_output"][:600])
            if o.get("stdout") is not None:
                print("stdout attendu :", repr(o.get("expected")))
                print("stdout obtenu  :", repr(o.get("stdout")))
            if o.get("stderr"):
                print("stderr :", o["stderr"][:300])
        sys.exit(1)

    print(f"\nOK : les {total} niveaux passent leur test. ✅")


if __name__ == "__main__":
    main()
