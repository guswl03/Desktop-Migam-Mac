"""Build and validate the app-ready Gamjabot pack from the approved pose sheet."""
from __future__ import annotations
import json
from collections import deque
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/"images/characters/gamjabot/source/production-pose-sheet.png"
PACK=ROOT/"images/characters/gamjabot/pack"
SIZE=(256,256)
BLACK=(0,0,0,255)
WHITE=(255,255,255,255)

def primary(image):
    alpha=image.getchannel("A"); px=alpha.load(); seen=set(); groups=[]
    for y in range(image.height):
        for x in range(image.width):
            if not px[x,y] or (x,y) in seen: continue
            queue=deque([(x,y)]); seen.add((x,y)); group=[]
            while queue:
                cx,cy=queue.popleft(); group.append((cx,cy))
                for nx,ny in ((cx-1,cy),(cx+1,cy),(cx,cy-1),(cx,cy+1)):
                    if 0<=nx<image.width and 0<=ny<image.height and px[nx,ny] and (nx,ny) not in seen:
                        seen.add((nx,ny)); queue.append((nx,ny))
            groups.append(group)
    if not groups: raise ValueError("empty pose cell")
    mask=Image.new("L",image.size,0); mp=mask.load()
    for x,y in max(groups,key=len): mp[x,y]=255
    result=Image.new("RGBA",image.size,(0,0,0,0)); result.paste(image,mask=mask)
    return result

def clean(image):
    source=image.convert("RGBA"); result=Image.new("RGBA",source.size,(0,0,0,0))
    src,dst=source.load(),result.load()
    for y in range(source.height):
        for x in range(source.width):
            r,g,b,_=src[x,y]
            magenta=r>150 and b>100 and g<180 and r>g+50 and b>g+30
            if not magenta: dst[x,y]=WHITE if r*299+g*587+b*114>=128000 else BLACK
    return primary(result)

def register(pose,airborne=False):
    cropped=pose.crop(pose.getbbox()); mw,mh=(220,204) if airborne else (218,218)
    scale=min(mw/cropped.width,mh/cropped.height)
    cropped=cropped.resize((round(cropped.width*scale),round(cropped.height*scale)),Image.Resampling.NEAREST)
    result=Image.new("RGBA",SIZE,(0,0,0,0)); x=(256-cropped.width)//2
    y=(256-cropped.height)//2 if airborne else 238-cropped.height
    result.alpha_composite(cropped,(x,y)); return result

def move(frame,dx=0,dy=0,angle=0):
    result=Image.new("RGBA",SIZE,(0,0,0,0))
    result.alpha_composite(frame.rotate(angle,resample=Image.Resampling.NEAREST),(dx,dy))
    return result

def save(name,frames):
    folder=PACK/name; folder.mkdir(parents=True,exist_ok=True); paths=[]
    for index,frame in enumerate(frames):
        path=folder/f"{index}.png"; frame.save(path,optimize=True); paths.append(f"{name}/{index}.png")
    return paths

def anim(frames,ms,loop,impact=None):
    value={"frames":frames,"frameMs":ms,"loop":loop}
    if impact is not None: value["impactFrame"]=impact
    return value

def validate(manifest):
    failures=[]; colours=set()
    for state,spec in manifest["animations"].items():
        for relative in spec["frames"]:
            with Image.open(PACK/relative) as frame:
                if frame.size!=SIZE or frame.mode!="RGBA": failures.append(f"{relative}: expected 256x256 RGBA")
                for r,g,b,a in frame.get_flattened_data():
                    if a:
                        colours.add((r,g,b))
                        if (r,g,b) not in {(0,0,0),(255,255,255)}:
                            failures.append(f"{relative}: non-monochrome pixel"); break
        if not 16<=spec["frameMs"]<=5000: failures.append(f"{state}: invalid frameMs")
        if "impactFrame" in spec and spec["impactFrame"]>=len(spec["frames"]): failures.append(f"{state}: invalid impactFrame")
    return {"ok":not failures,"canvas":list(SIZE),"visibleColours":[list(c) for c in sorted(colours)],
            "eyeNoiseGuard":"visible pixels restricted to pure black and pure white","failures":failures}

def main():
    sheet=Image.open(SOURCE).convert("RGBA"); poses=[]
    for row in range(2):
        for col in range(5):
            box=(round(col*sheet.width/5),round(row*sheet.height/2),round((col+1)*sheet.width/5),round((row+1)*sheet.height/2))
            poses.append(register(clean(sheet.crop(box)),row==0 and col in {2,3}))
    idle,walk,dragged,thrown,kick,speak,chase,dance,card,click=poses
    animations={
      "idle":anim(save("idle",[move(idle,dy=y) for y in (0,-2,0,1)]),420,True),
      "walk":anim(save("walk",[move(walk,x,y,a) for x,y,a in ((-2,0,-1),(0,-3,1),(2,0,-1),(0,-2,1))]),130,True),
      "dragged":anim(save("dragged",[move(dragged,dy=-3,angle=-2),move(dragged,dy=2,angle=2)]),160,True),
      "thrown":anim(save("thrown",[move(thrown,dy=y,angle=a) for y,a in ((4,-8),(-5,5),(0,18),(7,30))]),90,True),
      "kick":anim(save("kick",[idle,move(kick,dx=-2),move(kick,dx=5),idle]),90,False,2),
      "speak":anim(save("speak",[move(speak,dy=y) for y in (0,-2,0,1)]),180,True),
      "chase":anim(save("chase",[move(chase,x,y,a) for x,y,a in ((-2,1,-2),(1,-2,1),(3,0,-1),(0,-2,2))]),115,True),
      "dance":anim(save("dance",[move(dance,dy=-2,angle=-4),move(dance,dy=1,angle=4),move(dance,dy=-3,angle=-2),move(dance,angle=3)]),140,True),
      "carry-card":anim(save("carry-card",[card,move(card,dy=-2)]),260,True),
      "click-react":anim(save("click-react",[click,move(click,dy=-5),click]),110,False)}
    manifest={"schemaVersion":1,"id":"gamjabot","displayName":"Gamjabot","canvas":{"width":256,"height":256},
      "anchors":{"feet":{"x":128,"y":238},"speech":{"x":128,"y":18},"accessory":{"x":128,"y":92},"kickImpact":{"x":232,"y":196}},
      "hitbox":{"x":34,"y":22,"width":188,"height":218},"animations":animations,"accessories":[],
      "artRules":{"palette":["#000000","#FFFFFF"],"eyeNoise":"No marks other than one solid pupil per eye",
                  "accessoryStrategy":"Separate transparent 256x256 overlay layers"}}
    PACK.mkdir(parents=True,exist_ok=True)
    (PACK/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n",encoding="utf-8")
    report=validate(manifest); (PACK/"validation.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    if not report["ok"]: raise SystemExit("; ".join(report["failures"]))
    print(json.dumps({"states":len(animations),**report}))

if __name__=="__main__": main()
